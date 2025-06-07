// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ab_glyph::{FontVec, PxScale};
use anyhow::Result;
use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use std::{
    fs,
    io::{BufWriter, Cursor},
    sync::{Mutex, OnceLock},
    thread,
    time::{Duration, Instant},
};
mod handle;
mod i18n;
mod settings;
mod tray;

use i18n::I18n;
use settings::Settings;

use chrono::{Local, NaiveTime, Timelike};

use tauri::{
    menu::{Menu, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, WindowEvent,
};

use crate::{cmd::load_settings, handle::APP_HANDLE, tray::create_tray};

mod cmd;

fn update_language(language: i18n::Language) {
    // let i18n = I18n::new(language);

    // let system_tray_menu = SystemTrayMenu::new()
    //     .add_item(CustomMenuItem::new(
    //         "settings".to_string(),
    //         i18n.translate("settings", None),
    //     ))
    //     .add_item(CustomMenuItem::new(
    //         "quit".to_string(),
    //         i18n.translate("quit", None),
    //     ));

    // app_handle
    //     .tray_by_id("main")
    //     .unwrap()
    //     .set_menu(system_tray_menu)
    //     .unwrap();

    let app_handle = APP_HANDLE.get().unwrap();
    app_handle.emit("update-language", language).unwrap();

    if let Err(e) = create_tray() {
        eprintln!("Err when creating tray: {e:?}");
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .on_tray_icon_event(|app, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Down,
                ..
            } = event
            {
                let window = app.get_webview_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
        })
        // .plugin(tauri_plugin_shell::init())
        // .plugin(tauri_plugin_store::Builder::default().build())
        // .on_system_tray_event(|app, event| match event {
        //     SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
        //         "settings" => {
        //             let window = app.get_window("main").unwrap();
        //             window.show().unwrap();
        //             window.set_focus().unwrap();
        //             window.eval("window.location.href = '/settings';").unwrap();
        //         }
        //         "quit" => {
        //             std::process::exit(0);
        //         }
        //         _ => {}
        //     },
        //     _ => {}
        // })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .setup(move |app| {
            // init static values
            APP_HANDLE
                .set(app.handle().clone())
                .expect("AppHandle already set");

            let settings = load_settings(app.handle().clone());
            *Settings::get().write().unwrap() = settings;

            if let Err(e) = create_tray() {
                println!("{e:?}");
            }

            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                loop {
                    // let settings = app_handle.state::<Mutex<Settings>>();
                    // let settings = settings.lock().unwrap();
                    let settings = Settings::get();
                    println!("loop before read settings");
                    let settings_guard = settings.read().unwrap();
                    println!("loop: {settings:?}");
                    let settings = (*settings_guard).clone();
                    drop(settings_guard);

                    let now = Local::now();
                    let current_time =
                        NaiveTime::from_hms_opt(now.hour(), now.minute(), now.second()).unwrap();

                    let earned = if current_time < settings.start_time {
                        0.0
                    } else if current_time > settings.end_time {
                        settings.daily_salary
                    } else {
                        let work_duration = settings.end_time - settings.start_time;
                        let elapsed = current_time - settings.start_time;
                        settings.daily_salary
                            * (elapsed.num_seconds() as f64 / work_duration.num_seconds() as f64)
                    };

                    let earned_string = format!(" {}{:.2}", settings.currency_symbol, earned);

                    let start = Instant::now();
                    let _ = update_icon(&earned_string);
                    let elapsed = start.elapsed();
                    // println!("update icon: {elapsed:?}");

                    // app_handle
                    //     .tray_by_id("main")?
                    //     .set_title(Some(&format!(
                    //         "{}{earned_string}",
                    //         settings.currency_symbol
                    //     )))
                    //     .unwrap();

                    app_handle.emit("update-earnings", earned).unwrap();

                    thread::sleep(Duration::from_secs(1));
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cmd::save_settings,
            cmd::load_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

static FONT: OnceLock<FontVec> = OnceLock::new();

fn update_icon(earned: &str) -> Result<()> {
    let start = Instant::now();
    let icon = include_bytes!("../icons/icon.png");
    let img = image::load_from_memory(icon)?;
    let (width, height) = (img.width(), img.height());

    let font = FONT.get_or_init(|| {
        let font_bytes: Vec<u8> = include_bytes!("../assets/fonts/SFCompact.ttf").to_vec();
        FontVec::try_from_vec(font_bytes).unwrap()
    });

    let mut image = ImageBuffer::new((width as f32 * 5.0) as u32, height);
    image::imageops::replace(&mut image, &img, 0, 0);
    let text_color = Rgba([255u8, 255, 255, 255]);
    let base_size = height as f32;
    let scale = PxScale::from(base_size);
    let elapsed = start.elapsed();
    // println!("phase 1: {elapsed:?}");

    let start = Instant::now();
    draw_text_mut(&mut image, text_color, width as i32, 0, scale, font, earned);
    let elapsed = start.elapsed();
    // println!("phase 2: {elapsed:?}");

    let start = Instant::now();
    let mut bytes = Vec::new();
    let mut cursor = Cursor::new(&mut bytes);
    image.write_to(&mut cursor, image::ImageFormat::Png)?;
    let elapsed = start.elapsed();
    // println!("phase 3: {elapsed:?}");

    let start = Instant::now();
    let app_handle = APP_HANDLE.get().expect("AppHandle not initialized");
    let tray = app_handle.tray_by_id("main").unwrap();
    tray.set_icon(Some(tauri::image::Image::from_bytes(&bytes)?))?;
    let elapsed = start.elapsed();
    // println!("phase 4: {elapsed:?}");

    Ok(())
}

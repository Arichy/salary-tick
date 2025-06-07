// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time::Duration};
mod handle;
mod i18n;
mod settings;
mod tray;

use settings::Settings;

use chrono::{Local, NaiveTime, Timelike};

use crate::{
    cmd::load_settings,
    handle::APP_HANDLE,
    tray::{create_tray, update_icon},
};
use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconEvent},
    Emitter, Manager, WindowEvent,
};
use tracing::error;

mod cmd;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();

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
                error!("Err when creating tray: {e:?}");
            }

            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                loop {
                    let settings = Settings::get();
                    let settings_guard = settings.read().unwrap();
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

                    let _ = update_icon(&earned_string);

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

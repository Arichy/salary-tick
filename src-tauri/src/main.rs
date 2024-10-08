// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, sync::Mutex, thread, time::Duration};

mod i18n;
mod settings;

use i18n::I18n;
use settings::Settings;

use chrono::{Local, NaiveTime, Timelike};

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, WindowEvent};

fn get_settings_file_path(app_handle: &tauri::AppHandle) -> std::path::PathBuf {
    app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join("settings.json")
}

#[tauri::command]
fn load_settings(app_handle: tauri::AppHandle) -> Settings {
    let settings_file_path = get_settings_file_path(&app_handle);

    if settings_file_path.exists() {
        let settings_file = std::fs::read_to_string(settings_file_path).unwrap();
        let settings: Settings = serde_json::from_str(&settings_file).unwrap_or_default();
        settings
    } else {
        Settings::default()
    }
}

#[tauri::command]
fn save_settings(
    // state: tauri::State<Arc<Mutex<Settings>>>,
    app_handle: tauri::AppHandle,
    daily_salary: f64,
    start_time: String,
    end_time: String,
    currency_symbol: char,
    language: String,
) -> Result<(), String> {
    let start_time = NaiveTime::parse_from_str(&start_time, "%H:%M").unwrap();
    let end_time = NaiveTime::parse_from_str(&end_time, "%H:%M").unwrap();

    let settings = app_handle.state::<Mutex<Settings>>();
    let mut settings = settings.lock().unwrap();
    settings.daily_salary = daily_salary;
    settings.start_time = start_time;
    settings.end_time = end_time;
    settings.currency_symbol = currency_symbol;
    settings.language = match language.as_str() {
        "en" => i18n::Language::EN,
        "zh" => i18n::Language::ZH,
        _ => i18n::Language::EN,
    };

    update_language(settings.language, &app_handle);

    app_handle
        .emit_all("update-settings", settings.clone())
        .unwrap();

    let settings_json = serde_json::to_string(&*settings).map_err(|e| e.to_string())?;
    let settings_path = get_settings_file_path(&app_handle);

    // if the folder does not exist, create it
    if let Some(parent) = settings_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }

    fs::write(settings_path, settings_json).map_err(|e| e.to_string())?;

    Ok(())
}

fn update_language(language: i18n::Language, app_handle: &tauri::AppHandle) {
    let i18n = I18n::new(language);

    let system_tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new(
            "settings".to_string(),
            i18n.translate("settings", None),
        ))
        .add_item(CustomMenuItem::new(
            "quit".to_string(),
            i18n.translate("quit", None),
        ));

    app_handle.tray_handle().set_menu(system_tray_menu).unwrap();

    app_handle.emit_all("update-language", language).unwrap();
}

fn main() {
    let system_tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("settings".to_string(), "Settings"))
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    let system_tray = SystemTray::new().with_menu(system_tray_menu);

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "settings" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                    window.eval("window.location.href = '/settings';").unwrap();
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .on_window_event(|event| {
            if let WindowEvent::CloseRequested { api, .. } = event.event() {
                event.window().hide().unwrap();

                api.prevent_close();
            }
        })
        .setup(move |app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let app_handle = app.handle();

            let settings = load_settings(app.handle());

            let i18n = I18n::new(settings.language);

            let system_tray_menu = SystemTrayMenu::new()
                .add_item(CustomMenuItem::new(
                    "settings".to_string(),
                    i18n.translate("settings", None),
                ))
                .add_item(CustomMenuItem::new(
                    "quit".to_string(),
                    i18n.translate("quit", None),
                ));
            app_handle.tray_handle().set_menu(system_tray_menu)?;

            app.manage(Mutex::new(settings));

            tauri::async_runtime::spawn(async move {
                loop {
                    let settings = app_handle.state::<Mutex<Settings>>();
                    let settings = settings.lock().unwrap();

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

                    app_handle
                        .tray_handle()
                        .set_title(&format!("{}{:.2}", settings.currency_symbol, earned))
                        .unwrap();

                    app_handle.emit_all("update-earnings", earned).unwrap();

                    thread::sleep(Duration::from_secs(1));
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![save_settings, load_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

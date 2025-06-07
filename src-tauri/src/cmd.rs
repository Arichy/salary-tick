use std::fs;

use chrono::NaiveTime;
use tauri::{Emitter, Manager};
use tracing::{debug, error};

use crate::{handle::APP_HANDLE, i18n, settings::Settings, tray::create_tray};

fn get_settings_file_path(app_handle: &tauri::AppHandle) -> std::path::PathBuf {
    app_handle
        .path()
        .app_data_dir()
        .unwrap()
        .join("settings.json")
}

#[tauri::command]
pub fn load_settings(app_handle: tauri::AppHandle) -> Settings {
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
pub fn save_settings(
    daily_salary: f64,
    start_time: String,
    end_time: String,
    currency_symbol: char,
    language: String,
) -> Result<(), String> {
    let start_time = NaiveTime::parse_from_str(&start_time, "%H:%M").unwrap();
    let end_time = NaiveTime::parse_from_str(&end_time, "%H:%M").unwrap();

    let mut settings = Settings::get().read().unwrap().clone();

    let prev_language = settings.language;

    settings.daily_salary = daily_salary;
    settings.start_time = start_time;
    settings.end_time = end_time;
    settings.currency_symbol = currency_symbol;
    settings.language = match language.as_str() {
        "en" => i18n::Language::EN,
        "zh" => i18n::Language::ZH,
        _ => i18n::Language::EN,
    };

    debug!("update settings: {settings:?}");

    let app_handle = APP_HANDLE.get().unwrap();

    app_handle
        .emit("update-settings", settings.clone())
        .unwrap();

    let settings_json = serde_json::to_string(&settings).map_err(|e| e.to_string())?;
    let settings_path = get_settings_file_path(&app_handle);

    // if the folder does not exist, create it
    if let Some(parent) = settings_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }

    let mut guard = Settings::get().write().unwrap();

    *guard = settings.clone();

    drop(guard);

    fs::write(settings_path, settings_json).map_err(|e| e.to_string())?;

    if prev_language != settings.language {
        update_language(settings.language);
    }

    Ok(())
}

fn update_language(language: i18n::Language) {
    let app_handle = APP_HANDLE.get().unwrap();
    app_handle.emit("update-language", language).unwrap();

    if let Err(e) = create_tray() {
        error!("Err when creating tray: {e:?}");
    }
}

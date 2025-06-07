use std::fs;

use chrono::NaiveTime;
use tauri::{Emitter, Manager};

use crate::{handle::APP_HANDLE, i18n, settings::Settings, update_language};

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
    println!("{settings_file_path:?}");

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
    // state: tauri::State<Arc<Mutex<Settings>>>,
    // app_handle: tauri::AppHandle,
    daily_salary: f64,
    start_time: String,
    end_time: String,
    currency_symbol: char,
    language: String,
) -> Result<(), String> {
    println!("start save settings");
    let start_time = NaiveTime::parse_from_str(&start_time, "%H:%M").unwrap();
    let end_time = NaiveTime::parse_from_str(&end_time, "%H:%M").unwrap();

    // let settings = app_handle.state::<Mutex<Settings>>();
    println!("before lock");

    // let mut settings = settings.lock().unwrap();
    // let mut settings = match settings.lock() {
    //     Ok(s) => s,
    //     Err(e) => {
    //         eprintln!("Err when lock: {e:?}");
    //         return Ok(());
    //     }
    // };
    let mut settings = Settings::get().read().unwrap().clone();

    println!("after lock");

    let prev_language = settings.language;
    println!("{prev_language:?}");

    settings.daily_salary = daily_salary;
    settings.start_time = start_time;
    settings.end_time = end_time;
    settings.currency_symbol = currency_symbol;
    settings.language = match language.as_str() {
        "en" => i18n::Language::EN,
        "zh" => i18n::Language::ZH,
        _ => i18n::Language::EN,
    };

    println!("update settings: {settings:?}");

    let app_handle = APP_HANDLE.get().unwrap();
    println!("got app_handle");
    app_handle
        .emit("update-settings", settings.clone())
        .unwrap();

    let settings_json = serde_json::to_string(&settings).map_err(|e| e.to_string())?;
    let settings_path = get_settings_file_path(&app_handle);
    println!("{settings_path:?}");

    // if the folder does not exist, create it
    if let Some(parent) = settings_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }

    println!("before get");
    // let mut guard = Settings::get().write().unwrap();
    let tmp = Settings::get();
    println!("afterget, before write");
    let mut guard = match tmp.write() {
        Ok(g) => g,
        Err(e) => {
            panic!("{e:?}");
        }
    };
    println!("after write");

    *guard = settings.clone();
    println!("after update");

    drop(guard);
    println!("after drop");

    fs::write(settings_path, settings_json).map_err(|e| e.to_string())?;

    if prev_language != settings.language {
        println!("update lang");
        update_language(settings.language);
    }
    println!("done");

    Ok(())
}

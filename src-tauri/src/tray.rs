use tauri::{
    menu::{Menu, MenuEvent, MenuItem},
    tray::{TrayIcon, TrayIconId},
    AppHandle, Manager,
};

use crate::{handle::APP_HANDLE, i18n::I18n, load_settings};

pub fn create_tray() -> anyhow::Result<TrayIcon> {
    let app_handle = APP_HANDLE.get().unwrap();
    let settings = load_settings(app_handle.clone());
    let tray_incon_id = TrayIconId::new("main");
    let tray = app_handle.tray_by_id(&tray_incon_id).unwrap();

    let i18n = I18n::new(settings.language);

    let settings = MenuItem::with_id(
        app_handle,
        "settings",
        i18n.translate("settings", None),
        true,
        None::<&str>,
    )?;

    let quit = MenuItem::with_id(
        app_handle,
        "quit",
        i18n.translate("quit", None),
        true,
        None::<&str>,
    )?;

    let menu = Menu::with_items(app_handle, &[&settings, &quit])?;

    tray.set_menu(Some(menu))?;
    tray.on_menu_event(on_menu_event);
    tray.set_show_menu_on_left_click(false)?;

    Ok(tray)
}

fn on_menu_event(handle: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "settings" => {
            let window = handle.get_webview_window("main").unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
            window.eval("window.location.href = '/settings';").unwrap();
        }
        "quit" => {
            handle.exit(0);
        }
        _ => {}
    }
}

use std::{io::Cursor, sync::OnceLock};

use ab_glyph::{Font, FontVec, PxScale};
use anyhow::Result;
use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
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

static FONT: OnceLock<FontVec> = OnceLock::new();

fn get_font() -> &'static FontVec {
    FONT.get_or_init(|| {
        let font_bytes: Vec<u8> = include_bytes!("../assets/fonts/SourceCodePro-Bold.ttf").to_vec();
        FontVec::try_from_vec(font_bytes).unwrap()
    })
}

pub fn update_icon(earned: &str) -> Result<()> {
    let icon = include_bytes!("../icons/icon.png");
    let img = image::load_from_memory(icon)?;
    let (width, height) = (img.width(), img.height());

    let font = get_font();

    let text_color = Rgba([255u8, 255, 255, 255]);
    let base_size = height as f32;
    let scale_factor = 1.08;
    let scale = PxScale::from(base_size * scale_factor);

    let text_width = calculate_text_width(scale, earned);

    let mut image = ImageBuffer::new(width + text_width as u32, height);
    image::imageops::replace(&mut image, &img, 0, 0);

    draw_text_mut(
        &mut image,
        text_color,
        width as i32,
        -((scale_factor - 1.0) / 2.0 * base_size) as i32,
        scale,
        font,
        earned,
    );

    let mut bytes = Vec::new();
    let mut cursor = Cursor::new(&mut bytes);
    image.write_to(&mut cursor, image::ImageFormat::Png)?;

    let app_handle = APP_HANDLE.get().expect("AppHandle not initialized");
    let tray = app_handle.tray_by_id("main").unwrap();
    tray.set_icon(Some(tauri::image::Image::from_bytes(&bytes)?))?;

    Ok(())
}

fn calculate_text_width(scale: PxScale, text: &str) -> f32 {
    let font = get_font();

    let h_scale_factor = scale.x / font.height_unscaled();

    let glyph_id = font.glyph_id('1');
    let c_width = font.h_advance_unscaled(glyph_id) * h_scale_factor;

    c_width * text.len() as f32
}

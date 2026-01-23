use std::{str::FromStr, sync::Mutex};

use serde::Deserialize;
use strum::{Display, EnumString};
use tauri::{
    AppHandle, Emitter, Manager,
    image::Image,
    menu::{Menu, MenuBuilder, MenuId, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
};

use crate::{
    AppState,
    commands::windows::{show_dashboard_window, show_main_window},
    constants::TRAY_ID,
    types::settings::AppSettings,
    utils::platform::{is_macos, is_windows},
};

const ICON_NO_CONFIG: &[u8] = include_bytes!("../icons/tray/tray-no-config.png");
const ICON_ERROR: &[u8] = include_bytes!("../icons/tray/tray-error.png");
const ICON_SUCCESS: &[u8] = include_bytes!("../icons/tray/tray-success.png");
const ICON_RUNNING: &[u8] = include_bytes!("../icons/tray/tray-running.png");
const ICON_PAUSED: &[u8] = include_bytes!("../icons/tray/tray-paused.png");
#[cfg(target_os = "macos")]
const ICON_QB_MAC: &[u8] = include_bytes!("../icons/tray/tray-icon.png");
#[cfg(target_os = "windows")]
const ICON_QB_WIN: &[u8] = include_bytes!("../icons/tray/tray-icon-win.png");
#[cfg(target_os = "linux")]
const ICON_QB_LINUX: &[u8] = include_bytes!("../icons/tray/tray-icon-linux.png");

#[derive(Debug, Clone, Copy, EnumString, Display, Deserialize, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum TrayStatus {
    NoConfig,
    Running,
    Paused,
    Success,
    Error,
}

#[derive(Debug, Clone, Copy, EnumString, Display, Deserialize, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum TrayItem {
    Dashboard,
    ViewBuilds,
    ClearBuilds,
    ViewAlerts,
    ClearAlerts,
    Paused,
    Preferences,
    Quit,
}

impl TryFrom<MenuId> for TrayItem {
    type Error = String;

    fn try_from(value: MenuId) -> Result<Self, Self::Error> {
        let s = value.0.as_str();
        TrayItem::from_str(s).map_err(|_| format!("Invalid tray item id {:?}", value))
    }
}

fn build_tray_menu(app: &AppHandle) -> tauri::Result<Menu<tauri::Wry>> {
    let menu = MenuBuilder::new(app)
        .item(&MenuItem::with_id(
            app,
            TrayItem::Dashboard,
            "Dashboard",
            true,
            None::<&str>,
        )?)
        .item(&PredefinedMenuItem::separator(app)?)
        .item(&MenuItem::with_id(
            app,
            TrayItem::ViewBuilds,
            "Build History",
            true,
            None::<&str>,
        )?)
        .item(&MenuItem::with_id(
            app,
            TrayItem::ClearBuilds,
            "Clear Build History",
            true,
            None::<&str>,
        )?)
        .item(&PredefinedMenuItem::separator(app)?)
        .item(&MenuItem::with_id(
            app,
            TrayItem::ViewAlerts,
            "Alert History",
            true,
            None::<&str>,
        )?)
        .item(&MenuItem::with_id(
            app,
            TrayItem::ClearAlerts,
            "Clear Alert History",
            true,
            None::<&str>,
        )?)
        .item(&PredefinedMenuItem::separator(app)?)
        .item(&MenuItem::with_id(
            app,
            TrayItem::Preferences,
            "Preferences",
            true,
            None::<&str>,
        )?)
        .item(&PredefinedMenuItem::separator(app)?)
        .item(&MenuItem::with_id(
            app,
            TrayItem::Quit,
            "Quit",
            true,
            None::<&str>,
        )?)
        .build()?;
    Ok(menu)
}

fn initial_icon() -> tauri::Result<Image<'static>> {
    #[cfg(target_os = "macos")]
    {
        Image::from_bytes(ICON_QB_MAC)
    }
    #[cfg(target_os = "windows")]
    {
        Image::from_bytes(ICON_QB_WIN)
    }
    #[cfg(target_os = "linux")]
    {
        Image::from_bytes(ICON_QB_LINUX)
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    Err(tauri::Error::Other("Unsupported platform".to_string()))
}

#[allow(unused)]
fn get_tray_status_icon(status: TrayStatus) -> tauri::Result<Image<'static>> {
    match status {
        TrayStatus::NoConfig => Ok(Image::from_bytes(ICON_NO_CONFIG)?),
        TrayStatus::Running => Ok(Image::from_bytes(ICON_RUNNING)?),
        TrayStatus::Paused => Ok(Image::from_bytes(ICON_PAUSED)?),
        TrayStatus::Success => Ok(Image::from_bytes(ICON_SUCCESS)?),
        TrayStatus::Error => Ok(Image::from_bytes(ICON_ERROR)?),
    }
}

#[allow(unused)]
pub fn update_tray_icon(app: &AppHandle, status: TrayStatus) {
    if is_windows() {
        return;
    }

    let Some(tray) = app.tray_by_id(TRAY_ID) else {
        return;
    };

    if let Ok(icon) = get_tray_status_icon(status) {
        let _ = tray.set_icon(Some(icon));
    }
}

fn get_app_settings(app: &AppHandle) -> AppSettings {
    app.state::<Mutex<AppState>>()
        .lock()
        .unwrap()
        .settings
        .clone()
}

pub fn create_tray(app: &AppHandle) -> tauri::Result<()> {
    let menu = build_tray_menu(app)?;
    let app = app.clone();
    let initial_icon = initial_icon()?;
    let _ = TrayIconBuilder::with_id("tray")
        .icon(initial_icon)
        .icon_as_template(is_macos())
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event({
            move |app: &AppHandle, event| {
                let item = TrayItem::try_from(event.id);
                let settings = get_app_settings(app);
                if item == Ok(TrayItem::Quit) {
                    app.exit(0);
                    return;
                }

                if !settings.is_configured() {
                    let _ = app.emit("menu-view-settings", ());
                    let _ = show_main_window(app.clone(), Some("Preferences"));
                    return;
                }

                match item {
                    Ok(TrayItem::Dashboard) => {
                        tracing::debug!("Show dashboard event received");
                        let _ = show_dashboard_window(app.clone());
                    }
                    Ok(TrayItem::ViewBuilds) => {
                        tracing::debug!("View builds event received");
                        let _ = app.emit("menu-view-builds", ());
                        let _ = show_main_window(app.clone(), Some("Builds"));
                    }
                    Ok(TrayItem::ClearBuilds) => {
                        tracing::debug!("Clear builds event received");
                        let state = app.state::<Mutex<AppState>>();
                        let _ = state.lock().unwrap().clear_builds();
                        let _ = app.emit("menu-view-builds", ());
                    }
                    Ok(TrayItem::ViewAlerts) => {
                        tracing::debug!("View alerts event received");
                        let _ = app.emit("menu-view-alerts", ());
                        let _ = show_main_window(app.clone(), Some("Alerts"));
                    }
                    Ok(TrayItem::ClearAlerts) => {
                        tracing::debug!("Clear alerts event received");
                        let state = app.state::<Mutex<AppState>>();
                        let _ = state.lock().unwrap().clear_alerts();
                        let _ = app.emit("menu-view-alerts", ());
                    }
                    Ok(TrayItem::Preferences) => {
                        tracing::debug!("Preferences event received");
                        let _ = app.emit("menu-view-settings", ());
                        let _ = show_main_window(app.clone(), Some("Settings"));
                    }
                    _ => {
                        tracing::error!("Unhandled tray item event");
                    }
                }
            }
        })
        .on_tray_icon_event({
            move |tray, event| {
                tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);
                if let tauri::tray::TrayIconEvent::Click { .. } = event {
                    let _ = tray.set_visible(true);
                }
            }
        })
        .build(&app);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tray_item_from_menu_id() {
        let menu_id = MenuId::from("dashboard");
        let tray_item = TrayItem::try_from(menu_id).unwrap();
        assert_eq!(tray_item, TrayItem::Dashboard);

        let invalid_menu_id = MenuId::from("invalid_item");
        let result = TrayItem::try_from(invalid_menu_id);
        assert!(result.is_err());
    }
}

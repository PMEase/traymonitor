use std::str::FromStr;

use serde::Deserialize;
use strum::{Display, EnumString};
use tauri::{
    AppHandle, Emitter,
    image::Image,
    menu::{Menu, MenuBuilder, MenuId, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
};

use crate::{constants::TRAY_ID, utils::platform::is_windows};

const ICON_CONFIG: &[u8] = include_bytes!("../icons/tray/monitor-config.png");
const ICON_ERROR: &[u8] = include_bytes!("../icons/tray/monitor-error.png");
const ICON_OK: &[u8] = include_bytes!("../icons/tray/monitor-ok.png");
const ICON_PAUSED: &[u8] = include_bytes!("../icons/tray/monitor-paused.png");
const ICON_QB: &[u8] = include_bytes!("../icons/tray/monitor-qb.png");
const ICON_STARTED: &[u8] = include_bytes!("../icons/tray/monitor-started.png");

#[derive(Debug, Clone, Copy, EnumString, Display, Deserialize, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum TrayStatus {
    Config,
    Started,
    Paused,
    Ok,
    Error,
}

#[derive(Debug, Clone, Copy, EnumString, Display, Deserialize, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum TrayItem {
    ShowConfigurations,
    ViewBuilds,
    ClearBuilds,
    ViewAlerts,
    ClearAlerts,
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
            TrayItem::ShowConfigurations,
            "Show Configurations",
            true,
            None::<&str>,
        )?)
        .item(&PredefinedMenuItem::separator(app)?)
        .item(&MenuItem::with_id(
            app,
            TrayItem::ViewBuilds,
            "View Builds",
            true,
            None::<&str>,
        )?)
        .item(&MenuItem::with_id(
            app,
            TrayItem::ClearBuilds,
            "Clear Builds",
            true,
            None::<&str>,
        )?)
        .item(&PredefinedMenuItem::separator(app)?)
        .item(&MenuItem::with_id(
            app,
            TrayItem::ViewAlerts,
            "View Alerts",
            true,
            None::<&str>,
        )?)
        .item(&MenuItem::with_id(
            app,
            TrayItem::ClearAlerts,
            "Clear Alerts",
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
    Image::from_bytes(ICON_QB)
}

#[allow(unused)]
fn get_tray_status_icon(status: TrayStatus) -> tauri::Result<Image<'static>> {
    match status {
        TrayStatus::Config => Ok(Image::from_bytes(ICON_CONFIG)?),
        TrayStatus::Started => Ok(Image::from_bytes(ICON_STARTED)?),
        TrayStatus::Paused => Ok(Image::from_bytes(ICON_PAUSED)?),
        TrayStatus::Ok => Ok(Image::from_bytes(ICON_OK)?),
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

pub fn create_tray(app: &AppHandle) -> tauri::Result<()> {
    let menu = build_tray_menu(app)?;
    let app = app.clone();
    let initial_icon = initial_icon()?;
    let _ = TrayIconBuilder::with_id("tray")
        .icon(initial_icon)
        .icon_as_template(true)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event({
            let _app_handle = app.clone();
            move |app: &AppHandle, event| match TrayItem::try_from(event.id) {
                Ok(TrayItem::ShowConfigurations) => {
                    tracing::info!("Show configurations event received");
                    let _ = app.emit("menu-show-configurations", ());
                }
                Ok(TrayItem::ViewBuilds) => {
                    tracing::info!("View builds event received");
                    let _ = app.emit("menu-view-builds", ());
                }
                Ok(TrayItem::ClearBuilds) => {
                    tracing::info!("Clear builds event received");
                    let _ = app.emit("menu-clear-builds", ());
                }
                Ok(TrayItem::ViewAlerts) => {
                    tracing::info!("View alerts event received");
                    let _ = app.emit("menu-view-alerts", ());
                }
                Ok(TrayItem::ClearAlerts) => {
                    tracing::info!("Clear alerts event received");
                    let _ = app.emit("menu-clear-alerts", ());
                }
                Ok(TrayItem::Preferences) => {
                    tracing::info!("Preferences event received");
                    let _ = app.emit("menu-preferences", ());
                }
                Ok(TrayItem::Quit) => {
                    tracing::info!("Quit event received");
                    app.exit(0);
                }
                _ => {
                    tracing::error!("Unhandled tray item event");
                }
            }
        })
        .on_tray_icon_event({
            let _app_handle = app.clone();
            move |tray, event| {
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
        let menu_id = MenuId::from("show_configurations");
        let tray_item = TrayItem::try_from(menu_id).unwrap();
        assert_eq!(tray_item, TrayItem::ShowConfigurations);

        let invalid_menu_id = MenuId::from("invalid_item");
        let result = TrayItem::try_from(invalid_menu_id);
        assert!(result.is_err());
    }
}

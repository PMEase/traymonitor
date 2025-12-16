use std::str::FromStr;

use serde::Deserialize;
use strum::{Display, EnumString};
use tauri::{
    image::Image,
    menu::{Menu, MenuId, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter,
};

use crate::logger;

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

// impl From<TrayItem> for MenuId {
//     fn from(value: TrayItem) -> Self {
//         value.to_string().into()
//     }
// }

impl TryFrom<MenuId> for TrayItem {
    type Error = String;

    fn try_from(value: MenuId) -> Result<Self, Self::Error> {
        let s = value.0.as_str();
        TrayItem::from_str(s).map_err(|_| format!("Invalid tray item id {:?}", value))
    }
}

pub fn create_tray(app: &AppHandle) -> tauri::Result<()> {
    let menu = Menu::with_items(
        app,
        &[
            &MenuItem::with_id(
                app,
                TrayItem::ShowConfigurations,
                "Configurations",
                true,
                None::<&str>,
            )?,
            &MenuItem::with_id(app, TrayItem::ViewBuilds, "View Builds", true, None::<&str>)?,
            &MenuItem::with_id(
                app,
                TrayItem::ClearBuilds,
                "Clear Builds",
                true,
                None::<&str>,
            )?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, TrayItem::ViewAlerts, "View Alerts", true, None::<&str>)?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(
                app,
                TrayItem::Preferences,
                "Preferences",
                true,
                None::<&str>,
            )?,
            &MenuItem::with_id(app, TrayItem::Quit, "Quit", true, None::<&str>)?,
        ],
    )?;

    let app = app.clone();
    let _ = TrayIconBuilder::with_id("tray")
        .icon(Image::from_bytes(include_bytes!("../icons/64x64.png"))?)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event({
            let _app_handle = app.clone();
            move |app: &AppHandle, event| match TrayItem::try_from(event.id) {
                Ok(TrayItem::Quit) => {
                    app.exit(0);
                }
                Ok(TrayItem::ShowConfigurations) => {
                    logger::info!("Showing configuration tree ...");
                    app.emit("menu-show-configurations", ()).unwrap()
                }
                Ok(TrayItem::ViewBuilds) => {
                    logger::info!("Showing recent builds ...");
                }
                Ok(TrayItem::ClearBuilds) => {
                    logger::info!("Clearing recent builds ...");
                }
                Ok(TrayItem::ViewAlerts) => {
                    logger::info!("View recent alerts ...");
                }
                Ok(TrayItem::ClearAlerts) => {
                    logger::info!("Clearing recent alerts ...");
                }
                Ok(TrayItem::Preferences) => {}
                _ => {}
            }
        })
        // .on_tray_icon_event({})
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

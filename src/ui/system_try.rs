use std::{collections::HashMap, path::Path};

use tray_icon::{
    menu::{Menu, MenuId, MenuItem, PredefinedMenuItem},
    Icon, TrayIcon, TrayIconBuilder,
};

use crate::APP_TITLE;

use super::icons::{get_try_icons, TryIcons};

pub static MENU_ITEM_STATUS_ID: &str = "STATUS";
pub static MENU_ITEM_EXIT_ID: &str = "EXIT";

enum MenuItemId {
    Status,
    Exit,
}

pub struct SystemTry {
    instance: TrayIcon,
    icons: TryIcons,
}

pub enum TryStatus {
    Neutral,
    Running,
    Warning,
    Error,
}

impl SystemTry {
    pub fn new() -> Self {
        let icons = get_try_icons();
        let menu = Box::new(Menu::new());

        let title_menu_item = MenuItem::with_id(
            MenuId(MENU_ITEM_STATUS_ID.to_string()),
            APP_TITLE,
            true,
            None,
        );
        let separator_menu_item = PredefinedMenuItem::separator();
        let exit_menu_item = MenuItem::with_id(
            MenuId(MENU_ITEM_EXIT_ID.to_string()),
            "Exit".to_string(),
            true,
            None,
        );

        menu.append(&title_menu_item);
        menu.append(&separator_menu_item);
        menu.append(&exit_menu_item);

        let instance = TrayIconBuilder::new()
            .with_menu(menu)
            .with_tooltip(APP_TITLE)
            .with_icon(icons.neutral.clone())
            .build()
            .unwrap();

        instance.set_visible(true);

        SystemTry { icons, instance }
    }

    pub fn set_status(&mut self, status: TryStatus) {
        let new_icon = match status {
            TryStatus::Neutral => self.icons.neutral.clone(),
            TryStatus::Running => self.icons.running.clone(),
            TryStatus::Warning => self.icons.warning.clone(),
            TryStatus::Error => self.icons.error.clone(),
        };

        // Update the instance's icon
        self.instance.set_icon(Some(new_icon));
    }
}

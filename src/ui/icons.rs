use std::{io::Read, path::Path};

use crate::utils::file::{load_try_icon, load_window_icon};
use tray_icon::Icon as TryIcon;
use winit::window::Icon as WindowIcon;

const WINDOW_ICON_PNG: &'static [u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/window_icon.png"
));

const NEUTRAL_ICON_PNG: &'static [u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/icon_neutral.png"
));

const RUNNING_ICON_PNG: &'static [u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/icon_running.png"
));

const WARNING_ICON_PNG: &'static [u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/icon_warning.png"
));

const ERROR_ICON_PNG: &'static [u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/icon_error.png"
));

pub struct TryIcons {
    pub neutral: TryIcon,
    pub running: TryIcon,
    pub warning: TryIcon,
    pub error: TryIcon,
}

pub fn get_try_icons() -> TryIcons {
    TryIcons {
        neutral: load_try_icon(NEUTRAL_ICON_PNG),
        running: load_try_icon(RUNNING_ICON_PNG),
        warning: load_try_icon(WARNING_ICON_PNG),
        error: load_try_icon(ERROR_ICON_PNG),
    }
}

pub fn get_window_icon() -> WindowIcon {
    load_window_icon(WINDOW_ICON_PNG)
}

use std::path::Path;

use crate::utils::file::{load_try_icon, load_window_icon};
use tray_icon::Icon as TryIcon;
use winit::window::Icon as WindowIcon;

pub struct TryIcons {
    pub neutral: TryIcon,
    pub running: TryIcon,
    pub warning: TryIcon,
    pub error: TryIcon,
}

pub fn get_try_icons() -> TryIcons {
    TryIcons {
        neutral: load_try_icon(Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "\\static\\icon_neutral.png"
        ))),
        running: load_try_icon(Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "\\static\\icon_running.png"
        ))),
        warning: load_try_icon(Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "\\static\\icon_warning.png"
        ))),
        error: load_try_icon(Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "\\static\\icon_error.png"
        ))),
    }
}

pub fn get_window_icon() -> WindowIcon {
    load_window_icon(Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\static\\window_icon.png"
    )))
}

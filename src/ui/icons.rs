use std::path::Path;

use crate::utils::file::load_icon;
use tray_icon::Icon;

pub struct TryIcons {
    pub neutral: Icon,
    pub running: Icon,
    pub warning: Icon,
    pub error: Icon,
}

pub fn get_try_icons() -> TryIcons {
    TryIcons {
        neutral: load_icon(Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "\\static\\icon_neutral.png"
        ))),
        running: load_icon(Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "\\static\\icon_running.png"
        ))),
        warning: load_icon(Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "\\static\\icon_warning.png"
        ))),
        error: load_icon(Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "\\static\\icon_error.png"
        ))),
    }
}

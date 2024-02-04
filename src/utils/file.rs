use encoding::{DecoderTrap, EncoderTrap};
use std::fs;
use std::path::Path;
use tray_icon::Icon as TryIcon;
use winit::window::Icon as WindowIcon;

pub fn read_windows1252_file(file_path: &str) -> Result<String, String> {
    let content =
        fs::read(file_path).map_err(|e| format!("Error reading SimConnect.xml: {}", e))?;
    let encoding = encoding::label::encoding_from_whatwg_label("windows-1252").unwrap();
    let decoded = encoding.decode(&content, DecoderTrap::Replace);
    decoded.map_err(|e| format!("Error decoding Windows-1252 content: {}", e))
}

pub fn write_windows1252_file(file_path: &str, content: &str) -> Result<(), String> {
    let encoding = encoding::label::encoding_from_whatwg_label("windows-1252").unwrap();
    let encoded_content_result = encoding.encode(content, EncoderTrap::Replace);

    match encoded_content_result {
        Ok(encoded_content) => fs::write(file_path, encoded_content)
            .map_err(|e| format!("Error writing SimConnect.xml: {}", e)),
        Err(err) => Err(format!("Error encoding content to Windows-1252: {}", err)),
    }
}

fn load_icon_file(path: &std::path::Path) -> (Vec<u8>, u32, u32) {
    let image = image::open(path)
        .expect("Failed to open icon path")
        .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    (rgba, width, height)
}

pub fn load_try_icon(path: &std::path::Path) -> TryIcon {
    let (icon_rgba, icon_width, icon_height) = load_icon_file(path);
    TryIcon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open try icon")
}

pub fn load_window_icon(path: &Path) -> WindowIcon {
    let (icon_rgba, icon_width, icon_height) = load_icon_file(path);
    WindowIcon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open window icon")
}

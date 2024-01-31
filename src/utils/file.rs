use encoding::{DecoderTrap, EncoderTrap};
use std::fs;

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

pub fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

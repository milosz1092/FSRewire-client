use encoding::{DecoderTrap, EncoderTrap};
use image::codecs::png::PngDecoder;
use image::{load_from_memory, GenericImageView, ImageDecoder};
use std::fs;
use std::io::{Cursor, Read};
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

fn load_png(png_icon_data: &'static [u8]) -> (Vec<u8>, u32, u32) {
    let cursor = Cursor::new(png_icon_data);
    let dynamic_image = load_from_memory(&cursor.into_inner()).unwrap();

    let (width, height) = dynamic_image.dimensions();

    // Convert the image to RGBA format
    let rgba_image = dynamic_image.to_rgba8();

    // Extract RGBA bytes into a Vec<u8>
    let rgba_data = rgba_image.into_raw();
    (rgba_data, width, height)
}

pub fn load_try_icon(png_icon_data: &'static [u8]) -> TryIcon {
    let (icon_rgba, icon_width, icon_height) = load_png(png_icon_data);
    TryIcon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open try icon")
}

pub fn load_window_icon(png_icon_data: &'static [u8]) -> WindowIcon {
    let (icon_rgba, icon_width, icon_height) = load_png(png_icon_data);
    WindowIcon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open window icon")
}

#![allow(unused)]

mod schema;
mod utils;

use std::path::Path;
use utils::simconnect::update_simconnect_config;
use utils::{file::load_icon, msfs::check_if_msfs_running};

use tray_icon::{
    menu::{AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIconBuilder, TrayIconEvent,
};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

fn main() {
    let is_msfs_running = check_if_msfs_running();

    let icon_neutral = load_icon(Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\static\\icon_neutral.png"
    )));

    let icon_running = load_icon(Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\static\\icon_running.png"
    )));

    let icon_warning = load_icon(Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\static\\icon_warning.png"
    )));

    let icon_error = load_icon(Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\static\\icon_error.png"
    )));

    let event_loop = EventLoopBuilder::new().build().unwrap();

    let mut tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(Menu::new()))
            .with_tooltip("FSRewire-client")
            .with_icon(icon_neutral)
            .build()
            .unwrap(),
    )
    .unwrap();

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    let update_config_result = update_simconnect_config();

    if let Err(error) = update_config_result {
        tray_icon.set_icon(Some(icon_error));
    } else {
        tray_icon.set_icon(Some(icon_running));
    }

    event_loop.run(move |_event, event_loop| {
        event_loop.set_control_flow(ControlFlow::Wait);

        if let Ok(event) = tray_channel.try_recv() {
            println!("{event:?}");
        }
    });
}

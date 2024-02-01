#![allow(unused)]

mod schema;
mod ui;
mod utils;

use ui::icons::get_try_icons;
use utils::msfs::check_if_msfs_running;
use utils::simconnect::update_simconnect_config;

use tray_icon::{
    menu::{AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIconBuilder, TrayIconEvent,
};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

fn main() {
    let is_msfs_running = check_if_msfs_running();

    let event_loop = EventLoopBuilder::new().build().unwrap();

    let try_icons = get_try_icons();

    let mut tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(Menu::new()))
            .with_tooltip("FSRewire-client")
            .with_icon(try_icons.neutral)
            .build()
            .unwrap(),
    )
    .unwrap();

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    let update_config_result = update_simconnect_config();

    match update_config_result {
        Ok(config) => {
            if (config.is_changed && is_msfs_running) {
                tray_icon.set_icon(Some(try_icons.warning));
            } else {
                tray_icon.set_icon(Some(try_icons.running));
            }
        }
        Err(message) => {
            tray_icon.set_icon(Some(try_icons.error));
        }
    }

    event_loop.run(move |_event, event_loop| {
        event_loop.set_control_flow(ControlFlow::Wait);

        if let Ok(event) = tray_channel.try_recv() {
            println!("{event:?}");
        }
    });
}

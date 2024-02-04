#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

mod schema;
mod ui;
mod utils;

use ui::system_try::{SystemTry, TryStatus, MENU_ITEM_EXIT_ID, MENU_ITEM_STATUS_ID};
use utils::simconnect::update_simconnect_config;
use utils::{msfs::check_if_msfs_running, wgpu::configure_wgpu};

use tray_icon::menu::MenuEvent;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::{Theme, Window, WindowBuilder, WindowButtons},
};

pub static APP_TITLE: &str = "FSRewire-client";

async fn run(window: &Window, event_loop: EventLoop<()>) {
    let mut system_try = SystemTry::new();

    let (device, queue, viewport) = configure_wgpu(window).await;

    let redraw = || {
        let frame = viewport.get_current_texture().unwrap();

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.02,
                            g: 0.02,
                            b: 0.02,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        queue.submit(Some(encoder.finish()));
        frame.present();
    };

    redraw();
    window.set_visible(true);
    window.focus_window();

    let is_msfs_running = check_if_msfs_running();

    let menu_channel = MenuEvent::receiver();

    let update_config_result = update_simconnect_config();

    match update_config_result {
        Ok(config) => {
            if (config.is_changed && is_msfs_running) {
                system_try.set_status(TryStatus::Warning)
            } else {
                system_try.set_status(TryStatus::Running)
            }
        }
        Err(message) => system_try.set_status(TryStatus::Error),
    }

    event_loop.run(move |event: Event<()>, event_loop| {
        event_loop.set_control_flow(ControlFlow::Wait);

        if let Event::WindowEvent { window_id, event } = event {
            match event {
                WindowEvent::CloseRequested => {
                    window.set_visible(false);
                }
                WindowEvent::RedrawRequested => {
                    // redraw();
                }
                _ => {}
            }
        }

        if let Ok(event) = menu_channel.try_recv() {
            if event.id.0 == MENU_ITEM_EXIT_ID {
                std::process::exit(0);
            } else if event.id.0 == MENU_ITEM_STATUS_ID {
                if window.is_minimized().is_some() && window.is_minimized().unwrap() == true {
                    window.set_visible(false);
                }

                if (window.is_visible().is_some() && window.is_visible().unwrap() == false) {
                    window.set_visible(true);
                }

                window.focus_window();
            }
        }
    });
}

fn main() {
    env_logger::init();

    let event_loop = EventLoopBuilder::new().build().unwrap();

    let window = WindowBuilder::new()
        .with_title(APP_TITLE)
        .with_theme(Some(Theme::Dark))
        .with_active(false)
        .with_resizable(false)
        .with_visible(false)
        .with_inner_size(PhysicalSize {
            width: 600,
            height: 300,
        })
        .with_position(PhysicalPosition { x: 200, y: 200 })
        .with_enabled_buttons(WindowButtons::MINIMIZE.union(WindowButtons::CLOSE))
        .build(&event_loop)
        .unwrap();

    pollster::block_on(run(&window, event_loop));
}

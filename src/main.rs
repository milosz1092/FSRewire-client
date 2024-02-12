#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

include!("./env.rs");

mod schema;
mod state;
mod ui;
mod utils;

use ui::{
    icons::get_window_icon,
    system_try::{SystemTry, MENU_ITEM_EXIT_ID, MENU_ITEM_STATUS_ID},
};
use utils::{
    msfs::check_if_msfs_running,
    simconnect::update_simconnect_config,
    udp::{udp_broadcast_thread, UDP_THREAD_STATUS_ERROR, UDP_THREAD_STATUS_OK},
    wgpu::configure_wgpu,
};

use tray_icon::menu::MenuEvent;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::{Theme, Window, WindowBuilder, WindowButtons},
};

use glyphon::{
    Attrs, Buffer, Color, Family, Metrics, Resolution, Shaping, Style, TextArea, TextBounds, Weight,
};

use std::{sync::mpsc, thread};

use crate::state::{AppState, AppStatus};

pub static APP_TITLE: &str = "FSRewire-client";

async fn run(window: &Window, app_state: &mut AppState, event_loop: EventLoop<()>) {
    let mut system_try = SystemTry::new();
    let (udp_thread_sender, udp_thread_receiver) = mpsc::channel();

    let (
        device,
        queue,
        viewport,
        mut font_system,
        mut swash_cache,
        mut text_atlas,
        mut text_renderer,
    ) = configure_wgpu(window).await;

    let mut text_app_header = Buffer::new(&mut font_system, Metrics::new(22.0, 24.0));
    let mut text_app_version = Buffer::new(&mut font_system, Metrics::new(14.0, 16.0));
    let mut text_app_status = Buffer::new(&mut font_system, Metrics::new(22.0, 24.0));
    let mut text_app_message = Buffer::new(&mut font_system, Metrics::new(20.0, 22.0));

    let physical_width = window.inner_size().width;
    let physical_height = window.inner_size().height;

    {
        // text_app_header
        text_app_header.set_size(
            &mut font_system,
            physical_width as f32,
            physical_height as f32,
        );
        text_app_header.set_text(
            &mut font_system,
            "Discovery Service for Flight Simulator Host",
            Attrs::new().family(Family::SansSerif).weight(Weight::BOLD),
            Shaping::Advanced,
        );
        text_app_header.set_redraw(false);
    }

    {
        // text_app_status
        text_app_status.set_size(
            &mut font_system,
            physical_width as f32,
            physical_height as f32,
        );
        text_app_status.set_text(
            &mut font_system,
            "Status:",
            Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
        );
        text_app_status.set_redraw(false);
    }

    {
        // text_app_version
        text_app_version.set_size(
            &mut font_system,
            physical_width as f32,
            physical_height as f32,
        );
        text_app_version.set_text(
            &mut font_system,
            RELESE_TAG,
            Attrs::new().family(Family::Monospace),
            Shaping::Advanced,
        );
        text_app_version.set_redraw(false);
    }

    text_app_message.set_size(
        &mut font_system,
        physical_width as f32,
        physical_height as f32,
    );

    let mut redraw = |app_state: &AppState| {
        let mut text_areas: Vec<TextArea> = Vec::new();

        {
            // text_app_message
            text_app_message.set_text(
                &mut font_system,
                &app_state.msg_text,
                Attrs::new().family(Family::SansSerif).style(Style::Italic),
                Shaping::Advanced,
            );

            text_areas.push(TextArea {
                buffer: &text_app_message,
                left: 100.0,
                top: 125.0,
                scale: 1.0,
                bounds: TextBounds {
                    left: 0,
                    top: 0,
                    right: physical_width as i32,
                    bottom: physical_height as i32,
                },
                default_color: Color::rgb(220, 220, 220),
            });
        }

        text_areas.push(TextArea {
            buffer: &text_app_header,
            left: 75.0,
            top: 20.0,
            scale: 1.0,
            bounds: TextBounds {
                left: 0,
                top: 0,
                right: physical_width as i32,
                bottom: physical_height as i32,
            },
            default_color: Color::rgb(220, 220, 220),
        });

        text_areas.push(TextArea {
            buffer: &text_app_status,
            left: 100.0,
            top: 90.0,
            scale: 1.0,
            bounds: TextBounds {
                left: 0,
                top: 0,
                right: physical_width as i32,
                bottom: physical_height as i32,
            },
            default_color: Color::rgb(220, 220, 220),
        });

        text_areas.push(TextArea {
            buffer: &text_app_version,
            left: 480.0,
            top: 220.0,
            scale: 1.0,
            bounds: TextBounds {
                left: 0,
                top: 0,
                right: physical_width as i32,
                bottom: physical_height as i32,
            },
            default_color: Color::rgb(100, 100, 100),
        });

        text_renderer
            .prepare(
                &device,
                &queue,
                &mut font_system,
                &mut text_atlas,
                Resolution {
                    width: physical_width,
                    height: physical_height,
                },
                text_areas,
                &mut swash_cache,
            )
            .unwrap();

        let frame = viewport.get_current_texture().unwrap();

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

            text_renderer.render(&text_atlas, &mut rpass).unwrap();
        }

        queue.submit(Some(encoder.finish()));
        frame.present();

        text_atlas.trim();
    };

    redraw(&app_state);

    window.set_visible(true);
    window.focus_window();

    let is_msfs_running = check_if_msfs_running();

    let simconnect_config_result = update_simconnect_config();

    match simconnect_config_result {
        Ok(config) => {
            if (config.is_changed && is_msfs_running) {
                system_try.set_status(AppStatus::Warning);
                app_state.status = AppStatus::Warning;
                app_state.msg_text =
                    "⭕ Run this client before the simulator is started.".to_string();
            } else {
                thread::spawn(move || udp_broadcast_thread(udp_thread_sender, config.port));
            }
        }
        Err(_) => {
            system_try.set_status(AppStatus::Error);
            app_state.status = AppStatus::Error;
            app_state.msg_text = format!("🔴 Fatal error during SimConnect configuration.");
        }
    }

    let menu_channel = MenuEvent::receiver();

    event_loop.run(move |event: Event<()>, event_loop| {
        event_loop.set_control_flow(ControlFlow::Wait);

        if let Event::WindowEvent { window_id, event } = event {
            match event {
                WindowEvent::CloseRequested => {
                    window.set_visible(false);
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

        match udp_thread_receiver.try_recv() {
            Ok(message) => {
                if message == UDP_THREAD_STATUS_ERROR {
                    system_try.set_status(AppStatus::Error);
                    app_state.status = AppStatus::Error;
                    app_state.msg_text = "🔴 Fatal error during data broadcasting.".to_string();

                    redraw(&app_state);
                } else if message == UDP_THREAD_STATUS_OK && app_state.status != AppStatus::Running
                {
                    system_try.set_status(AppStatus::Running);
                    app_state.status = AppStatus::Running;
                    app_state.msg_text = "✅ Client is working normally.".to_string();

                    redraw(&app_state);
                }
            }
            _ => {}
        }
    });
}

fn main() {
    env_logger::init();

    let mut app_state = AppState::new();

    let event_loop = EventLoopBuilder::new().build().unwrap();

    let window = WindowBuilder::new()
        .with_title(APP_TITLE)
        .with_theme(Some(Theme::Dark))
        .with_active(false)
        .with_resizable(false)
        .with_visible(false)
        .with_inner_size(PhysicalSize {
            width: 600,
            height: 240,
        })
        .with_position(PhysicalPosition { x: 200, y: 200 })
        .with_enabled_buttons(WindowButtons::MINIMIZE.union(WindowButtons::CLOSE))
        .with_window_icon(Some(get_window_icon()))
        .build(&event_loop)
        .unwrap();

    pollster::block_on(run(&window, &mut app_state, event_loop));
}

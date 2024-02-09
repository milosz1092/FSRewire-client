#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

mod schema;
mod ui;
mod utils;

use ui::icons::get_window_icon;
use ui::system_try::{SystemTry, MENU_ITEM_EXIT_ID, MENU_ITEM_STATUS_ID};
use utils::simconnect::update_simconnect_config;
use utils::{msfs::check_if_msfs_running, wgpu::configure_wgpu};

use tray_icon::menu::MenuEvent;
use wgpu::{Device, MultisampleState, Queue, Surface, TextureFormat};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::{Icon, Theme, Window, WindowBuilder, WindowButtons},
};

use glyphon::{
    Attrs, Buffer, Color, Family, FontSystem, Metrics, Resolution, Shaping, Style, SwashCache,
    TextArea, TextAtlas, TextBounds, TextRenderer, Weight,
};

pub static APP_TITLE: &str = "FSRewire-client";

#[derive(Debug)]
pub enum AppStatus {
    Neutral,
    Running,
    Warning,
    Error,
}

#[derive(Debug)]
struct AppState {
    pub status: AppStatus,
    pub msg_text: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            status: AppStatus::Neutral,
            msg_text: None,
        }
    }
}

fn get_text_renderer(device: &Device, queue: &Queue, swapchain_format: TextureFormat) {}

async fn run(window: &Window, app_state: &mut AppState, event_loop: EventLoop<()>) {
    let mut system_try = SystemTry::new();

    let (
        device,
        queue,
        viewport,
        swapchain_format,
        mut font_system,
        mut swash_cache,
        mut text_atlas,
        mut text_renderer,
    ) = configure_wgpu(window).await;

    let mut text_app_header = Buffer::new(&mut font_system, Metrics::new(22.0, 42.0));
    let mut text_app_version = Buffer::new(&mut font_system, Metrics::new(14.0, 42.0));
    let mut text_app_status = Buffer::new(&mut font_system, Metrics::new(22.0, 42.0));
    let mut text_app_message = Buffer::new(&mut font_system, Metrics::new(20.0, 42.0));

    let physical_width = window.inner_size().width;
    let physical_height = window.inner_size().height;

    text_app_header.set_size(
        &mut font_system,
        physical_width as f32,
        physical_height as f32,
    );
    text_app_header.set_text(
        &mut font_system,
        "Discovery Service for Flight Simulator Host", //👋
        Attrs::new().family(Family::SansSerif).weight(Weight::BOLD),
        Shaping::Advanced,
    );

    text_app_status.set_size(
        &mut font_system,
        physical_width as f32,
        physical_height as f32,
    );
    text_app_message.set_size(
        &mut font_system,
        physical_width as f32,
        physical_height as f32,
    );

    text_app_version.set_size(
        &mut font_system,
        physical_width as f32,
        physical_height as f32,
    );
    text_app_version.set_text(
        &mut font_system,
        "ver 1.0.3",
        Attrs::new().family(Family::Monospace),
        Shaping::Advanced,
    );

    let mut redraw = |app_state: &AppState| {
        println!("redraw...{:#?}", app_state);

        let mut text_areas: Vec<TextArea> = Vec::new();

        let status_text = match (&app_state.status) {
            &AppStatus::Neutral => "Status: PENDING",
            &AppStatus::Error => "Status: ERROR",
            &AppStatus::Warning => "Status: WARNED",
            &AppStatus::Running => "Status: RUNNING",
        };

        text_app_status.set_text(
            &mut font_system,
            status_text,
            Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
        );

        if Option::is_some(&app_state.msg_text) {
            text_app_message.set_text(
                &mut font_system,
                &app_state.msg_text.as_ref().unwrap(),
                Attrs::new().family(Family::SansSerif).style(Style::Italic),
                Shaping::Advanced,
            );

            text_areas.push(TextArea {
                buffer: &text_app_message,
                left: 100.0,
                top: 130.0,
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
            top: 100.0,
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
            left: 520.0,
            top: 270.0,
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

    let update_config_result = update_simconnect_config();

    match update_config_result {
        Ok(config) => {
            if (config.is_changed && is_msfs_running) {
                system_try.set_status(AppStatus::Warning);
                app_state.status = AppStatus::Warning;
                app_state.msg_text = Some(
                    "Configuration has been changed during MSFS2020 runtime.\n
                    Let's restart Microsoft Flight Simulator 2020."
                        .to_string(),
                );
            } else {
                system_try.set_status(AppStatus::Running);
                app_state.status = AppStatus::Running;
                app_state.msg_text = Some("App is running normally.".to_string());
            }
        }
        Err(message) => {
            system_try.set_status(AppStatus::Error);
            app_state.status = AppStatus::Error;
            app_state.msg_text = Some(message.clone());
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
                WindowEvent::RedrawRequested => {
                    redraw(&app_state);
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
            height: 300,
        })
        .with_position(PhysicalPosition { x: 200, y: 200 })
        .with_enabled_buttons(WindowButtons::MINIMIZE.union(WindowButtons::CLOSE))
        .with_window_icon(Some(get_window_icon()))
        .build(&event_loop)
        .unwrap();

    pollster::block_on(run(&window, &mut app_state, event_loop));
}

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
    Attrs, Buffer, Color, Family, FontSystem, Metrics, Resolution, Shaping, SwashCache, TextArea,
    TextAtlas, TextBounds, TextRenderer, Weight,
};

pub static APP_TITLE: &str = "FSRewire-client";

pub enum AppStatus {
    Neutral,
    Running,
    Warning,
    Error,
}

struct AppState {
    status: AppStatus,
    error_msg: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            status: AppStatus::Neutral,
            error_msg: None,
        }
    }
}

fn get_text_renderer(device: &Device, queue: &Queue, swapchain_format: TextureFormat) {}

fn redraw(
    window: &Window,
    mut font_system: FontSystem,
    mut text_renderer: TextRenderer,
    mut text_atlas: TextAtlas,
    device: &Device,
    queue: &Queue,
    viewport: &Surface,
    mut swash_cache: SwashCache,
) {
    let physical_width = window.inner_size().width;
    let physical_height = window.inner_size().height;

    let mut text_first = Buffer::new(&mut font_system, Metrics::new(20.0, 42.0));
    let mut text_second = Buffer::new(&mut font_system, Metrics::new(14.0, 42.0));

    text_first.set_size(
        &mut font_system,
        physical_width as f32,
        physical_height as f32,
    );
    text_first.set_text(
        &mut font_system,
        "Hello world! 👋",
        Attrs::new().family(Family::SansSerif),
        Shaping::Advanced,
    );
    text_first.shape_until_scroll(&mut font_system);

    text_second.set_size(
        &mut font_system,
        physical_width as f32,
        physical_height as f32,
    );
    text_second.set_text(
        &mut font_system,
        "ver 1.0.3",
        Attrs::new().family(Family::Monospace),
        Shaping::Advanced,
    );
    text_second.shape_until_scroll(&mut font_system);

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
            [
                TextArea {
                    buffer: &text_first,
                    left: 0.0,
                    top: 0.0,
                    scale: 1.0,
                    bounds: TextBounds {
                        left: 0,
                        top: 0,
                        right: physical_width as i32,
                        bottom: physical_height as i32,
                    },
                    default_color: Color::rgb(220, 220, 220),
                },
                TextArea {
                    buffer: &text_second,
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
                },
            ],
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
}

async fn run(window: &Window, app_state: &AppState, event_loop: EventLoop<()>) {
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

    redraw(
        window,
        font_system,
        text_renderer,
        text_atlas,
        &device,
        &queue,
        &viewport,
        swash_cache,
    );

    window.set_visible(true);
    window.focus_window();

    let is_msfs_running = check_if_msfs_running();

    let update_config_result = update_simconnect_config();

    match update_config_result {
        Ok(config) => {
            if (config.is_changed && is_msfs_running) {
                system_try.set_status(AppStatus::Warning)
            } else {
                system_try.set_status(AppStatus::Running)
            }
        }
        Err(message) => system_try.set_status(AppStatus::Error),
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

    pollster::block_on(run(&window, &app_state, event_loop));
}

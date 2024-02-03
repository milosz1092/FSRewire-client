#![allow(unused)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod schema;
mod ui;
mod utils;

use ui::icons::get_try_icons;
use utils::msfs::check_if_msfs_running;
use utils::simconnect::update_simconnect_config;

use tray_icon::{
    menu::{
        accelerator::Accelerator, AboutMetadata, IconMenuItem, Menu, MenuEvent, MenuId, MenuItem,
        PredefinedMenuItem,
    },
    TrayIconBuilder, TrayIconEvent,
};
use winit::{
    dpi::{LogicalPosition, PhysicalPosition, PhysicalSize, Position},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::{Theme, Window, WindowAttributes, WindowBuilder, WindowButtons, WindowId},
};

static APP_TITLE: &str = "FSRewire-client";

async fn render(window: &Window) {
    let instance = wgpu::Instance::default();
    let viewport = instance.create_surface(&window).unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            // Request an adapter which can render to our surface
            compatible_surface: Some(&viewport),
            ..Default::default()
        })
        .await
        .expect("Failed to find an appropriate adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let size = window.inner_size();
    let config = viewport
        .get_default_config(&adapter, size.width, size.height)
        .unwrap();

    viewport.configure(&device, &config);

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
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
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
}

fn main() {
    env_logger::init();

    let is_msfs_running = check_if_msfs_running();

    let event_loop = EventLoopBuilder::new().build().unwrap();

    let try_icons = get_try_icons();

    let mut window = WindowBuilder::new()
        .with_title("FSRewire-client")
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

    pollster::block_on(render(&window));

    window.set_visible(true);
    window.focus_window();

    let menu = Box::new(Menu::new());
    let title_menu_item = MenuItem::new(APP_TITLE, true, None);
    let separator_menu_item = PredefinedMenuItem::separator();
    let exit_menu_item = MenuItem::new("Exit".to_string(), true, None);
    menu.append(&title_menu_item);
    menu.append(&separator_menu_item);
    menu.append(&exit_menu_item);

    let mut tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(menu)
            .with_tooltip(APP_TITLE)
            .with_icon(try_icons.neutral)
            .build()
            .unwrap(),
    )
    .unwrap();

    tray_icon.set_visible(true);

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

    event_loop.run(move |event: Event<()>, event_loop| {
        event_loop.set_control_flow(ControlFlow::Wait);

        if let Event::WindowEvent { window_id, event } = event {
            match event {
                WindowEvent::CloseRequested => {
                    window.set_visible(false);
                }
                WindowEvent::RedrawRequested => {}
                _ => {}
            }
        }

        if let Ok(event) = menu_channel.try_recv() {
            if event.id.0 == exit_menu_item.id().0 {
                std::process::exit(0);
            } else if event.id.0 == title_menu_item.id().0 {
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

    println!("Test");
}

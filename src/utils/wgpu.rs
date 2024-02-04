use wgpu::{Device, Queue, Surface};
use winit::window::Window;

pub async fn configure_wgpu(window: &Window) -> (Device, Queue, Surface<'_>) {
    let wgpu_instance = wgpu::Instance::default();
    let viewport = wgpu_instance.create_surface(window).unwrap();
    let adapter = wgpu_instance
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

    (device, queue, viewport)
}

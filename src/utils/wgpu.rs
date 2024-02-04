use wgpu::{
    CompositeAlphaMode, Device, PresentMode, Queue, Surface, SurfaceConfiguration, TextureFormat,
    TextureUsages,
};
use winit::window::Window;

pub async fn configure_wgpu(window: &Window) -> (Device, Queue, Surface<'_>, TextureFormat) {
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

    let swapchain_format = TextureFormat::Bgra8UnormSrgb;

    let mut config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: PresentMode::Fifo,
        alpha_mode: CompositeAlphaMode::Opaque,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    viewport.configure(&device, &config);

    (device, queue, viewport, swapchain_format)
}

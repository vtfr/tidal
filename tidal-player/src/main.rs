use std::ffi::c_void;
use std::{cell::RefCell, collections::HashMap, f32::consts::PI, mem};

use uuid::uuid;
use wgpu::{Device, Queue, TextureFormat, TextureView, TextureViewDescriptor};
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;
use winit::window::WindowBuilder;

use tidal_core::default_program;
use tidal_core::renderer::wgpu::context_window::WindowContextHolder;
use tidal_core::renderer::wgpu::renderer::WgpuRenderer;
use tidal_core::renderer::wgpu::{Context, Surface};
use tidal_core::renderer::{RendererError, Vertex};
use tidal_core::vm::instruction::Instruction;
use tidal_core::vm::program::{Program, Resources};
use tidal_core::vm::Interpreter;

use crate::renderer::wgpu::{Context, ContextHolder, Screen};

#[derive(Debug)]
pub struct WindowContext {
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

#[derive(Debug)]
pub struct WindowContextSurfaceTexture {
    surface: wgpu::SurfaceTexture,
    view: wgpu::TextureView,
}

impl Surface for WindowContextSurfaceTexture {
    #[inline(always)]
    fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    #[inline(always)]
    fn present(self) {
        self.surface.present()
    }
}

impl Context for WindowContext {
    type Surface = WindowContextSurfaceTexture;

    #[inline(always)]
    fn device(&self) -> &Device {
        &self.device
    }

    #[inline(always)]
    fn queue(&self) -> &Queue {
        &self.queue
    }

    #[inline(always)]
    fn surface_format(&self) -> TextureFormat {
        self.surface_config.format
    }

    #[inline(always)]
    fn prepare_surface(&self) -> Self::Surface {
        let surface = self
            .surface
            .get_current_texture()
            .expect("unexpected surface error during WindowContext::prepare_screen");

        let view = surface
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        WindowContextSurfaceTexture { surface, view }
    }
}

impl WindowContext {
    pub async fn new(window: &Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("default device"),
                    features: wgpu::Features::empty(),
                    limits: Default::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .filter(|f| f.is_srgb())
            .next()
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let size = window.inner_size();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &surface_config);

        Self {
            surface,
            surface_config,
            device,
            queue,
        }
    }
}

fn main() {
    pollster::block_on(run()).unwrap();
}

async fn run() -> Result<(), RendererError> {
    env_logger::init();

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Tidal")
        .build(&event_loop)
        .unwrap();

    let context = WindowContextHolder::new(&window).await;
    let mut renderer = WgpuRenderer::new(context);

    let program = default_program();

    let mut interpreter = Interpreter::new(program);
    interpreter.run_init(&mut renderer);

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(_) => {
            interpreter.run_frame(&mut renderer);
        }
        Event::WindowEvent { ref event, .. } => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::ExitWithCode(0),
            _ => {}
        },
        // Event::MainEventsCleared => renderer.context.window.request_redraw(),
        _ => {}
    });
}

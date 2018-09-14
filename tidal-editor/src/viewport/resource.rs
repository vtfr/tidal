use std::rc::Rc;
use std::sync::{Arc, Mutex};

use eframe::egui_wgpu::RenderState;
use eframe::wgpu;
use eframe::wgpu::util::DeviceExt;

use tidal_core::renderer::{Context, Renderer};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex(pub [[f32; 2]; 2]);

const FULL_SCREEN_VERTICES: [Vertex; 6] = [
    // Triangle 1
    Vertex([[-1.0, 1.0], [0.0, 0.0]]),
    Vertex([[-1.0, -1.], [0.0, 1.0]]),
    Vertex([[1.0, -1.0], [1.0, 1.0]]),
    // Triangle 2
    Vertex([[-1.0, 1.0], [0.0, 0.0]]),
    Vertex([[1.0, -1.0], [1.0, 1.0]]),
    Vertex([[1.0, 1.0], [1.0, 0.0]]),
];

pub(crate) struct EguiRendererContext {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub surface_format: wgpu::TextureFormat,
}

impl EguiRendererContext {
    pub fn new(render_state: RenderState, surface_format: wgpu::TextureFormat) -> Self {
        let device = render_state.device;
        let queue = render_state.queue;

        Self {
            device,
            queue,
            surface_format,
        }
    }
}

impl Context for EguiRendererContext {
    fn device(&self) -> &wgpu::Device {
        &*self.device
    }

    fn queue(&self) -> &wgpu::Queue {
        &*self.queue
    }

    fn surface_format(&self) -> wgpu::TextureFormat {
        self.surface_format
    }
}

pub(crate) struct ViewportCallbackResource {
    pub renderer: Arc<Mutex<Renderer>>,
    pub bind_group: wgpu::BindGroup,
    pub pipeline: wgpu::RenderPipeline,
    pub full_screen_quad_vertex_buffer: wgpu::Buffer,
    pub texture_view: wgpu::TextureView,
    pub texture: wgpu::Texture,
}

impl ViewportCallbackResource {
    pub fn new(render_state: &RenderState) -> Self {
        let device = &render_state.device;

        // Prepare the texture we'll be using for rendering
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: 1920,
                height: 1080,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: render_state.target_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        // Create a view for this texture
        let texture_view = texture.create_view(&Default::default());

        // Prepare a sampler for this texture
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

        // Prepare texture
        let shader = device.create_shader_module(wgpu::include_wgsl!("resource.wgsl"));

        let full_screen_quad_vertex_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&FULL_SCREEN_VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });

        // Prepare bind group and bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: core::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![
                        0 => Float32x2,
                        1 => Float32x2,
                    ],
                }],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(render_state.target_format.into())],
            }),
            multiview: None,
        });

        let context = EguiRendererContext::new(render_state.clone(), render_state.target_format);
        let context = Box::new(context) as Box<dyn Context + Sync + Send>;
        let renderer = Renderer::new(context);

        Self {
            pipeline,
            bind_group,
            full_screen_quad_vertex_buffer,
            texture,
            texture_view,
            renderer: Arc::new(Mutex::new(renderer)),
        }
    }
}

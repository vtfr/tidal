use wgpu::{include_wgsl, RenderPassDescriptor};

use crate::renderer::{CameraUniform, CommandList, SamplerCache, Vertex};

pub struct RenderPassContext<'a> {
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,

    pub sampler_cache: &'a mut SamplerCache,

    pub camera_uniform: &'a CameraUniform,

    pub surface_format: wgpu::TextureFormat,
}

pub trait RenderPass {
    type Attributes<'a>;

    fn render(
        &mut self,
        command_list: &CommandList,
        context: RenderPassContext,
        attributes: Self::Attributes<'_>,
    );
}

pub trait RenderPassFactory {
    type Output;

    fn new(context: RenderPassContext) -> Self::Output;
}

#[derive(Debug)]
pub struct ScreenRenderPass {
    pipeline_layout: wgpu::PipelineLayout,
    shader_module: wgpu::ShaderModule,
    pipeline: wgpu::RenderPipeline,
}

impl RenderPassFactory for ScreenRenderPass {
    type Output = ScreenRenderPass;

    fn new(context: RenderPassContext) -> Self {
        let shader_module = context
            .device
            .create_shader_module(include_wgsl!("screen_render_pass.wgsl"));

        let pipeline_layout =
            context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("[ScreenRenderPass] pipeline layout"),
                    bind_group_layouts: &[context.camera_uniform.bind_group_layout()],
                    push_constant_ranges: &[],
                });

        let pipeline = context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader_module,
                    entry_point: "vs_main",
                    buffers: &[Vertex::buffer_layout()],
                },
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &shader_module,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: context.surface_format,
                        blend: None,
                        write_mask: Default::default(),
                    })],
                }),
                multiview: None,
            });

        Self {
            shader_module,
            pipeline_layout,
            pipeline,
        }
    }
}

impl RenderPass for ScreenRenderPass {
    type Attributes<'a> = (&'a wgpu::TextureView);

    fn render(
        &mut self,
        command_list: &CommandList,
        context: RenderPassContext,
        (screen_texture_view): Self::Attributes<'_>,
    ) {
        context
            .camera_uniform
            .set(context.queue, &command_list.camera);

        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("x") });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ScreenRenderPass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: screen_texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, context.camera_uniform.bind_group(), &[]);

            for object in command_list.objects.iter() {
                render_pass.set_vertex_buffer(0, object.mesh.buffer().slice(..));
                render_pass.draw(0..object.mesh.num_vertices(), 0..1);
            }
        }

        context.queue.submit(std::iter::once(encoder.finish()));
    }
}

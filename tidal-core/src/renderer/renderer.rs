use std::borrow::Cow;

use wgpu::TextureFormat;

use crate::renderer::{
    CameraUniform, CommandList, Mesh, MeshDescriptor, RenderPass, RenderPassContext,
    RenderPassFactory, SamplerCache, SamplerDescriptor, Shader, ShaderModuleDescriptor, Texture,
    TextureDescriptor,
};

pub trait Context {
    fn device(&self) -> &wgpu::Device;
    fn queue(&self) -> &wgpu::Queue;
    fn surface_format(&self) -> wgpu::TextureFormat;
}

pub struct Renderer {
    context: Box<dyn Context + Send + Sync>,
    sampler_cache: SamplerCache,
    camera_uniform: CameraUniform,
}

impl Renderer {
    #[inline]
    pub fn new(context: Box<dyn Context + Send + Sync>) -> Self {
        let device = context.device();
        let camera_uniform = CameraUniform::new(device);

        Self {
            context,
            sampler_cache: Default::default(),
            camera_uniform,
        }
    }

    #[inline]
    pub fn samplers(&mut self) -> &mut SamplerCache {
        &mut self.sampler_cache
    }

    pub fn create_mesh(&mut self, desc: &MeshDescriptor) -> Mesh {
        let device = self.context.device();

        Mesh::new(device, desc)
    }

    pub fn create_texture(&mut self, desc: &TextureDescriptor) -> Texture {
        let device = self.context.device();

        Texture::new(device, desc)
    }

    pub fn create_shader_module(&mut self, desc: &ShaderModuleDescriptor) -> Shader {
        let device = self.context.device();

        Shader::new(device, Cow::Borrowed(desc.source))
    }

    #[deprecated]
    pub fn create_render_pass<R: RenderPassFactory>(&mut self) -> R::Output {
        R::new(self.render_pass_context())
    }

    #[deprecated]
    pub fn render<R: RenderPass>(
        &mut self,
        render_pass: &mut R,
        command_list: &CommandList,
        attributes: R::Attributes<'_>,
    ) {
        render_pass.render(command_list, self.render_pass_context(), attributes);
    }

    #[inline(always)]
    fn render_pass_context(&mut self) -> RenderPassContext {
        RenderPassContext {
            device: self.context.device(),
            queue: self.context.queue(),
            sampler_cache: &mut self.sampler_cache,
            camera_uniform: &self.camera_uniform,
            surface_format: self.context.surface_format(),
        }
    }
}

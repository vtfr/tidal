use std::borrow::Cow;

#[derive(Debug)]
pub struct Shader {
    inner: wgpu::ShaderModule,
}

#[derive(Debug, Clone)]
pub struct ShaderModuleDescriptor<'a> {
    pub source: &'a str,
}

impl Shader {
    pub fn new(device: &wgpu::Device, source: Cow<str>) -> Self {
        let inner = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(source),
        });

        Self { inner }
    }

    #[inline]
    pub(crate) fn wgpu_shader_module(&self) -> &wgpu::ShaderModule {
        &self.inner
    }

    #[inline]
    pub(crate) fn vert_entry_point(&self) -> &str {
        "vs_main"
    }

    #[inline]
    pub(crate) fn frag_entry_point(&self) -> &str {
        "fs_main"
    }
}

use derive_more::Display;

#[derive(Debug)]
pub struct Texture {
    inner: wgpu::Texture,
    view: wgpu::TextureView,
}

#[derive(Display, Debug, Copy, Clone)]
pub enum TextureFormat {
    RGBA16F,
    RGBA8U,
    SRGBA8U,
}

#[derive(Debug)]
pub struct TextureDescriptor<'a> {
    pub format: wgpu::TextureFormat,
    pub dimensions: cgmath::Vector2<u32>,
    pub data: Option<&'a [u8]>,
}

impl Texture {
    pub fn new(device: &wgpu::Device, desc: &TextureDescriptor) -> Self {
        let inner = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: desc.dimensions.x,
                height: desc.dimensions.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: desc.format,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = inner.create_view(&wgpu::TextureViewDescriptor::default());

        Texture { inner, view }
    }

    #[inline]
    pub(crate) fn wgpu_texture(&self) -> &wgpu::Texture {
        &self.inner
    }

    #[inline]
    pub(crate) fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    #[inline]
    pub(crate) fn format(&self) -> wgpu::TextureFormat {
        self.inner.format()
    }
}

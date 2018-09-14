use wgpu::util::DeviceExt;

use crate::renderer::buffers::ConstantBuffer;
use crate::renderer::Camera;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraBufferContents([[f32; 4]; 4]);

#[derive(Debug)]
pub struct CameraUniform {
    buffer: ConstantBuffer<CameraBufferContents>,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

impl CameraUniform {
    pub fn new(device: &wgpu::Device) -> Self {
        let matrix = Camera::default().to_view_projection_matrix();
        let contents = CameraBufferContents(matrix.into());

        let buffer = ConstantBuffer::new(device, &contents);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("[CameraUniform] bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("[CameraUniform] bind_group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(buffer.buffer().as_entire_buffer_binding()),
            }],
        });

        Self {
            buffer,
            bind_group,
            bind_group_layout,
        }
    }

    pub fn set(&self, queue: &wgpu::Queue, camera: &Camera) {
        let matrix = camera.to_view_projection_matrix();
        let contents = CameraBufferContents(matrix.into());

        self.buffer.update(queue, &contents);
    }

    #[inline]
    pub(crate) fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    #[inline]
    pub(crate) fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}

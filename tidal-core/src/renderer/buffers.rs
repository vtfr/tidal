use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use cgmath::Vector4;
use wgpu::util::DeviceExt;
use wgpu::{PipelineLayout, PipelineLayoutDescriptor, RenderPipelineDescriptor, VertexState};

use crate::renderer::{CommandList, Context, Object, RenderPass, Texture};

/// Constant Buffer is a buffer that contain render-constants.
/// It only holds one value.
#[derive(Debug)]
pub struct ConstantBuffer<T> {
    buffer: wgpu::Buffer,
    _phantom: PhantomData<T>,
}

impl<T> ConstantBuffer<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn new(device: &wgpu::Device, value: &T) -> Self {
        let contents = bytemuck::bytes_of(value);

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            buffer,
            _phantom: Default::default(),
        }
    }

    pub fn update(&self, queue: &wgpu::Queue, value: &T) {
        let contents = bytemuck::bytes_of(value);

        queue.write_buffer(&self.buffer, 0, contents);
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }
}

pub type ConstantFloatBuffer = ConstantBuffer<f32>;

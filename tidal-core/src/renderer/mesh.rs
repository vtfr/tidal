use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub positions: [f32; 3],
    pub normals: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
    #[inline]
    pub fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        static ATTR_ARRAY: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
            0 => Float32x3,
            1 => Float32x3,
            2 => Float32x2
        ];

        wgpu::VertexBufferLayout {
            array_stride: core::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTR_ARRAY,
        }
    }
}

#[derive(Debug)]
pub struct Mesh {
    buffer: wgpu::Buffer,
    num_vertices: u32,
}

#[derive(Debug)]
pub struct MeshDescriptor<'a> {
    pub vertices: &'a [Vertex],
}

impl Mesh {
    pub fn new(device: &wgpu::Device, desc: &MeshDescriptor) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(desc.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Mesh {
            buffer,
            num_vertices: desc.vertices.len() as u32,
        }
    }

    #[inline]
    pub(crate) fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    #[inline]
    pub(crate) fn num_vertices(&self) -> u32 {
        self.num_vertices
    }
}

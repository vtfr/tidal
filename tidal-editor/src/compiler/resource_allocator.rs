use tidal_core::renderer::Vertex;

#[derive(Default)]
pub struct ResourceAllocator {
    pub next_allocation: usize,
    pub resources: Vec<Box<[u8]>>,
}

impl ResourceAllocator {
    pub fn add_mesh(&mut self, vertices: &[Vertex]) -> usize {
        let allocation = self.next_allocation;

        let slice: &[u8] = bytemuck::cast_slice(vertices);

        self.resources.push(slice.into());
        self.next_allocation += 1;
        allocation
    }

    pub fn add_shader(&mut self, source: &str) -> usize {
        let allocation = self.next_allocation;

        self.resources.push(source.as_bytes().into());
        self.next_allocation += 1;
        allocation
    }
}

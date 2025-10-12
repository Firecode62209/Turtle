use ash::vk as avk;
use gpu_allocator::MemoryLocation;
use crate::{tvk, AnyResult};

pub struct Mesh<V> where V: Copy, V: tvk::VertexDescription  {
    pub vertices: Vec<V>,
    pub indices: Vec<u32>,
    pub vertex_buffer: tvk::Buffer,
    pub index_buffer: tvk::Buffer
}

impl<V> Mesh<V> where V: Copy, V: tvk::VertexDescription {
    pub fn from_vertices(context: &tvk::Context, vertices: Vec<V>, indices: Vec<u32>) -> AnyResult<Self> {
        let mut vertex_buffer = context.create_buffer(
            avk::BufferUsageFlags::VERTEX_BUFFER,
            MemoryLocation::CpuToGpu,
            size_of_val(vertices.as_slice()) as u64
        )?;
        vertex_buffer.copy_memory(&vertices)?;
        let mut index_buffer = context.create_buffer(
            avk::BufferUsageFlags::INDEX_BUFFER,
            MemoryLocation::CpuToGpu,
            size_of_val(indices.as_slice()) as u64
        )?;
        index_buffer.copy_memory(&indices)?;

        Ok(Self {
            vertices: vertices,
            indices: indices,
            vertex_buffer,
            index_buffer
        })
    }
}

impl tvk::Context {
    pub fn create_mesh_from_vertices<V>(&self, vertices: Vec<V>, indices: Vec<u32>) -> AnyResult<Mesh<V>>
    where V: Copy, V: tvk::VertexDescription {
        Mesh::from_vertices(self, vertices, indices)
    }
}
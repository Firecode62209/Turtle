use ash::vk as avk;
use glam::vec3;
use gpu_allocator::MemoryLocation;
use crate::{tvk::{self, Vertex}, AnyResult};

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

impl tvk::Context {
    pub fn create_mesh_from_cube(&self) -> AnyResult<Mesh<Vertex>> {
        Mesh::from_vertices(&self, CUBE_VERTICES.into(), CUBE_INDICES.into())
    }
}

pub const CUBE_VERTICES: [Vertex; 24] = [
    // Front face (Z+)
    Vertex { position: vec3(-0.5, -0.5,  0.5) },
    Vertex { position: vec3( 0.5, -0.5,  0.5) },
    Vertex { position: vec3( 0.5,  0.5,  0.5) },
    Vertex { position: vec3(-0.5,  0.5,  0.5) },

    // Back face (Z-)
    Vertex { position: vec3( 0.5, -0.5, -0.5) },
    Vertex { position: vec3(-0.5, -0.5, -0.5) },
    Vertex { position: vec3(-0.5,  0.5, -0.5) },
    Vertex { position: vec3( 0.5,  0.5, -0.5) },

    // Left face (X-)
    Vertex { position: vec3(-0.5, -0.5, -0.5) },
    Vertex { position: vec3(-0.5, -0.5,  0.5) },
    Vertex { position: vec3(-0.5,  0.5,  0.5) },
    Vertex { position: vec3(-0.5,  0.5, -0.5) },

    // Right face (X+)
    Vertex { position: vec3( 0.5, -0.5,  0.5) },
    Vertex { position: vec3( 0.5, -0.5, -0.5) },
    Vertex { position: vec3( 0.5,  0.5, -0.5) },
    Vertex { position: vec3( 0.5,  0.5,  0.5) },

    // Top face (Y+)
    Vertex { position: vec3(-0.5,  0.5,  0.5) },
    Vertex { position: vec3( 0.5,  0.5,  0.5) },
    Vertex { position: vec3( 0.5,  0.5, -0.5) },
    Vertex { position: vec3(-0.5,  0.5, -0.5) },

    // Bottom face (Y-)
    Vertex { position: vec3(-0.5, -0.5, -0.5) },
    Vertex { position: vec3( 0.5, -0.5, -0.5) },
    Vertex { position: vec3( 0.5, -0.5,  0.5) },
    Vertex { position: vec3(-0.5, -0.5,  0.5) },
];

pub const CUBE_INDICES: [u32; 36] = [
    // Front
    0, 1, 2,  2, 3, 0,
    // Back
    4, 5, 6,  6, 7, 4,
    // Left
    8, 9,10, 10,11, 8,
    // Right
    12,13,14, 14,15,12,
    // Top
    16,17,18, 18,19,16,
    // Bottom
    20,21,22, 22,23,20,
];
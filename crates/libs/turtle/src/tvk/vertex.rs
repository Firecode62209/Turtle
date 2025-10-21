use ash::vk as avk;
use glam::{vec3, Mat4, Vec3, Vec4};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: glam::Vec3,
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

impl VertexDescription for Vertex {
    fn get_binding_descriptions() -> Vec<avk::VertexInputBindingDescription> {
        vec![
            avk::VertexInputBindingDescription::default()
                .binding(0)
                .stride(std::mem::size_of::<Vertex>() as u32)
                .input_rate(avk::VertexInputRate::VERTEX)
        ]
    }

    fn get_attribute_descriptions() -> Vec<avk::VertexInputAttributeDescription> {
        vec![
            avk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(0)
                .format(avk::Format::R32G32B32_SFLOAT)
                .offset(0),
        ]
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug)]
pub struct InstanceData {
    pub model: Mat4,
    pub color: Vec3,
}

impl VertexDescription for InstanceData {
    fn get_attribute_descriptions() -> Vec<avk::VertexInputAttributeDescription> {
        let mut vec = (0..4).map(|i| avk::VertexInputAttributeDescription {
            binding: 1,
            location: 1 + i,
            format: avk::Format::R32G32B32A32_SFLOAT,
            offset: size_of::<Vec4>() as u32 * i,
        })
        .collect::<Vec<_>>();
        vec.push(avk::VertexInputAttributeDescription {
            binding: 1,
            location: 5, 
            format: avk::Format::R32G32B32A32_SFLOAT,
            offset: std::mem::size_of::<Mat4>() as u32,
        });
        vec
    }

    fn get_binding_descriptions() -> Vec<avk::VertexInputBindingDescription> {
        vec![avk::VertexInputBindingDescription {
            binding: 1,
            stride: std::mem::size_of::<InstanceData>() as u32,
            input_rate: avk::VertexInputRate::INSTANCE,
        }]
    }
}

pub trait VertexDescription {
    fn get_binding_descriptions() -> Vec<avk::VertexInputBindingDescription>;
    fn get_attribute_descriptions() -> Vec<avk::VertexInputAttributeDescription>;
}
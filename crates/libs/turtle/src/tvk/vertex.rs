use ash::vk as avk;
use glam::{vec3, Mat4};

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

#[derive(Clone, Copy)]
pub struct UniformBufferObject {
    pub model: Mat4,
    pub view: Mat4,
    pub proj: Mat4
}

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

pub trait VertexDescription {
    fn get_binding_descriptions() -> Vec<avk::VertexInputBindingDescription>;
    fn get_attribute_descriptions() -> Vec<avk::VertexInputAttributeDescription>;
}
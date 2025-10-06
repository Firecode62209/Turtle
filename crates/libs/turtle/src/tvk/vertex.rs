use ash::vk as avk;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: glam::Vec3,
}

pub const VERTICES: [Vertex; 3] = [
    Vertex { position: glam::vec3(0.0, -0.5, 0.0) },
    Vertex { position: glam::vec3(0.5, 0.5, 0.0) },
    Vertex { position: glam::vec3(-0.5, 0.5, 0.0) },
];

impl Vertex {
    pub fn get_binding_descriptions() -> Vec<avk::VertexInputBindingDescription> {
        vec![
            avk::VertexInputBindingDescription::default()
                .binding(0)
                .stride(std::mem::size_of::<Vertex>() as u32)
                .input_rate(avk::VertexInputRate::VERTEX)
        ]
    }

    pub fn get_attribute_descriptions() -> Vec<avk::VertexInputAttributeDescription> {
        vec![
            avk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(0)
                .format(avk::Format::R32G32B32_SFLOAT)
                .offset(0),
        ]
    }
}
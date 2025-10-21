use ash::vk as avk;
use gpu_allocator::MemoryLocation;

use crate::*;

pub struct InstanceGroup {
    pub mesh: Mesh<tvk::Vertex>,
    pub all_instances: Vec<tvk::InstanceData>,
    pub visible_indices: Vec<usize>,
    pub instance_buffer: Option<tvk::Buffer>,
    pub visible_count: usize,
}

impl From<Mesh<tvk::Vertex>> for InstanceGroup  {
    fn from(value: Mesh<tvk::Vertex>) -> Self {
        Self {
            mesh: value,
            all_instances: Vec::new(),
            visible_indices: Vec::new(),
            instance_buffer: None,
            visible_count: 0
        }
    }
}

impl InstanceGroup {
    pub fn add_instance(&mut self, data: tvk::InstanceData, visible: bool) {
        self.all_instances.push(data);
        if visible {
            self.visible_indices.push(self.all_instances.len() - 1);
        }
        self.visible_count = self.visible_indices.len();
    }

    pub fn create_instance_buffer(&mut self, context: &tvk::Context) -> AnyResult<()> {
        self.instance_buffer = Some(context.create_buffer(
            avk::BufferUsageFlags::VERTEX_BUFFER,
            MemoryLocation::CpuToGpu,
            1
        )?);

        Ok(())
    }

    pub fn update_gpu_buffer(&mut self) -> AnyResult<()> {
        let visible_data: Vec<tvk::InstanceData> = self
            .visible_indices
            .iter()
            .map(|&i| self.all_instances[i])
            .collect();
        if let Some(instance_buffer) = &mut self.instance_buffer {
            instance_buffer.copy_memory(&self.all_instances)?;
        }
        self.visible_count = visible_data.len();
        Ok(())
    }

    pub fn set_visible(&mut self, instance_index: usize, visible: bool) {
        if visible {
            if !self.visible_indices.contains(&instance_index) {
                self.visible_indices.push(instance_index);
            }
        } else {
            self.visible_indices.retain(|&i| i != instance_index);
        }
    }
}
use ash::vk as avk;
use crate::{tvk, AnyResult};
use std::sync::Arc;

pub struct Descriptor {
    pub sets: Vec<avk::DescriptorSet>,
    pub pool: avk::DescriptorPool,
    pub layout: avk::DescriptorSetLayout,
    count: u32,
    logical_device: Arc<tvk::LogicalDevice>
}

impl Descriptor {
    pub fn new(logical_device: Arc<tvk::LogicalDevice>, count: u32) -> AnyResult<Self> {
        let layout_bindings = [avk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(avk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(avk::ShaderStageFlags::VERTEX)];
        let layout_create_info = avk::DescriptorSetLayoutCreateInfo::default()
            .bindings(&layout_bindings);

        let layout = unsafe { logical_device.inner.create_descriptor_set_layout(&layout_create_info, None)? };

        let pool_sizes = [avk::DescriptorPoolSize::default()
            .ty(avk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(count)];

        let pool_create_info = avk::DescriptorPoolCreateInfo::default()
            .pool_sizes(&pool_sizes)
            .max_sets(count);

        let pool = unsafe { logical_device.inner.create_descriptor_pool(&pool_create_info, None)? };

        Ok(Self {
            layout,
            pool,
            sets: Vec::new(),
            count,
            logical_device
        })
    }

    pub fn allocate_sets(&mut self) ->AnyResult<()> {
        let layouts = vec![self.layout; self.count as usize];

        let allocate_info = avk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.pool)
            .set_layouts(&layouts);
        
        self.sets = unsafe { self.logical_device.inner.allocate_descriptor_sets(&allocate_info)? };

        Ok(())
    }

    pub fn update(&self, buffers: &Vec<tvk::Buffer>) -> AnyResult<()> {
        self.sets.iter().zip(buffers.iter()).for_each(|(&set, buffer)| {
            let buffer_info = [avk::DescriptorBufferInfo::default()
                .buffer(buffer.inner)
                .offset(0)
                .range(avk::WHOLE_SIZE)];

            let writes = [avk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(0)
                .dst_array_element(0)
                .descriptor_type(avk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(1)
                .buffer_info(&buffer_info)];

            unsafe { self.logical_device.inner.update_descriptor_sets(&writes, &[]);}
        });

        Ok(())
    }
}

impl tvk::Context {
    pub fn create_descriptor_dependecies(&self, count: u32) -> AnyResult<Descriptor> {
        Descriptor::new(self.logical_device.clone(), count)
    }
}

impl Drop for Descriptor {
    fn drop(&mut self) {
        unsafe {
            self.logical_device.inner.destroy_descriptor_pool(self.pool, None);
            self.logical_device.inner.destroy_descriptor_set_layout(self.layout, None);
        }
    }
}
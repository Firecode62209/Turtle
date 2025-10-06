use ash::vk as avk;
use std::sync::Arc;
use crate::{tvk, AnyResult};

pub struct CommandPool {
    pub(crate) inner: avk::CommandPool,
    logical_device: Arc<tvk::LogicalDevice>,
}

impl CommandPool {
    pub fn new(
        logical_device: Arc<tvk::LogicalDevice>,
        queue_family_index: u32
    ) -> AnyResult<Self> {
        let create_info = avk::CommandPoolCreateInfo::default()
            .queue_family_index(queue_family_index)
            .flags(avk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        
        let inner = unsafe { logical_device.inner.create_command_pool(&create_info, None)? };

        Ok(Self {
            inner,
            logical_device
        })
    }
}

impl Drop for CommandPool {
    fn drop(&mut self) {
        unsafe { self.logical_device.inner.destroy_command_pool(self.inner, None); }
    }
}
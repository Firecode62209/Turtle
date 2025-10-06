use std::sync::Arc;

use ash::vk as avk;
use crate::{tvk, AnyResult};

pub struct Queue {
    pub(crate) inner: avk::Queue,
    logical_device: Arc<tvk::LogicalDevice>,
}

impl Queue {
    pub fn new(
        index: u32,
        logical_device: Arc<tvk::LogicalDevice>
    ) -> Self {
        let inner = unsafe { logical_device.inner.get_device_queue(index, 0) };

        Self {
            inner,
            logical_device
        }
    }

    pub fn submit(
        &self,
        submit_infos: &[avk::SubmitInfo],
        fence: avk::Fence,
    ) -> AnyResult<()> {
        unsafe {
            self.logical_device.inner.queue_submit(
                self.inner,
                submit_infos,
                fence
            )?;
        }

        Ok(())
    }
}
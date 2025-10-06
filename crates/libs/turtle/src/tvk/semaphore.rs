use ash::vk as avk;
use std::sync::Arc;
use crate::{tvk, AnyResult};

pub struct Semaphore {
    pub(crate) inner: avk::Semaphore,
    logical_device: Arc<tvk::LogicalDevice>,
}

impl Semaphore {
    pub fn new(logical_device: Arc<tvk::LogicalDevice>) -> AnyResult<Self> {
        let create_info = avk::SemaphoreCreateInfo::default();
        let inner = unsafe { logical_device.inner.create_semaphore(&create_info, None)? };

        Ok(Self {
            inner,
            logical_device
        })
    }
}

impl tvk::Context {
    pub fn create_semaphore(&self) -> AnyResult<Semaphore> {
        Semaphore::new(Arc::clone(&self.logical_device))
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe { self.logical_device.inner.destroy_semaphore(self.inner, None); }
    }
}
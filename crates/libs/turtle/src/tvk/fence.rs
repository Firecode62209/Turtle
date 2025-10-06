use ash::vk as avk;
use std::sync::Arc;
use crate::{tvk, AnyResult};

pub struct Fence {
    pub(crate) inner: avk::Fence,
    logical_device: Arc<tvk::LogicalDevice>,
}

impl Fence {
    pub fn new(logical_device: Arc<tvk::LogicalDevice>, signaled: bool) -> AnyResult<Self> {
        let create_info = avk::FenceCreateInfo::default().flags(
            if signaled {
                avk::FenceCreateFlags::SIGNALED
            } else {
                avk::FenceCreateFlags::empty()
            }
        );
        let inner = unsafe { logical_device.inner.create_fence(&create_info, None)? };

        Ok(Self {
            inner,
            logical_device
        })
    }

    pub fn wait(&self, timeout: u64) -> AnyResult<()> {
        unsafe {
            self.logical_device.inner.wait_for_fences(&[self.inner], true, timeout)?;
        }
        Ok(())
    }

    pub fn reset(&self) -> AnyResult<()> {
        unsafe {
            self.logical_device.inner.reset_fences(&[self.inner])?;
        }
        Ok(())
    }
}

impl tvk::Context {
    pub fn create_fence(&self, signaled: bool) -> AnyResult<Fence> {
        Fence::new(Arc::clone(&self.logical_device), signaled)
    }
}

impl Drop for Fence {
    fn drop(&mut self) {
        unsafe { self.logical_device.inner.destroy_fence(self.inner, None); }
    }
}
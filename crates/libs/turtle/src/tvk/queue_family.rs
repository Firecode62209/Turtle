use ash::vk as avk;

use crate::{tvk, AnyResult};


#[derive(Copy, Debug, Clone)]
pub struct QueueFamily {
    supports_present: bool,
    pub index: u32,
    pub inner: avk::QueueFamilyProperties,
}

impl QueueFamily {
    pub fn new(
        physical_device: &avk::PhysicalDevice,
        surface: &tvk::Surface,
        index: u32,
        inner: avk::QueueFamilyProperties
    ) -> AnyResult<Self> {
        let supports_present = unsafe {
            surface.inner.get_physical_device_surface_support(
                *physical_device,
                index,
                surface.surface_khr
            )?
        };

        Ok(Self {
            supports_present,
            index,
            inner
        })
    }

    pub fn supports_graphics(&self) -> bool {
        self.inner.queue_flags.contains(avk::QueueFlags::GRAPHICS)
    }

    pub fn supports_transfer(&self) -> bool {
        self.inner.queue_flags.contains(avk::QueueFlags::TRANSFER)
    }

    pub fn supports_present(&self) -> bool {
        self.supports_present
    }

    pub fn has_queues(&self) -> bool {
        self.inner.queue_count > 0
    }
}
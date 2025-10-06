use gpu_allocator::vulkan as gvk;
use crate::{tvk, AnyResult};

#[derive(Debug)]
pub struct Allocator {
    pub(crate) inner: gvk::Allocator
}

impl Allocator {
    pub(crate) fn new(
        instance: &tvk::Instance,
        logical_device: &tvk::LogicalDevice,
        physical_device: &tvk::PhysicalDevice
    ) -> AnyResult<Self> {
        let desc = gvk::AllocatorCreateDesc {
            instance: instance.inner.clone(),
            device: logical_device.inner.clone(),
            physical_device: physical_device.inner,
            debug_settings: Default::default(),
            buffer_device_address: false,
            allocation_sizes: Default::default()
        };

        let inner = gvk::Allocator::new(&desc)?;

        Ok(Self {
            inner
        })
    }
}

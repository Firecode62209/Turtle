use std::sync::{Arc, Mutex};

use ash::vk as avk;
use crate::*;
use gpu_allocator::{vulkan as mvk, MemoryLocation};

pub struct Image {
    pub(crate) inner: avk::Image,
    allocation: Option<mvk::Allocation>,
    allocator: Arc<Mutex<tvk::Allocator>>,
    logical_device: Arc<tvk::LogicalDevice>
}

impl Image {
    pub fn new(
        logical_device: Arc<tvk::LogicalDevice>,
        allocator: Arc<Mutex<tvk::Allocator>>,
        extent: avk::Extent2D,
        format: avk::Format,
        usage: avk::ImageUsageFlags
    ) -> AnyResult<Self> {
        let image_info = avk::ImageCreateInfo::default()
            .image_type(avk::ImageType::TYPE_2D)
            .format(format)
            .tiling(avk::ImageTiling::OPTIMAL)
            .initial_layout(avk::ImageLayout::UNDEFINED)
            .usage(usage)
            .sharing_mode(avk::SharingMode::EXCLUSIVE)
            .samples(avk::SampleCountFlags::TYPE_1)
            .extent(avk::Extent3D {
                width: extent.width,
                height: extent.height,
                depth: 1
            })
            .mip_levels(1)
            .array_layers(1);

        let inner = unsafe {
            logical_device.inner.create_image(&image_info, None)?
        };

        let requirements = unsafe {
            logical_device.inner.get_image_memory_requirements(inner)
        };

        let alloc_desc = mvk::AllocationCreateDesc {
            name: "Image",
            requirements,
            location: MemoryLocation::GpuOnly,
            linear: false,
            allocation_scheme: mvk::AllocationScheme::GpuAllocatorManaged,
        };

        let allocation = allocator.lock().unwrap().inner.allocate(&alloc_desc)?;
        
        unsafe { logical_device.inner.bind_image_memory(inner, allocation.memory(), allocation.offset())? };

        Ok(Self {
            inner,
            logical_device,
            allocation: Some(allocation),
            allocator
        })
    }
}

impl tvk::Context {
    pub fn create_image(&self, extent: avk::Extent2D, format: avk::Format, usage: avk::ImageUsageFlags) -> AnyResult<Image> {
        Image::new(self.logical_device.clone(), self.allocator.clone(), extent, format, usage)
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            self.logical_device.inner.destroy_image(self.inner, None);
            self.allocator.lock().unwrap().inner.free(self.allocation.take().unwrap()).unwrap();
        }
    }
}
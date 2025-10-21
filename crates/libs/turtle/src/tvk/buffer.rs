use std::{mem::{size_of_val, align_of}, sync::{Arc, Mutex}};
use ash::vk as avk;
use crate::{tvk, AnyResult};
use gpu_allocator::{vulkan as mvk, MemoryLocation};

pub struct Buffer {
    pub(crate) inner: avk::Buffer,
    allocation: Option<mvk::Allocation>,
    allocator: Arc<Mutex<tvk::Allocator>>,
    logical_device: Arc<tvk::LogicalDevice>,
    pub size: avk::DeviceSize
}

impl Buffer {
    pub fn create(
        allocator: Arc<Mutex<tvk::Allocator>>,
        logical_device: Arc<tvk::LogicalDevice>,
        size: u64,
        usage: avk::BufferUsageFlags,
        location: MemoryLocation,
    ) -> AnyResult<Self> {
        let buffer_info = avk::BufferCreateInfo::default()
            .size(size)
            .usage(usage)
            .sharing_mode(avk::SharingMode::EXCLUSIVE);    
        let inner = unsafe { logical_device.inner.create_buffer(&buffer_info, None)? };
        let requirements = unsafe { logical_device.inner.get_buffer_memory_requirements(inner)};
        let alloc_desc = mvk::AllocationCreateDesc {
            name: "Buffer",
            requirements,
            location,
            linear: true,
            allocation_scheme: mvk::AllocationScheme::GpuAllocatorManaged,
        };
        let allocation = allocator.lock().unwrap().inner.allocate(&alloc_desc)?;
        
        unsafe { logical_device.inner.bind_buffer_memory(inner, allocation.memory(), allocation.offset())? };
        Ok(Self {
            inner,
            allocation: Some(allocation),
            allocator,
            logical_device,
            size
        })
    }
    pub fn copy_memory<T: Copy>(&mut self, data: &[T]) -> AnyResult<()> {
        let required_size = size_of_val(data) as u64;

        if required_size > self.size {
            let new_size = ((required_size + 255) / 256) * 256;
            let mut new_buffer = Buffer::create(self.allocator.clone(), self.logical_device.clone(),
                new_size,
                avk::BufferUsageFlags::VERTEX_BUFFER,
                MemoryLocation::CpuToGpu
            )?;

            std::mem::swap(self, &mut new_buffer);
        }

        unsafe {
            let data_ptr = self.allocation.as_ref().unwrap().mapped_ptr().unwrap().as_ptr();
            let mut align = ash::util::Align::new(data_ptr, align_of::<T>() as _, size_of_val(data) as _);
            align.copy_from_slice(data);
        };
        Ok(())
    }

    pub fn copy_buffer(&self, context: &tvk::Context, dst_buffer: &tvk::Buffer) -> AnyResult<()> {
        let command_buffers = context.allocate_command_buffers(avk::CommandBufferLevel::PRIMARY, tvk::QueueType::Graphics, 1)?;
        let command_buffer = &command_buffers[0];

        command_buffer.begin(avk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)?;
        command_buffer.copy_buffer(self, dst_buffer);
        command_buffer.end()?;

        let command_buffers = [command_buffer.inner];

        let submits = [
            avk::SubmitInfo::default()
                .command_buffers(&command_buffers)
        ];

        context.queues.get(&tvk::QueueType::Graphics).unwrap().submit(&submits, avk::Fence::null())?;
        context.logical_device.device_wait_idle()?;
        Ok(())
    }
}

impl tvk::Context {
    pub fn create_buffer(
        &self,
        usage: avk::BufferUsageFlags,
        memory_location: MemoryLocation,
        size: avk::DeviceSize
    ) -> AnyResult<Buffer> {
        Buffer::create(self.allocator.clone(), self.logical_device.clone(), size, usage, memory_location)
    }
    
    pub fn copy_buffer(&self, src_buffer: &tvk::Buffer, dst_buffer: &tvk::Buffer) -> AnyResult<()> {
        src_buffer.copy_buffer(self, dst_buffer)
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.logical_device.inner.destroy_buffer(self.inner, None);
            self.allocator.lock().unwrap().inner.free(self.allocation.take().unwrap()).unwrap();
        }
    }
}
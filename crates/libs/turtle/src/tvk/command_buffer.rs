use std::sync::Arc;
use ash::vk as avk;
use crate::{tvk, AnyResult};

pub struct CommandBuffer {
    pub(crate) inner: avk::CommandBuffer,
    logical_device: Arc<tvk::LogicalDevice>,
    command_pool: Arc<tvk::CommandPool>,
}

impl CommandBuffer {
    pub fn allocate(
        logical_device: Arc<tvk::LogicalDevice>,
        command_pool: Arc<tvk::CommandPool>,
        level: avk::CommandBufferLevel,
        count: u32,
    ) -> AnyResult<Vec<Self>> {
        let allocate_info = avk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool.inner)
            .level(level)
            .command_buffer_count(count);
        
        let buffers = unsafe { logical_device.inner.allocate_command_buffers(&allocate_info)? };
        
        buffers.iter().map(|&inner| {
            Ok(Self {
                inner,
                logical_device: logical_device.clone(),
                command_pool: command_pool.clone(),
            })
        }).collect::<AnyResult<Vec<Self>>>()
    }

    pub fn begin(&self, flags: avk::CommandBufferUsageFlags) -> AnyResult<()> {
        let begin_info = avk::CommandBufferBeginInfo::default()
            .flags(flags);
        unsafe { self.logical_device.inner.begin_command_buffer(self.inner, &begin_info)?; }
        Ok(())
    }

     pub fn begin_render_pass(
        &self,
        swapchain: &tvk::Swapchain,
        render_pass: &tvk::RenderPass,
        frame_buffer: &tvk::FrameBuffer,
        subpass_contents: avk::SubpassContents,
        clear_values: &[avk::ClearValue],
    ) {
        let begin_info = &avk::RenderPassBeginInfo::default()
            .render_pass(render_pass.inner)
            .framebuffer(frame_buffer.inner)
            .render_area(avk::Rect2D {
                offset: avk::Offset2D { x: 0, y: 0 },
                extent: swapchain.extent,
            })
            .clear_values(&clear_values);
        unsafe {
            self.logical_device.inner.cmd_begin_render_pass(self.inner, begin_info, subpass_contents);
        }
    }

    pub fn bind_descriptor_sets(&self, layout: avk::PipelineLayout, set: avk::DescriptorSet) {
        unsafe {
            let sets = [set];
            self.logical_device.inner.cmd_bind_descriptor_sets(
                self.inner,
                avk::PipelineBindPoint::GRAPHICS,
                layout,
                0,
                &sets,
                &[]
            );
        }
    }

    pub fn bind_index_buffer(&self, buffer: &tvk::Buffer) {
        unsafe {
            self.logical_device.inner.cmd_bind_index_buffer(
                self.inner,
                buffer.inner,
                0,
                avk::IndexType::UINT32
            );
        }
    }

    pub fn bind_pipeline(&self, pipeline: &tvk::Pipeline) {
        unsafe {
            self.logical_device.inner.cmd_bind_pipeline(
                self.inner,
                avk::PipelineBindPoint::GRAPHICS,
                pipeline.inner
            );
        }
    }

    pub fn set_scissor(&self, scissor: avk::Rect2D) {
        let scissors = [scissor];
        unsafe {
            self.logical_device.inner.cmd_set_scissor(self.inner, 0, &scissors);
        }
    }

    pub fn set_viewport(&self, viewport: avk::Viewport) {
        let viewports = [viewport];
        unsafe {
            self.logical_device.inner.cmd_set_viewport(self.inner, 0, &viewports);
        }
    }

    pub fn bind_vertex_buffers(&self, buffers: &[avk::Buffer]) {
        let offsets = vec![0; buffers.len()];
        unsafe {
            self.logical_device.inner.cmd_bind_vertex_buffers(
                self.inner,
                0,
                buffers,
                &offsets
            );
        }
    }

    pub fn copy_buffer(&self, src_buffer: &tvk::Buffer, dst_buffer: &tvk::Buffer) {
        let copy_region = avk::BufferCopy::default()
            .size(src_buffer.size);
        unsafe {
            self.logical_device.inner.cmd_copy_buffer(
                self.inner,
                src_buffer.inner,
                dst_buffer.inner,
                &[copy_region]
            );
        }
    }

    pub fn draw(&self, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32) {
        unsafe {
            self.logical_device.inner.cmd_draw(
                self.inner,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance
            );
        }
    }

    pub fn draw_indexed(&self, index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32) {
        unsafe {
            self.logical_device.inner.cmd_draw_indexed(
                self.inner,
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance
            )
        }
    }

    pub fn end_render_pass(&self) {
        unsafe {
            self.logical_device.inner.cmd_end_render_pass(self.inner);
        }
    }

    pub fn end(&self) -> AnyResult<()> {
        unsafe { self.logical_device.inner.end_command_buffer(self.inner)?; }
        Ok(())
    }

    pub fn reset(&self, flags: avk::CommandBufferResetFlags) -> AnyResult<()> {
        unsafe { self.logical_device.inner.reset_command_buffer(self.inner, flags)?; }
        Ok(())
    }
}

impl tvk::Context  {
    pub fn allocate_command_buffers(&self, level: avk::CommandBufferLevel, queue_type: tvk::QueueType, count: u32) -> AnyResult<Vec<tvk::CommandBuffer>> {
        let logical_device = self.logical_device.clone();
        let command_pool = self.command_pools.get(&queue_type).expect("No command pool for the given queue type").clone();
        tvk::CommandBuffer::allocate(logical_device, command_pool, level, count)
    }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        unsafe { self.logical_device.inner.free_command_buffers(self.command_pool.inner, &[self.inner]); }
    }
}
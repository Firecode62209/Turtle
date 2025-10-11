use std::{path::PathBuf, sync::Arc};

use gpu_allocator::MemoryLocation;
use winit::window::Window;

use ash::vk as avk;
use crate::{tvk, AnyResult};

const MAX_FRAMES_IN_FLIGHT: usize = 2;

pub struct Renderer {
    pub frame_buffers: Vec<tvk::FrameBuffer>,
    pub pipeline: tvk::Pipeline,
    pub render_pass: tvk::RenderPass,
    pub command_buffers: Vec<tvk::CommandBuffer>,
    pub sync_objects: tvk::SyncObjects,
    pub vertex_buffer: tvk::Buffer,
    pub swapchain: tvk::Swapchain,
    pub context: tvk::Context,

    pub frame_index: usize,
    pub vertex_source: PathBuf,
    pub fragment_source: PathBuf,
}

impl Renderer {
    pub fn new(window: &Window) -> AnyResult<Self> {
        let context: tvk::Context = tvk::Context::new(window)?;
        let swapchain = context.create_swapchain(window)?;
        let render_pass = context.create_render_pass(&swapchain)?;
        let frame_buffers = context.create_frame_buffers(&swapchain, &render_pass)?;
        
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = manifest_dir
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
        .unwrap();
        let vertex_source = workspace_root.join("assets/generated/shaders/shader.vert.spv");
        let fragment_source = workspace_root.join("assets/generated/shaders/shader.frag.spv");

        let pipeline = context.create_pipeline(
            &swapchain,
            &render_pass, 
            &vec![
                tvk::PipelineShaderCreateInfo {
                    stage: avk::ShaderStageFlags::VERTEX,
                    path: vertex_source.as_path()
                },
                tvk::PipelineShaderCreateInfo {
                    stage: avk::ShaderStageFlags::FRAGMENT,
                    path: fragment_source.as_path()
                }
            ]
        )?;
        
        let sync_objects = context.create_sync_objects(swapchain.images.len(), MAX_FRAMES_IN_FLIGHT)?;
        let command_buffers = context.allocate_command_buffers(avk::CommandBufferLevel::PRIMARY, tvk::QueueType::Graphics, MAX_FRAMES_IN_FLIGHT as u32)?;
        let mut vertex_buffer = context.create_buffer(
            avk::BufferUsageFlags::VERTEX_BUFFER,
            MemoryLocation::CpuToGpu,
            size_of_val(&tvk::VERTICES) as u64
        )?;
        vertex_buffer.copy_memory(&tvk::VERTICES)?;

        Ok(Self {
            frame_index: 0,
            context,
            swapchain,
            render_pass,
            frame_buffers,
            pipeline,
            sync_objects,
            command_buffers,
            vertex_buffer,
            vertex_source,
            fragment_source,
        })
    }

    pub fn render(&mut self) -> AnyResult<bool> {
        let in_flight_fence = &self.sync_objects.in_flight_fences[self.frame_index];
        let image_available_semaphore = &[self.sync_objects.image_available_semaphores[self.frame_index].inner];
        let command_buffer = &[self.command_buffers[self.frame_index].inner];
        let render_finished_semaphore = &[self.sync_objects.render_finished_semaphores[self.frame_index].inner];
        
        self.context.logical_device.wait_for_fences(&[in_flight_fence.inner], true, u64::MAX)?;
        let (image_index, _) = self.swapchain.acquire_next_image(
                u64::MAX,
                image_available_semaphore[0],
                avk::Fence::null()
        )?;
        
        if let Some(weak_images_in_flight) = &self.sync_objects.images_in_flight[image_index as usize] {
            if let Some(arc_images_in_flight) = weak_images_in_flight.upgrade() {
                arc_images_in_flight.wait(u64::MAX)?;
            }
        }
        self.sync_objects.images_in_flight[image_index as usize] = Some(Arc::downgrade(in_flight_fence));

        self.command_buffers[self.frame_index].reset(avk::CommandBufferResetFlags::empty())?;
        self.record_command_buffer(&self.command_buffers[self.frame_index], image_index as usize)?;
        
        in_flight_fence.reset()?;

        let submit_info = avk::SubmitInfo::default()
            .wait_semaphores(image_available_semaphore)
            .wait_dst_stage_mask(&[avk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
            .command_buffers(command_buffer)
            .signal_semaphores(render_finished_semaphore);
        self.context.queues.get(&tvk::QueueType::Graphics).unwrap().submit(&[submit_info], in_flight_fence.inner)?;
        
        let is_suboptimal = self.swapchain.queue_present(
            self.context.queues.get(&tvk::QueueType::Graphics).unwrap(),
            image_index,
            render_finished_semaphore
        )?;
        self.frame_index = (self.frame_index + 1) % MAX_FRAMES_IN_FLIGHT;
       
        Ok(is_suboptimal)
    }

    pub fn recreate_swapchain(&mut self, window: &Window) -> AnyResult<()> {
        self.context.logical_device.device_wait_idle()?;
        self.frame_buffers.clear();
        self.swapchain.recreate(&self.context, window)?;
        self.frame_buffers = self.context.create_frame_buffers(&self.swapchain, &self.render_pass)?;
        Ok(())
    }

    pub fn record_command_buffer(
        &self,
        command_buffer: &tvk::CommandBuffer,
        image_index: usize
    ) -> AnyResult<()> {
        command_buffer.begin(avk::CommandBufferUsageFlags::default())?;
        let clear_values = [avk::ClearValue {
            color: avk::ClearColorValue { float32: [0.0, 0.0, 0.08, 1.0] },
        }]; 
        command_buffer.begin_render_pass(
            &self.swapchain, 
            &self.render_pass, 
            &self.frame_buffers[image_index], 
            avk::SubpassContents::INLINE,
            &clear_values
        );
        command_buffer.bind_pipeline(&self.pipeline);
        command_buffer.bind_vertex_buffer(&self.vertex_buffer);
        command_buffer.draw(3, 1, 0, 0);
        command_buffer.end_render_pass();
        command_buffer.end()?;
        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.context.logical_device.device_wait_idle().unwrap();
        for command_buffer in &self.command_buffers {
            command_buffer.reset(avk::CommandBufferResetFlags::empty()).unwrap();
        }
    }
}
use std::{path::PathBuf, sync::Arc};
use winit::window::Window;

use ash::vk as avk;
use crate::*;

pub mod mesh;
pub use mesh::*;

pub mod instance_group;
pub use instance_group::*;

const MAX_FRAMES_IN_FLIGHT: usize = 2;

pub struct Renderer {
    pub frame_buffers: Vec<tvk::FrameBuffer>,
    pub pipeline: tvk::Pipeline,
    pub descriptor: tvk::Descriptor,
    pub render_pass: tvk::RenderPass,
    pub command_buffers: Vec<tvk::CommandBuffer>,
    pub sync_objects: tvk::SyncObjects,
    pub depth_buffer: tvk::DepthBuffer,
    pub swapchain: tvk::Swapchain,
    pub uniform_buffers: Vec<tvk::Buffer>,
    pub context: tvk::Context,
    pub frame_index: usize,
}

impl Renderer {
    pub fn new(window: &Window) -> AnyResult<Self> {
        let context: tvk::Context = tvk::Context::new(window)?;
        let swapchain = context.create_swapchain(window)?;
        let depth_buffer = context.create_depth_buffer(&swapchain)?;
        let render_pass = context.create_render_pass(&swapchain)?;
        let frame_buffers = context.create_frame_buffers(&swapchain, &render_pass, &depth_buffer.image_view)?;
        
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = manifest_dir
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
        .unwrap();
        let vertex_source = workspace_root.join("assets/generated/shaders/shader.vert.spv");
        let fragment_source = workspace_root.join("assets/generated/shaders/shader.frag.spv");
        let mut descriptor = context.create_descriptor_dependecies(MAX_FRAMES_IN_FLIGHT as u32)?;
        descriptor.allocate_sets()?;
        let pipeline = context.create_pipeline(
            &render_pass,
            descriptor.layout, 
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
        let uniform_buffers = (0..MAX_FRAMES_IN_FLIGHT).into_iter().map(|_| {
            context.create_buffer(
                avk::BufferUsageFlags::UNIFORM_BUFFER,
                gpu_allocator::MemoryLocation::CpuToGpu,
                size_of::<camera::Matrix>() as u64
            )
        }).collect::<AnyResult<Vec<_>>>()?;
        descriptor.update(&uniform_buffers)?;

        Ok(Self {
            frame_index: 0,
            context,
            swapchain,
            render_pass,
            frame_buffers,
            pipeline,
            sync_objects,
            command_buffers,
            descriptor,
            uniform_buffers,
            depth_buffer
        })
    }

    pub fn recreate_swapchain(&mut self, window: &Window) -> AnyResult<()> {
        self.context.logical_device.device_wait_idle()?;
        self.frame_buffers.clear();
        self.swapchain.recreate(&self.context, window)?;
        self.frame_buffers = self.context.create_frame_buffers(&self.swapchain, &self.render_pass, &self.depth_buffer.image_view)?;
        Ok(())
    }
    
    pub fn record_command_buffer(
        &self,
        command_buffer: &tvk::CommandBuffer,
        instance_groups: &[InstanceGroup],
        image_index: usize
    ) -> AnyResult<()> {
        command_buffer.begin(avk::CommandBufferUsageFlags::default())?;
        let clear_values = [avk::ClearValue {
            color: avk::ClearColorValue { float32: [0.0, 0.0, 0.08, 1.0] },
        },
        avk::ClearValue {
            depth_stencil: avk::ClearDepthStencilValue { depth: 1.0, stencil: 0}
        }]; 
        command_buffer.begin_render_pass(
            &self.swapchain, 
            &self.render_pass, 
            &self.frame_buffers[image_index], 
            avk::SubpassContents::INLINE,
            &clear_values
        );
        command_buffer.bind_pipeline(&self.pipeline);
        command_buffer.set_scissor(self.swapchain.get_scissor());
        command_buffer.set_viewport(self.swapchain.get_viewport());
        command_buffer.bind_descriptor_sets(self.pipeline.layout, self.descriptor.sets[self.frame_index]);
        for instance_group in instance_groups.iter() {
            let buffers = [instance_group.mesh.vertex_buffer.inner, instance_group.instance_buffer.as_ref().unwrap().inner];
            command_buffer.bind_vertex_buffers(&buffers);
            command_buffer.bind_index_buffer(&instance_group.mesh.index_buffer);
            command_buffer.draw_indexed(instance_group.mesh.indices.len() as u32, instance_group.visible_count as u32, 0, 0, 0);
        }
        command_buffer.end_render_pass();
        command_buffer.end()?;
        Ok(())
    }

    pub fn reset_command_buffers(&self) -> AnyResult<()> {
        for command_buffer in self.command_buffers.iter() {
            command_buffer.reset(avk::CommandBufferResetFlags::empty())?;
        }
        Ok(())
    }

    pub fn render(&mut self, camera: &Camera, instance_groups: &[InstanceGroup]) -> AnyResult<bool> {

        self.sync_objects.in_flight_fences[self.frame_index].wait(u64::MAX)?;
        let (image_index, _) = self.swapchain.acquire_next_image(
                u64::MAX,
                self.sync_objects.image_available_semaphores[self.frame_index].inner,
                avk::Fence::null()
        )?;

        if let Some(weak_images_in_flight) = &self.sync_objects.images_in_flight[image_index as usize] {
            if let Some(arc_images_in_flight) = weak_images_in_flight.upgrade() {
                arc_images_in_flight.wait(u64::MAX)?;
            }
        }
        self.sync_objects.images_in_flight[image_index as usize] = Some(Arc::downgrade(
            &self.sync_objects.in_flight_fences[self.frame_index]
        ));

        self.update_uniform_buffer(camera, image_index as usize)?;
        self.command_buffers[self.frame_index].reset(avk::CommandBufferResetFlags::empty())?;
        self.record_command_buffer(&self.command_buffers[self.frame_index], instance_groups, image_index as usize)?;
        
        self.sync_objects.in_flight_fences[self.frame_index].reset()?;

        let image_available_semaphores = [self.sync_objects.image_available_semaphores[self.frame_index].inner];
        let render_finished_semaphores = [self.sync_objects.render_finished_semaphores[self.frame_index].inner];
        let command_buffers = [self.command_buffers[self.frame_index].inner];
        let submit_info = avk::SubmitInfo::default()
            .wait_semaphores(&image_available_semaphores)
            .wait_dst_stage_mask(&[avk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
            .command_buffers(&command_buffers)
            .signal_semaphores(&render_finished_semaphores);
        self.context.queues.get(&tvk::QueueType::Graphics).unwrap().submit(&[submit_info], self.sync_objects.in_flight_fences[self.frame_index].inner)?;
        
        let is_suboptimal = self.swapchain.queue_present(
            self.context.queues.get(&tvk::QueueType::Graphics).unwrap(),
            image_index,
            &render_finished_semaphores
        )?;
        self.frame_index = (self.frame_index + 1) % MAX_FRAMES_IN_FLIGHT;
       
        Ok(is_suboptimal)
    }

    pub fn update_uniform_buffer(&mut self, camera: &Camera, index: usize) -> AnyResult<()>{
        let ubos = [camera::Matrix {
            view: camera.view_matrix(),
            proj: camera.projection
        }];
        self.uniform_buffers[index].copy_memory(&ubos)?;
        Ok(())
    }
} 

impl Drop for Renderer {
    fn drop(&mut self) {
        self.context.logical_device.device_wait_idle().unwrap();
    }
}
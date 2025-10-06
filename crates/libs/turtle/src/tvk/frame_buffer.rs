use std::sync::Arc;
use ash::vk as avk;
use crate::{tvk, AnyResult};

pub struct FrameBuffer {
    pub inner: avk::Framebuffer,
    logical_device: Arc<tvk::LogicalDevice>
}

impl FrameBuffer {
    pub fn new(
        logical_device: Arc<tvk::LogicalDevice>,
        swapchain: &tvk::Swapchain,
        render_pass: &tvk::RenderPass,
        image_view: &tvk::ImageView,
        ) -> AnyResult<Self> {
        let attachments = &[image_view.inner];
        let create_info = avk::FramebufferCreateInfo::default()
            .render_pass(render_pass.inner)
            .attachments(attachments)
            .width(swapchain.extent.width)
            .height(swapchain.extent.height)
            .layers(1);

        let inner = unsafe { logical_device.inner.create_framebuffer(&create_info, None)? };
        
        Ok(Self{
            inner,
            logical_device
        })
    } 
}

impl tvk::Context {
    pub fn create_frame_buffers(&self, swapchain:&tvk::Swapchain, render_pass: &tvk::RenderPass) -> AnyResult<Vec<FrameBuffer>> {
        swapchain.image_views.iter().map(|image_view| {
            tvk::FrameBuffer::new(self.logical_device.clone(), swapchain, render_pass, image_view)
        }).collect()
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe { self.logical_device.inner.destroy_framebuffer(self.inner, None);}
    }
}
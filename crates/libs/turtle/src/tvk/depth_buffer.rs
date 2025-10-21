use ash::vk as avk;

use crate::*;

pub struct DepthBuffer {
    pub image_view: tvk::ImageView,
    pub image: tvk::Image,
}

impl DepthBuffer {
    pub fn new(context: &tvk::Context, swapchain: &tvk::Swapchain) -> AnyResult<Self> {
        let format = context.physical_device.depth_format;
        let image = context.create_image(swapchain.extent, format, avk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)?;
        let image_view = context.create_image_view(&image, format, avk::ImageAspectFlags::DEPTH)?;


        Ok(Self {
            image,
            image_view
        })
    }
}

impl tvk::Context {
    pub fn create_depth_buffer(&self, swapchain: &tvk::Swapchain) -> AnyResult<DepthBuffer> {
        DepthBuffer::new(self, swapchain)
    }
}
use std::sync::Arc;
use ash::vk as avk;
use crate::{tvk, AnyResult};
pub struct RenderPass {
    logical_device: Arc<tvk::LogicalDevice>,
    pub inner: avk::RenderPass
}

impl RenderPass {
    pub fn new(
        logical_device: Arc<tvk::LogicalDevice>,
        swapchain: &tvk::Swapchain
    ) -> AnyResult<Self> {
        let dependency = avk::SubpassDependency::default()
            .src_subpass(avk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(avk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(avk::AccessFlags::empty())
            .dst_stage_mask(avk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(avk::AccessFlags::COLOR_ATTACHMENT_READ | avk::AccessFlags::COLOR_ATTACHMENT_WRITE);    
    
        let color_attachment = avk::AttachmentDescription::default()
            .format(swapchain.format)
            .samples(avk::SampleCountFlags::TYPE_1)
            .load_op(avk::AttachmentLoadOp::CLEAR)
            .store_op(avk::AttachmentStoreOp::STORE)
            .stencil_load_op(avk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(avk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(avk::ImageLayout::UNDEFINED)
            .final_layout(avk::ImageLayout::PRESENT_SRC_KHR)
            .samples(avk::SampleCountFlags::TYPE_1);
    
        let color_attachment_ref = avk::AttachmentReference::default()
            .attachment(0)
            .layout(avk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
    
        let color_attachments = &[color_attachment_ref];
        let subpass = avk::SubpassDescription::default()
            .pipeline_bind_point(avk::PipelineBindPoint::GRAPHICS)
            .color_attachments(color_attachments);
    
        let attachments = &[color_attachment];
        let subpasses = &[subpass];
        let dependencies = &[dependency];
        let create_info = avk::RenderPassCreateInfo::default()
            .attachments(attachments)
            .subpasses(subpasses)
            .dependencies(dependencies);
    
        let inner = unsafe { logical_device.inner.create_render_pass(&create_info, None)? };

        Ok(Self {
            inner,
            logical_device
        })
    }
}

impl tvk::Context {
    pub fn create_render_pass(&self, swapchain: &tvk::Swapchain) -> AnyResult<tvk::RenderPass> {
        tvk::RenderPass::new(self.logical_device.clone(), swapchain)
    }
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        unsafe { self.logical_device.inner.destroy_render_pass(self.inner, None); }
    }
}
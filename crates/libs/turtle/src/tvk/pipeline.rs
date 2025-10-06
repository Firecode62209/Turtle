use std::{path::Path, sync::Arc};
use ash::vk as avk;
use crate::{tvk, AnyResult};
pub struct Pipeline {
    pub viewport: avk::Viewport,
    pub scissor: avk::Rect2D,
    pub inner: avk::Pipeline,
    pub layout: avk::PipelineLayout,
    logical_device: Arc<tvk::LogicalDevice>,
    
}

#[derive(Debug, Clone, Copy)]
pub struct PipelineShaderCreateInfo<'a> {
    pub path: &'a Path,
    pub stage: avk::ShaderStageFlags
}

impl Pipeline {
    pub fn new(
        logical_device: Arc<tvk::LogicalDevice>,
        swapchain: &tvk::Swapchain,
        render_pass: &tvk::RenderPass,
        shaders: &[tvk::PipelineShaderCreateInfo],
    ) -> AnyResult<Self> {
        let layout_info = avk::PipelineLayoutCreateInfo::default();
        let layout = unsafe { logical_device.inner.create_pipeline_layout(&layout_info, None)? };

        let (_modules, stages) = shaders.into_iter()
            .map(|shader| {
            let shader_module = tvk::ShaderModule::create(logical_device.clone(), shader.path).unwrap();

            let stages = avk::PipelineShaderStageCreateInfo::default()
            .stage(shader.stage)
            .module(shader_module.inner)
            .name(c"main");
            (shader_module, stages)
        }).collect::<(Vec<_>, Vec<_>)>();

        let dynamic_states = &[
            avk::DynamicState::VIEWPORT,
            avk::DynamicState::SCISSOR
        ];

        let dynamic_state = avk::PipelineDynamicStateCreateInfo::default()
            .dynamic_states(dynamic_states);

        let attribute_descriptions = tvk::Vertex::get_attribute_descriptions();
        let binding_descriptions = tvk::Vertex::get_binding_descriptions();

        let vertex_input_state = avk::PipelineVertexInputStateCreateInfo::default()
            .vertex_attribute_descriptions(&attribute_descriptions)
            .vertex_binding_descriptions(&binding_descriptions);
        let input_assembly_state = avk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(avk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let viewport = avk::Viewport::default()
            .x(0.0)
            .y(0.0)
            .width(swapchain.extent.width as f32)
            .height(swapchain.extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0);

        let scissor = avk::Rect2D::default()
            .offset(avk::Offset2D { x: 0, y: 0 })
            .extent(swapchain.extent);

        let viewports = &[viewport];
        let scissors = &[scissor];
        let viewport_state = avk::PipelineViewportStateCreateInfo::default()
            .viewports(viewports)
            .scissors(scissors);

        let rasterization_state = avk::PipelineRasterizationStateCreateInfo::default()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(avk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(avk::CullModeFlags::BACK)
            .front_face(avk::FrontFace::CLOCKWISE)
            .depth_bias_enable(false);

        let multisample_state = avk::PipelineMultisampleStateCreateInfo::default()
            .sample_shading_enable(false)
            .rasterization_samples(avk::SampleCountFlags::TYPE_1);
        
        let attachment = avk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(avk::ColorComponentFlags::RGBA)
            .blend_enable(true)
            .src_color_blend_factor(avk::BlendFactor::SRC_ALPHA)
            .dst_color_blend_factor(avk::BlendFactor::ONE_MINUS_SRC_ALPHA)
            .color_blend_op(avk::BlendOp::ADD)
            .src_alpha_blend_factor(avk::BlendFactor::ONE)
            .dst_alpha_blend_factor(avk::BlendFactor::ZERO)
            .alpha_blend_op(avk::BlendOp::ADD);   

        let attachments = &[attachment];
        let color_blend_state = avk::PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .logic_op(avk::LogicOp::COPY)
            .attachments(attachments)
            .blend_constants([0.0, 0.0, 0.0, 0.0]);

        let create_info = avk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .vertex_input_state(&vertex_input_state)
            .input_assembly_state(&input_assembly_state)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterization_state)
            .multisample_state(&multisample_state)
            .color_blend_state(&color_blend_state)
            .layout(layout)
            .subpass(0)
            .render_pass(render_pass.inner)
            .dynamic_state(&dynamic_state);

        let inner = unsafe { logical_device.inner.create_graphics_pipelines(avk::PipelineCache::null(), &[create_info], None)
            .map_err(|e| e.1)?[0]
        };

        Ok(Self {
            inner,
            layout,
            logical_device,
            viewport,
            scissor
        })
    }
}

impl tvk::Context {
    pub fn create_pipelines(
        &self,
        swapchain: &tvk::Swapchain,
        render_pass: &tvk::RenderPass,
        shaders: &[tvk::PipelineShaderCreateInfo]
    ) -> AnyResult<tvk::Pipeline> {
        tvk::Pipeline::new(self.logical_device.clone(), swapchain, render_pass, shaders)
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe { 
            self.logical_device.inner.destroy_pipeline(self.inner, None);
            self.logical_device.inner.destroy_pipeline_layout(self.layout, None);
        }
    }
}
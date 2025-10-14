use ash::{khr::swapchain, vk as avk};
use winit::window::Window;
use crate::{tvk, AnyResult};

fn get_swapchain_surface_format(
    formats: &[avk::SurfaceFormatKHR],
) -> avk::SurfaceFormatKHR {
    formats
        .iter()
        .cloned()
        .find(|f| {
            f.format == avk::Format::B8G8R8A8_SRGB
                && f.color_space == avk::ColorSpaceKHR::SRGB_NONLINEAR
        })
        .unwrap_or_else(|| formats[0])
}

fn get_swapchain_present_mode(
    present_modes: &[avk::PresentModeKHR],
) -> avk::PresentModeKHR {
    present_modes
        .iter()
        .cloned()
        .find(|m| {
            *m == avk::PresentModeKHR::MAILBOX
        })
        .unwrap_or(avk::PresentModeKHR::FIFO)
}

fn get_swapchain_extent(
    window: &Window,
    capabilities: avk::SurfaceCapabilitiesKHR
) -> avk::Extent2D {
    avk::Extent2D::default()
        .width(window.inner_size().width.clamp(
            capabilities.min_image_extent.width,
            capabilities.max_image_extent.width
        ))
        .height(window.inner_size().height.clamp(
            capabilities.min_image_extent.height,
            capabilities.max_image_extent.height
        ))
}

pub struct Swapchain {
    pub image_views: Vec<tvk::ImageView>,
    pub images: Vec<avk::Image>,
    pub format: avk::Format,
    pub extent: avk::Extent2D,
    pub color_space: avk::ColorSpaceKHR,
    pub present_mode: avk::PresentModeKHR,
    pub swapchain_khr: avk::SwapchainKHR,
    inner: swapchain::Device,
}

impl Swapchain {
    pub fn new(
        context: &tvk::Context,
        window: &Window,
    ) -> AnyResult<Self> {
        let format = get_swapchain_surface_format(&context.physical_device.formats);
        let present_mode = get_swapchain_present_mode(&context.physical_device.present_modes);
        log::info!("Swapchain present mode set to {:?}", present_mode);
        let extent = get_swapchain_extent(window, context.physical_device.surface_capabilities);

        let mut queue_family_indices = Vec::new();
        let sharing_mode = if context.queue_families.get(&tvk::QueueType::Graphics).unwrap().index != context.queue_families.get(&tvk::QueueType::Present).unwrap().index {
            queue_family_indices.push(context.queue_families.get(&tvk::QueueType::Graphics).unwrap().index);
            queue_family_indices.push(context.queue_families.get(&tvk::QueueType::Present).unwrap().index);
            avk::SharingMode::CONCURRENT
        } else {
            avk::SharingMode::EXCLUSIVE
        };

        let create_info = avk::SwapchainCreateInfoKHR::default()
            .surface(context.surface.surface_khr)
            .min_image_count(2) // Double buffering
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(extent) // Placeholder dimensions
            .image_array_layers(1)
            .image_usage(avk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(sharing_mode)
            .queue_family_indices(&queue_family_indices)
            .pre_transform(context.physical_device.surface_capabilities.current_transform)
            .composite_alpha(avk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true);

        let inner = swapchain::Device::new(&context.instance.inner, &context.logical_device.inner);
        let swapchain_khr = unsafe { inner.create_swapchain(&create_info, None)? };        
        let images = unsafe { inner.get_swapchain_images(swapchain_khr)? };
        let image_views = images.iter().map(|&image| {
            tvk::ImageView::new(
                image,
                format.format,
                context.logical_device.clone(),
            )
        }).collect::<AnyResult<Vec<_>>>()?;

        Ok(Self {
            inner,
            swapchain_khr,
            images,
            image_views,
            format: format.format,
            extent,
            color_space: format.color_space,
            present_mode,
        })
    }

    pub fn recreate(
        &mut self,
        context: &tvk::Context,
        window: &Window
    ) -> AnyResult<()> {
        self.cleanup();
        self.extent = get_swapchain_extent(window, context.physical_device.surface_capabilities);
        
        let mut queue_family_indices = Vec::new();
        let sharing_mode = if context.queue_families.get(&tvk::QueueType::Graphics).unwrap().index != context.queue_families.get(&tvk::QueueType::Present).unwrap().index {
            queue_family_indices.push(context.queue_families.get(&tvk::QueueType::Graphics).unwrap().index);
            queue_family_indices.push(context.queue_families.get(&tvk::QueueType::Present).unwrap().index);
            avk::SharingMode::CONCURRENT
        } else {
            avk::SharingMode::EXCLUSIVE
        };

        let create_info = avk::SwapchainCreateInfoKHR::default()
            .surface(context.surface.surface_khr)
            .min_image_count(2)
            .image_format(self.format)
            .image_color_space(self.color_space)
            .image_extent(self.extent) 
            .image_array_layers(1)
            .image_usage(avk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(sharing_mode)
            .queue_family_indices(&queue_family_indices)
            .pre_transform(context.physical_device.surface_capabilities.current_transform)
            .composite_alpha(avk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(self.present_mode)
            .clipped(true);

        self.swapchain_khr = unsafe { self.inner.create_swapchain(&create_info, None)? };        
        self.images = unsafe { self.inner.get_swapchain_images(self.swapchain_khr)? };
        self.image_views = self.images.iter().map(|&image| {
            tvk::ImageView::new(
                image,
                self.format,
                context.logical_device.clone(),
            )
        }).collect::<AnyResult<Vec<_>>>()?;

        Ok(())
    }

    pub fn cleanup(&mut self) {
        unsafe {
            self.image_views.clear();
            self.images.clear();
            self.inner.destroy_swapchain(self.swapchain_khr, None);
        }
    }

    pub fn acquire_next_image(&self, timeout: u64, semaphore: avk::Semaphore, fence: avk::Fence) -> AnyResult<(u32, bool)> {
        let (image_index, is_suboptimal) = unsafe {
            self.inner.acquire_next_image(
                self.swapchain_khr,
                timeout,
                semaphore,
                fence
            )?
        };
        Ok((image_index, is_suboptimal))
    }

    pub fn queue_present(&self, queue: &tvk::Queue, image_index: u32, wait_semaphores: &[avk::Semaphore]) -> AnyResult<bool> {
        let image_indices = &[image_index];
        let swapchains = &[self.swapchain_khr];
        let present_info = avk::PresentInfoKHR::default()
            .wait_semaphores(wait_semaphores)
            .swapchains(swapchains)
            .image_indices(image_indices);

        let result = unsafe {
            self.inner.queue_present(queue.inner, &present_info)?
        };

        Ok(result)
    }

    pub fn get_scissor(&self) -> avk::Rect2D {
        avk::Rect2D::default()
            .offset(avk::Offset2D { x: 0, y: 0 })
            .extent(self.extent)
    }

    pub fn get_viewport(&self) -> avk::Viewport {
        avk::Viewport::default()
            .x(0.0)
            .y(0.0)
            .width(self.extent.width as f32)
            .height(self.extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0)
    }
}

impl tvk::Context {
    pub fn create_swapchain(&self, window: &Window) -> AnyResult<tvk::Swapchain> {
        tvk::Swapchain::new(self, window)
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            self.image_views.clear();
            self.images.clear();
            self.inner.destroy_swapchain(self.swapchain_khr, None);
        }
    }
}


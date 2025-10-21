use std::sync::Arc;

use ash::vk as avk;
use crate::{tvk, AnyResult};

pub struct ImageView {
    pub(crate) inner: avk::ImageView,
    logical_device: Arc<tvk::LogicalDevice>,
}

impl ImageView {
    pub fn new(
        logical_device: Arc<tvk::LogicalDevice>,
        image: avk::Image,
        format: avk::Format,
        aspect_flags: avk::ImageAspectFlags
    ) -> AnyResult<Self> {
        let create_info = avk::ImageViewCreateInfo::default()
            .image(image)
            .view_type(avk::ImageViewType::TYPE_2D)
            .format(format)
            .components(avk::ComponentMapping {
                r: avk::ComponentSwizzle::IDENTITY,
                g: avk::ComponentSwizzle::IDENTITY,
                b: avk::ComponentSwizzle::IDENTITY,
                a: avk::ComponentSwizzle::IDENTITY,
            })
            .subresource_range(avk::ImageSubresourceRange {
                aspect_mask: aspect_flags,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            });

        let inner = unsafe { logical_device.inner.create_image_view(&create_info, None)? };

        Ok(Self {
            inner,
            logical_device
        })
    }
}

impl tvk::Context {
    pub fn create_image_view(&self, image: &tvk::Image, format: avk::Format, aspect_flags: avk::ImageAspectFlags) -> AnyResult<ImageView> {
        ImageView::new(self.logical_device.clone(), image.inner, format, aspect_flags)
    }
}

impl Drop for ImageView {
    fn drop(&mut self) {
        unsafe {
            self.logical_device.inner.destroy_image_view(self.inner, None);
        }
    }
}
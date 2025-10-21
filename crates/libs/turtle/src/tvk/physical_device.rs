

use ash::vk as avk;
use crate::{tvk, AnyResult};


#[derive(Debug, Clone)]
pub struct PhysicalDevice {
    pub depth_format: avk::Format,
    pub surface_capabilities: avk::SurfaceCapabilitiesKHR,
    pub formats: Vec<avk::SurfaceFormatKHR>,
    pub present_modes: Vec<avk::PresentModeKHR>,
    pub(crate) queue_families: Vec<tvk::QueueFamily>,
    pub(crate) properties: avk::PhysicalDeviceProperties,
    pub(crate) inner: avk::PhysicalDevice,
}

impl PhysicalDevice {
    pub fn new(
        surface: &tvk::Surface,
        inner: avk::PhysicalDevice,
        instance: &tvk::Instance,
    ) -> AnyResult<Self> {
        let properties = unsafe { instance.inner.get_physical_device_properties(inner)};

        let queue_families = unsafe {
            instance.inner.get_physical_device_queue_family_properties(inner)
                .iter()
                .enumerate()
                .map(|(i, qf)| tvk::QueueFamily::new(&inner, surface, i as u32, *qf) )
                .collect::<AnyResult<Vec<_>>>()?
        };

        let surface_capabilities = unsafe {
            surface.inner.get_physical_device_surface_capabilities(inner, surface.surface_khr)?
        };
        let formats = unsafe {
            surface.inner.get_physical_device_surface_formats(inner, surface.surface_khr)?
        };
        let present_modes = unsafe {
            surface.inner.get_physical_device_surface_present_modes(inner, surface.surface_khr)?
        };

        let depth_format = PhysicalDevice::find_depth_format(inner, instance)?;

        Ok(Self {
            inner,
            properties,
            queue_families,
            surface_capabilities,
            formats,
            present_modes,
            depth_format
        })
    }

    fn find_supported_format(
    physical_device: avk::PhysicalDevice,
    instance: &tvk::Instance,
    candidates: &[avk::Format], 
    tiling: avk::ImageTiling,
    features: avk::FormatFeatureFlags
    ) -> AnyResult<avk::Format> {
        for &format in candidates.iter() {
            unsafe {
                let props = instance.inner.get_physical_device_format_properties(physical_device, format);

                if tiling == avk::ImageTiling::LINEAR && (props.linear_tiling_features & features == features) {
                    return Ok(format);
                } else if tiling == avk::ImageTiling::OPTIMAL && (props.optimal_tiling_features & features) == features {
                    return Ok(format);
                }
            }
        }
        
        Err(String::from("cannot find supported format").into())
    }

    fn find_depth_format(physical_device: avk::PhysicalDevice, instance: &tvk::Instance) -> AnyResult<avk::Format> {
        let candidates = [avk::Format::D32_SFLOAT, avk::Format::D32_SFLOAT_S8_UINT, avk::Format::D24_UNORM_S8_UINT];
        PhysicalDevice::find_supported_format(physical_device, instance, &candidates, avk::ImageTiling::OPTIMAL, avk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT)
    }
}
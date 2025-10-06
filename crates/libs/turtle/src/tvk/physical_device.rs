use ash::vk as avk;
use crate::{tvk, AnyResult};


#[derive(Debug, Clone)]
pub struct PhysicalDevice {
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

        Ok(Self {
            inner,
            properties,
            queue_families,
            surface_capabilities,
            formats,
            present_modes
        })
    }
}
use ash::vk as avk;
use winit::{raw_window_handle::HasDisplayHandle, window::Window};
use crate::{tvk, AnyResult};

pub struct Instance {
    pub debug_utils: ash::ext::debug_utils::Instance,
    pub debug_utils_messenger: avk::DebugUtilsMessengerEXT,
    pub(crate) physical_devices: Vec<tvk::PhysicalDevice>,
    pub(crate) inner: ash::Instance,
}

impl Instance {
    pub fn new(
        entry: &ash::Entry,
        window: &Window,
        extension_names: &[&std::ffi::CStr],
        layer_names: &[&std::ffi::CStr],
    ) -> AnyResult<Self> {
            let application_info = avk::ApplicationInfo::default()
            .api_version(avk::make_api_version(0, 1, 0, 0));

            let mut extensions_pointer = extension_names
                .iter()
                .map(|e| e.as_ptr())
                .collect::<Vec<_>>();

            extensions_pointer.extend(ash_window::enumerate_required_extensions(window.display_handle()?.as_raw())?);
            
            let layers_pointer = layer_names
                .iter()
                .map(|l| l.as_ptr())
                .collect::<Vec<_>>();

            let create_flags = avk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR;

            let mut debug_create_info = avk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(avk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | avk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                | avk::DebugUtilsMessageSeverityFlagsEXT::INFO
                | avk::DebugUtilsMessageSeverityFlagsEXT::ERROR)
            .message_type(avk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | avk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                | avk::DebugUtilsMessageTypeFlagsEXT::VALIDATION)
            .pfn_user_callback(Some(vulkan_debug_utils_callback));

            let create_info = avk::InstanceCreateInfo::default()
                .application_info(&application_info)
                .enabled_extension_names(&extensions_pointer)
                .flags(create_flags)
                .enabled_layer_names(&layers_pointer)
                .push_next(&mut debug_create_info);

            let inner = unsafe { entry.create_instance(&create_info, None)? };  
            let debug_utils = ash::ext::debug_utils::Instance::new(&entry, &inner);

            let debug_utils_messenger = unsafe { debug_utils.create_debug_utils_messenger(&debug_create_info, None)? };

            let physical_devices = vec![];

        Ok(Self {
            debug_utils,
            debug_utils_messenger,
            physical_devices,
            inner
        })
    }

    pub fn enumerate_physical_devices(&mut self, surface: &tvk::Surface) -> AnyResult<&[tvk::PhysicalDevice]> {
        if self.physical_devices.is_empty() {
            self.physical_devices = unsafe {
            self.inner.enumerate_physical_devices()?
                .iter()
                .map(|pd| tvk::PhysicalDevice::new(surface, *pd, &self))
                .collect::<AnyResult<Vec<_>>>()?
            };

            self.physical_devices.sort_by_key(|pd| match pd.properties.device_type {
                avk::PhysicalDeviceType::DISCRETE_GPU => 0,
                avk::PhysicalDeviceType::INTEGRATED_GPU => 1,
                _ => 2
            });
        };

        Ok(&self.physical_devices)
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { 
            self.debug_utils.destroy_debug_utils_messenger(self.debug_utils_messenger, None);
            self.inner.destroy_instance(None); 
        }
    }
}

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: avk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: avk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const avk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut std::ffi::c_void,
) -> avk::Bool32 {
    let message = unsafe { std::ffi::CStr::from_ptr((*p_callback_data).p_message) };
    let severity = format!("{:?}", message_severity).to_lowercase();
    let ty = format!("{:?}", message_type).to_lowercase();
    match message_severity {
        avk::DebugUtilsMessageSeverityFlagsEXT::ERROR => {
            log::error!("[{}][{}] {:?}", severity, ty, message);
        },
        avk::DebugUtilsMessageSeverityFlagsEXT::WARNING => {
            log::warn!("[{}][{}] {:?}", severity, ty, message);
        },
        avk::DebugUtilsMessageSeverityFlagsEXT::INFO => {
            log::info!("[{}][{}] {:?}", severity, ty, message);
        },
        avk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => {
            log::debug!("[{}][{}] {:?}", severity, ty, message);
        },
        _ => {}
    }
    avk::FALSE
}
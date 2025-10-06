use ash::vk as avk;
use crate::{tvk, AnyResult};

pub struct LogicalDevice {
    pub(crate) inner: ash::Device
}

impl LogicalDevice {
    pub fn new (
        instance: &tvk::Instance,
        physical_device: &tvk::PhysicalDevice,
        queue_families: Vec<tvk::QueueFamily>,
        extension_names: &[&std::ffi::CStr],
    ) -> AnyResult<Self> {
        let priorities = [1.0f32];

        let queue_infos = {
            let mut indices = queue_families.iter().map(|qf| qf.index).collect::<Vec<_>>();
            indices.dedup();

            indices.iter().map(|i| avk::DeviceQueueCreateInfo::default()
                .queue_family_index(*i)
                .queue_priorities(&priorities)
            )
            .collect::<Vec<_>>()
        };
        

        let extensions_pointer = extension_names
                .iter()
                .map(|e| e.as_ptr())
                .collect::<Vec<_>>();

        let pf = avk::PhysicalDeviceFeatures {
            shader_int64: avk::TRUE,
            ..Default::default()
        };
        
        let mut features = avk::PhysicalDeviceFeatures2::default()
            .features(pf);
            
        let create_info = avk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_infos)
            .enabled_extension_names(&extensions_pointer)
            .push_next(&mut features);

        let inner = unsafe { instance.inner.create_device(physical_device.inner, &create_info, None)? };

        Ok(Self {
            inner
        })
    }

    pub fn wait_for_fences(&self, fences: &[avk::Fence], wait_all: bool, timeout: u64) -> AnyResult<()> {
        unsafe {
            self.inner.wait_for_fences(fences, wait_all, timeout)?;
        }
        Ok(())
    }

    pub fn reset_fences(&self, fences: &[avk::Fence]) -> AnyResult<()> {
        unsafe {
            self.inner.reset_fences(fences)?;
        }
        Ok(())
    }

    pub fn device_wait_idle(&self) -> AnyResult<()> {
        unsafe {
            self.inner.device_wait_idle()?;
        }
        Ok(())
    }
}

impl Drop for LogicalDevice {
    fn drop(&mut self) {
        unsafe { self.inner.destroy_device(None); }
    }
}
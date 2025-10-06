use ash::vk as avk;
use winit::{raw_window_handle::{HasDisplayHandle, HasWindowHandle}, window::Window};

use crate::{tvk, AnyResult};

pub struct Surface {
    pub surface_khr: avk::SurfaceKHR,
    pub(crate) inner: ash::khr::surface::Instance
}

impl Surface {
    pub fn new(
        entry: &ash::Entry,
        instance: &tvk::Instance,
        window: &Window
    ) -> AnyResult<Self> {
        let surface_khr = unsafe { ash_window::create_surface(
            entry, 
            &instance.inner, 
            window.display_handle()?.as_raw(), 
            window.window_handle()?.as_raw(), 
            None
            )?
        };
        let inner = ash::khr::surface::Instance::new(entry, &instance.inner);

        Ok(Self {
            inner,
            surface_khr
        })
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.inner.destroy_surface(self.surface_khr, None);
        }
    }
}
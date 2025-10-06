use std::{fs::File, path::Path, sync::Arc};
use ash::vk as avk;
use crate::{tvk, AnyResult};

pub struct ShaderModule {
    logical_device: Arc<tvk::LogicalDevice>,
    pub inner: avk::ShaderModule
}

impl ShaderModule {
    pub fn create(
        logical_device: Arc<tvk::LogicalDevice>,
        path: &Path
    ) -> AnyResult<Self> {
        let file = File::open(path)?;
        let mut reader = std::io::BufReader::new(file);
        let code = ash::util::read_spv(&mut reader)?;
        let create_info = avk::ShaderModuleCreateInfo::default().code(&code);

        let inner = unsafe { logical_device.inner.create_shader_module(&create_info, None)? };
    
        Ok(Self {
            inner,
            logical_device
        })
    }
}

impl Drop for ShaderModule {
    fn drop(&mut self) {
        unsafe { self.logical_device.inner.destroy_shader_module(self.inner, None); }
    }
}
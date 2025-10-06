use ash::vk as avk;
use std::sync::Arc;
use crate::{tvk, AnyResult};

pub struct SyncObjects {
    pub(crate) image_available_semaphores: Vec<tvk::Semaphore>,
    pub(crate) render_finished_semaphores: Vec<tvk::Semaphore>,
    pub(crate) in_flight_fences: Vec<tvk::Fence>,
}

impl SyncObjects {
    pub fn new(logical_device: Arc<tvk::LogicalDevice>, max_frames_in_flight: usize) -> AnyResult<Self> {
        let mut image_available_semaphores = Vec::with_capacity(max_frames_in_flight);
        let mut render_finished_semaphores = Vec::with_capacity(max_frames_in_flight);
        let mut in_flight_fences = Vec::with_capacity(max_frames_in_flight);

        for _ in 0..max_frames_in_flight {
            image_available_semaphores.push(tvk::Semaphore::new(Arc::clone(&logical_device))?);
            render_finished_semaphores.push(tvk::Semaphore::new(Arc::clone(&logical_device))?);
            in_flight_fences.push(tvk::Fence::new(Arc::clone(&logical_device), true)?);
        }

        Ok(Self {
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences
        })
    }
}

impl tvk::Context {
    pub fn create_sync_objects(&self, max_frames_in_flight: usize) -> AnyResult<SyncObjects> {
        SyncObjects::new(Arc::clone(&self.logical_device), max_frames_in_flight)
    }
}
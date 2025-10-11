use std::sync::{Arc, Weak};
use crate::{tvk, AnyResult};

pub struct SyncObjects {
    pub(crate) image_available_semaphores: Vec<tvk::Semaphore>,
    pub(crate) render_finished_semaphores: Vec<tvk::Semaphore>,
    pub(crate) in_flight_fences: Vec<Arc<tvk::Fence>>,
    pub(crate) images_in_flight: Vec<Option<Weak<tvk::Fence>>>,
}

impl SyncObjects {
    pub fn new(logical_device: Arc<tvk::LogicalDevice>, swapchain_image_count: usize, max_frames_in_flight: usize) -> AnyResult<Self> {
        let mut image_available_semaphores = Vec::with_capacity(max_frames_in_flight);
        let mut render_finished_semaphores = Vec::with_capacity(max_frames_in_flight);
        let mut in_flight_fences = Vec::with_capacity(max_frames_in_flight);
        let images_in_flight = vec![None; swapchain_image_count];

        for _ in 0..max_frames_in_flight {
            image_available_semaphores.push(tvk::Semaphore::new(Arc::clone(&logical_device))?);
            render_finished_semaphores.push(tvk::Semaphore::new(Arc::clone(&logical_device))?);
            in_flight_fences.push(Arc::new(tvk::Fence::new(Arc::clone(&logical_device), true)?));
        }



        Ok(Self {
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences,
            images_in_flight,
        })
    }
}

impl tvk::Context {
    pub fn create_sync_objects(&self, swapchain_image_count: usize, max_frames_in_flight: usize) -> AnyResult<SyncObjects> {
        SyncObjects::new(Arc::clone(&self.logical_device), max_frames_in_flight, swapchain_image_count)
    }
}
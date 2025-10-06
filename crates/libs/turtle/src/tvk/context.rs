use crate::{tvk, AnyResult};
use std::{collections::HashMap, sync::{Arc, Mutex}};
use ash::vk as avk;
use winit::window::Window;

const INSTANCE_EXTENSION_NAMES: [&'static std::ffi::CStr; 4] = [
    avk::KHR_PORTABILITY_ENUMERATION_NAME,
    ash::ext::debug_utils::NAME,
    avk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_NAME,
    avk::KHR_SURFACE_NAME,
];

const INSTANCE_LAYER_NAMES: [&'static std::ffi::CStr; 1] = [
    unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0") }
];

const DEVICE_EXTENSION_NAMES: [&'static std::ffi::CStr; 2] = [
    avk::KHR_PORTABILITY_SUBSET_NAME,
    avk::KHR_SWAPCHAIN_NAME,
];

#[derive(PartialEq, Eq, Hash)]
pub enum QueueType {
    Graphics,
    Transfer,
    Present
}

pub struct Context {
    pub allocator: Arc<Mutex<tvk::Allocator>>,
    pub command_pools: HashMap<QueueType, Arc<tvk::CommandPool>>,
    pub queues: HashMap<QueueType, tvk::Queue>,
    pub logical_device: Arc<tvk::LogicalDevice>,
    pub queue_families: HashMap<QueueType, tvk::QueueFamily>,
    pub physical_device: tvk::PhysicalDevice,
    pub surface: tvk::Surface,
    pub instance: tvk::Instance,
    pub entry: ash::Entry
}

impl Context {
    pub fn new(window: &Window) -> AnyResult<Self> {

        let entry = unsafe { ash::Entry::load()? };
        let mut instance = tvk::Instance::new(&entry, window, &INSTANCE_EXTENSION_NAMES, &INSTANCE_LAYER_NAMES)?;
        let surface = tvk::Surface::new(&entry, &instance, window)?;
        let (physical_device, graphics, transfer, present) = select_physical_device(instance.enumerate_physical_devices(&surface)?)?;
        let logical_device = Arc::new(tvk::LogicalDevice::new(&instance, &physical_device, vec![graphics, transfer], &DEVICE_EXTENSION_NAMES)?);
        let mut queues = HashMap::new();
        let mut queue_families = HashMap::new();
        let mut command_pools = HashMap::new();
        queue_families.insert(QueueType::Graphics, graphics);
        queue_families.insert(QueueType::Transfer, transfer);
        queue_families.insert(QueueType::Present, present);
        queues.insert(QueueType::Graphics, tvk::Queue::new(graphics.index, logical_device.clone()));
        queues.insert(QueueType::Transfer, tvk::Queue::new(transfer.index, logical_device.clone()));
        queues.insert(QueueType::Present, tvk::Queue::new(present.index, logical_device.clone()));
        command_pools.insert(QueueType::Graphics, Arc::new(tvk::CommandPool::new(logical_device.clone(), graphics.index)?));
        let allocator = Arc::new(Mutex::new(tvk::Allocator::new(&instance, &logical_device, &physical_device)?));
        
        Ok(Self {
            entry,
            instance,
            surface,
            logical_device,
            physical_device,
            queues,
            queue_families,
            command_pools,
            allocator,
        })
    }
}

pub fn select_physical_device(physical_devices: &[tvk::PhysicalDevice]) -> AnyResult<(tvk::PhysicalDevice, tvk::QueueFamily, tvk::QueueFamily, tvk::QueueFamily)> {
    let mut graphics = None;
    let mut transfer = None;
    let mut present = None;

    let device = physical_devices.iter()
        .find(|device| {
            for family in device.queue_families.iter().filter(|f| f.has_queues()) {
                if family.supports_graphics() && graphics.is_none() {
                    graphics = Some(*family);
                }

                if family.supports_transfer() && transfer.is_none() {
                    transfer = Some(*family);
                }

                if family.supports_present() && present.is_none() {
                    present = Some(*family);
                }

                if graphics.is_some() && transfer.is_some() && present.is_some() {
                    break;
                }
            }

            graphics.is_some() &&
            transfer.is_some() &&
            present.is_some()
        }).unwrap();
        Ok((device.clone(), graphics.unwrap(), transfer.unwrap(), present.unwrap()))
    
}
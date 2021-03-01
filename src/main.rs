#![feature(generic_associated_types)]

use std::sync::Arc;
use magma::{
	Entry,
	Instance,
	instance::{
		PhysicalDevice,
		physical_device::QueueFamily
	},
	device,
	Device,
	win::{
		self,
		WindowBuilderExt
	},
	swapchain
};
use winit::{
	event_loop::EventLoop,
	event::{
		Event,
		WindowEvent,
		MouseButton,
		ElementState
	},
	window::{
		Window as WinitWindow,
		WindowBuilder
	}
};
use scene::Scene;
use cycles::Conductor;

mod render;
mod pov;
mod generator;

use pov::Pov;
use generator::Generator;

fn main() {
	// Logging facility.
	stderrlog::new().verbosity(3).init().unwrap();

	// Vulkan/magma initialization.
	let instance = init_magma();
	let physical_device = choose_physical_device(&instance);

	// Window creation.
	let event_loop = EventLoop::new();
	let window = WindowBuilder::new().build_vk_surface(&event_loop, &instance).unwrap();

	// Device and queues.
	let (device, queues) = create_device_and_queues(&physical_device, &window);

	// The scene.
	let mut scene: Scene<wonder::Object, scene::Event> = Scene::new();
	scene.insert(wonder::Object::Planet(wonder::object::Planet::new()));

	// Architect thread (in charge of imagining the scene).
	// TODO

	// Builder thread (in charge of building geometries/textures/materials).
	// TODO

	// Loading/Waiting thread (in charge of waiting & tranfering data from/to the CPU/GPU).
	let mut loading_thread = cycles::Thread::new();
	let allocator = panic!("TODO");
	let (loader, thread, worker) = engine::sync::Loader::new(allocator, queues.transfer);

	// Render thread
	let pov = Pov::new();
	let generator = Generator::new();
	let mut render_thread = cycles::Thread::new();
	render_thread.add(move || {
		let render_surface = render::Surface::new(&device, queues.presentation, window);
		engine::render::Worker::new(render_surface, pov, generator)
	});

	// Conductor (in charge of synchronizing all the threads).
	let mut conductor = Conductor::new(scene);
	conductor.add(loading_thread);
	conductor.add(render_thread);

	// Let's go!
	event_loop.run(move |event, _, _| {
		conductor.cycle()
	});
}

fn init_magma() -> Arc<Instance> {
	let entry = Arc::new(Entry::new().expect("Unable to load vulkan"));

	let required_extensions = win::required_extensions(&entry);

	for ext in required_extensions {
		println!("extension: {}", ext);
	}

	match Instance::new(entry, required_extensions) {
		Ok(i) => Arc::new(i),
		Err(e) => {
			log::error!("Could not build instance: {:?}", e);
			std::process::exit(1);
		}
	}
}

fn choose_physical_device(instance: &Arc<Instance>) -> PhysicalDevice {
	for physical_device in instance.physical_devices() {
		println!("device: {}", physical_device.name());
	}

	let physical_device = instance.physical_devices().last().unwrap();
	println!("choosen device: {}", physical_device.name());

	physical_device
}

pub enum QueueType<'a> {
	Graphics,
	Presentation(&'a swapchain::Surface<WinitWindow>),
	Transfer
}

pub struct Queues {
	transfer: device::Queue,
	graphics: device::Queue,
	presentation: device::Queue
}

fn get_queue_family<'a>(physical_device: &'a PhysicalDevice, ty: QueueType) -> QueueFamily<'a> {
	physical_device.queue_families().find(|&queue| {
		match ty {
			QueueType::Graphics => queue.supports_graphics(),
			QueueType::Presentation(surface) => surface.is_supported(queue).unwrap_or(false),
			QueueType::Transfer => queue.supports_transfer()
		}
	}).unwrap()
}

fn create_device_and_queues<'a>(physical_device: &'a PhysicalDevice, surface: &swapchain::Surface<WinitWindow>) -> (Arc<Device>, Queues) {
	let transfer_queue_family = get_queue_family(physical_device, QueueType::Transfer);
	let graphics_queue_family = get_queue_family(physical_device, QueueType::Graphics);
	let presentation_queue_family = get_queue_family(physical_device, QueueType::Presentation(surface));
	
	// TODO check that this extension is supported?
	let device_ext = device::Extensions {
		khr_swapchain: true,
		..device::Extensions::none()
	};

	let (device, mut queues) = Device::new(
		physical_device.clone(),
		physical_device.supported_features(), // enabled features (all of them?)
		&device_ext,
		[
			(transfer_queue_family, 1.0),
			(graphics_queue_family, 1.0),
			(presentation_queue_family, 1.0)
		].iter().cloned()
	).expect("unable to create logical device and queues");

	(device, Queues {
		transfer: queues.next().unwrap(),
		graphics: queues.next().unwrap(),
		presentation: queues.next().unwrap()
	})
}
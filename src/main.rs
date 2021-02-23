#![feature(generic_associated_types)]

use std::sync::Arc;
use magma::{
	Entry,
	Instance,
	instance::{
		PhysicalDevice,
		physical_device::QueueFamily
	},
	Device,
	win::{
		self,
		WindowBuilderExt
	},
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
	stderrlog::new().verbosity(3).init().unwrap();

	let instance = init_magma();

	let event_loop = EventLoop::new();
	let window = WindowBuilder::new().build_vk_surface(&event_loop, &instance).unwrap();

	let mut scene: Scene<wonder::Object, scene::Event> = Scene::new();
	scene.insert(wonder::Object::Planet(wonder::object::Planet::new()));

	let pov = Pov::new();
	let generator = Generator::new();

	let mut conductor = Conductor::new(scene);
	let mut render_thread = cycles::Thread::new();
	
	render_thread.add(move || {
		let physical_device = choose_physical_device(&instance);
		let render_surface = render::Surface::new(physical_device, window);
		engine::render::Worker::new(render_surface, pov, generator)
	});

	conductor.add(render_thread);

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
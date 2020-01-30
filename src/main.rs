extern crate wonder;

extern crate wabs;
extern crate wabs_wayland as wayland;
extern crate pastel;
extern crate layout;
extern crate bottle;
extern crate render_gl as render;
extern crate engine;
extern crate gl_loader;
extern crate gl;
extern crate khronos_egl as egl;
#[macro_use]
extern crate cascading;

#[macro_use]
extern crate log;
extern crate stderrlog;
#[macro_use]
extern crate clap;

use std::sync::Arc;
use wabs::{Client, Window};
use layout::Layout;
use bottle::{Remote, Handler, Emitter, Sender, Scheduler, SimpleScheduler};
use pastel::{view::RemoteView};

mod graphics;
mod renderer;

use renderer::Renderer;

fn main() {
	// Parse options.
	let yaml = load_yaml!("cli.yml");
	let matches = clap::App::from_yaml(yaml).get_matches();

	// Init logger.
	let verbosity = matches.occurrences_of("verbose") as usize;
	stderrlog::new().verbosity(verbosity).init().unwrap();

	// Number of CPUs.
	//let n = num_cpus::get();
	let n = 1;

	// Create an event scheduler.
	let scheduler = Arc::new(SimpleScheduler::new(n));

	// Connect to the wayland display server.
	let (wayland_display, mut event_queue) = wayland::Client::new().unwrap();

	// Make a wasm display client connection handler.
	let display = wabs::Client::new(wayland_display);

	// Load EGL.
	graphics::load_egl(&display);

	// Create the Pastel context for the GUI.
	let pastel = pastel::Context::new(&display, &scheduler);

	// Create an rendering context.
	let rendering = graphics::context::new(&display);

	// Create a view + controller for the render.
	let (renderer, node) = Renderer::new(&rendering, pastel.scheduler());
	let view = pastel::View::new(node, &pastel);

	// Show the render view.
	view.show();

	// Start everything.
	scheduler.start();
	event_queue.process();
}

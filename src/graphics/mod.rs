pub mod context;
mod render;

pub use context::Context;
pub use self::render::Render;

pub fn load_egl(display: &wabs::Client) {
	// Load OpenGL.
	let egl_display = display.egl_display();

	let mut egl_major = 0;
	let mut egl_minor = 0;
	if !egl::initialize(egl_display, &mut egl_major, &mut egl_minor) {
		error!("EGL initialization failed!");
		std::process::exit(1);
	} else {
		info!("EGL version {}.{}", egl_major, egl_minor);
	}

	// Load OpenGL functions.
	::render::load(|name| {
		let ptr = egl::get_proc_address(name) as *const ();
		if ptr.is_null() {
			None
		} else {
			Some(ptr)
		}
	});
}

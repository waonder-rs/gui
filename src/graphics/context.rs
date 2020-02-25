use std::sync::Arc;
use egl::{EGLDisplay, EGLContext, EGLConfig, EGLSurface};

/// Our context type.
/// Its an EGL context.
pub type Context = Arc<render::EGLContext>;

/// Create a whole new EGL context for the given display.
pub fn new(display: &wabs::Client) -> Context {
	let egl_display = display.egl_display();

	egl::bind_api(egl::EGL_OPENGL_API);
	// let egl_display = egl::get_display(egl::EGL_DEFAULT_DISPLAY).unwrap();

	let mut egl_major = 0;
	let mut egl_minor = 0;
	if !egl::initialize(egl_display, &mut egl_major, &mut egl_minor) {
		error!("EGL initialization failed!");
		std::process::exit(1);
	} else {
		info!("EGL version {}.{}", egl_major, egl_minor);
	}

	let attributes = [
		egl::EGL_RED_SIZE, 8,
		egl::EGL_GREEN_SIZE, 8,
		egl::EGL_BLUE_SIZE, 8,
		egl::EGL_NONE
	];

	let mut config = 0;
	let mut num_config = 0;

	let egl_config = match egl::choose_config(egl_display, &attributes, 1) {
		Some(config) => config,
		None => {
			panic!("error config: {}", egl::get_error());
		}
	};

	let context_attributes = [
		egl::EGL_CONTEXT_MAJOR_VERSION, 4,
		egl::EGL_CONTEXT_MINOR_VERSION, 0,
		egl::EGL_CONTEXT_OPENGL_PROFILE_MASK, egl::EGL_CONTEXT_OPENGL_CORE_PROFILE_BIT,
		egl::EGL_NONE
	];

	let egl_context = egl::create_context(egl_display, egl_config, egl::EGL_NO_CONTEXT, &context_attributes).unwrap();
	Arc::new(render::EGLContext::new(egl_display, egl_config, egl_context))
}

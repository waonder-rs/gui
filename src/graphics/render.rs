use std::sync::Arc;
use egl::EGLSurface;
use render::Surface;

use super::Context;

/// Render surface.
pub struct Render {
	context: Context,
	surface: Option<EGLSurface>
}

impl Render {
	/// Create a new render surface for the given context.
	pub fn new(context: &Context) -> Render {
		Render {
			context: context.clone(),
			surface: None
		}
	}

	pub fn context(&self) -> &Arc<render::EGLContext> {
		&self.context
	}

	pub fn set_egl(&mut self, surface: EGLSurface) {
		if surface.is_null() {
			self.surface = None
		} else {
			self.surface = Some(surface);
		}
	}

	pub fn ready(&self) -> bool {
		self.surface.is_some()
	}
}

impl render::GLSurface<Arc<render::EGLContext>> for Render {
	fn bind(&self) {
		if let Some(handle) = self.surface {
			trace!("eglMakeCurrent({:?}, {:?}, {:?}, {:?})", self.context.display, handle, handle, self.context.handle);
			if egl::make_current(self.context.display, handle, handle, self.context.handle) {
					render::check_errors();
			} else {
					panic!("failed to bind the context")
			}
		} else {
			panic!("no surface")
		}
	}

	fn release(&self) {
		if let Some(handle) = self.surface {
			trace!("eglSwapBuffers(display={:?}, surface={:?})", self.context.display, handle);
			if egl::swap_buffers(self.context.display, handle) {
				render::check_errors();
			} else {
				panic!("failed to swap buffers")
			}
		}
	}
}

unsafe impl Send for Render {}
unsafe impl Sync for Render {}

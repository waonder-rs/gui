use std::sync::Arc;
use egl::{EGLDisplay};
use bottle::{Remote, Handler, Emitter, Sender, Scheduler, SimpleScheduler};
use render::Surface;

use crate::graphics::{Render, Context};

/// Render contoller.
pub struct Renderer {
	/// Render surface.
	surface: Render,

	/// Render view.
	view: Remote<layout::Node>,

	/// Scene to render.
	scene: Arc<engine::Scene<Context>>,

	/// ...
}

impl Renderer {
	pub fn new(context: &Context, scheduler: &Arc<dyn Scheduler>) -> (Remote<Renderer>, Remote<layout::Node>) {
		let scene = Arc::new(engine::Scene::new(context));

		let model = Render::new(context);
		let view = Remote::new(scheduler.next_thread(), layout::Node::egl_canvas(model.context().handle, model.context().config));

		let renderer = Remote::new(scheduler.next_thread(), Renderer {
			surface: model,
			view: view.clone(),
			scene
		});

		Emitter::<layout::event::EGL>::subscribe(&view, &renderer);
		Emitter::<layout::event::Geometry>::subscribe(&view, &renderer);
		Emitter::<layout::event::IO>::subscribe(&view, &renderer);
		(renderer, view)
	}

	/// Do render, is the surface is ready.
	pub fn render(&self) {
		if self.surface.ready() {
			// let r = self.render.write();
			self.scene.render_on(&self.surface);
		}
	}
}

impl Handler<layout::event::EGL> for Renderer {
	fn handle(&mut self, _sender: Option<Sender>, e: layout::event::EGL) {
		use layout::event::EGL::*;
		match e {
			Mapped(egl_surface) => {
				self.surface.set_egl(egl_surface);
				self.render();
			},
			Repaint(egl_surface) => {
				self.surface.set_egl(egl_surface);
				self.render();
			}
		}
	}
}

impl Handler<layout::event::Geometry> for Renderer {
	fn handle(&mut self, _sender: Option<Sender>, e: layout::event::Geometry) {
		use layout::event::Geometry::*;
		match e {
			Resize(new_size) => {
				println!("resized to {}", new_size);
			}
		}
	}
}

impl Handler<layout::event::IO> for Renderer {
	fn handle(&mut self, _sender: Option<Sender>, e: layout::event::IO) {
		use layout::event::IO::*;
		match e {
			Mouse(e) => {
				use layout::event::io::Mouse::*;
				match e {
					Move(pos) => {
						println!("move mouse to {}", pos);
					},
					Button { button, state } => {
						println!("button: {:?} {:?}", button, state);
					}
					Axis { axis, value } => {
						println!("axis: {:?} {:?}", axis, value);
					}
				}
			}
		}
	}
}

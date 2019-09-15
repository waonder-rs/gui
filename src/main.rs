extern crate wabs;
extern crate wabs_wayland as wayland;
extern crate pastel;
extern crate layout;
extern crate bottle;
extern crate render_gl as render;
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
use egl::{EGLDisplay, EGLContext, EGLConfig, EGLSurface};
use wabs::{Client, Window};
use layout::Layout;
use bottle::{Remote, Handler, Emitter, Sender, Scheduler, SimpleScheduler};
use pastel::{view::RemoteView};

struct Render {
    display: EGLDisplay,
    context: EGLContext,
    config: EGLConfig,
    surface: Option<EGLSurface>
}

unsafe impl Send for Render {}
unsafe impl Sync for Render {}

struct Renderer {
    render: Render,
    view: Remote<layout::Node>
}

impl Render {
    fn new(egl_display: EGLDisplay) -> Render {
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

        let context = egl::create_context(egl_display, egl_config, egl::EGL_NO_CONTEXT, &context_attributes).unwrap();

        Render {
            display: egl_display,
            context: context,
            config: egl_config,
            surface: None
        }
    }
}

impl Renderer {
    pub fn new(egl_display: EGLDisplay, scheduler: Arc<dyn Scheduler>) -> (Remote<Renderer>, Remote<layout::Node>) {
        let model = Render::new(egl_display);
        let view = Remote::new(scheduler.next_thread(), layout::Node::egl_canvas(model.context, model.config));
        let renderer = Remote::new(scheduler.next_thread(), Renderer {
            render: model,
            view: view.clone()
        });

        Emitter::<layout::event::EGL>::subscribe(&view, &renderer);
        Emitter::<layout::event::Geometry>::subscribe(&view, &renderer);
        Emitter::<layout::event::IO>::subscribe(&view, &renderer);
        (renderer, view)
    }
}

impl Handler<layout::event::EGL> for Renderer {
    fn handle(&mut self, _sender: Option<Sender>, e: layout::event::EGL) {
        use layout::event::EGL::*;
        match e {
            Mapped(egl_surface) => {
                egl::make_current(self.render.display, egl_surface, egl_surface, self.render.context);

                unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

                egl::swap_buffers(self.render.display, egl_surface);
            },
            Repaint(egl_surface) => {
                egl::make_current(self.render.display, egl_surface, egl_surface, self.render.context);

                unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

                egl::swap_buffers(self.render.display, egl_surface);
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

fn main() {
    // Parse options.
	let yaml = load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    // Init logger.
	let verbosity = matches.occurrences_of("verbose") as usize;
    stderrlog::new().verbosity(verbosity).init().unwrap();

    //let n = num_cpus::get();
    let n = 1;

    let (wayland_display, mut event_queue) = wayland::Client::new().unwrap();
	let display = wabs::Client::new(wayland_display);
    let scheduler = Arc::new(SimpleScheduler::new(n));

    load_egl(&display);

    // create Pastel context.
    let ctx = pastel::Context::new(&display, &scheduler);
    // ctx.style().load(css_load!("style.css"));

    let (renderer, node) = Renderer::new(display.egl_display(), ctx.scheduler().clone());
	let view = pastel::View::new(node, &ctx);

	view.show();

    scheduler.start();
    event_queue.process();
}

fn load_egl(display: &wabs::Client) {
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
    render::load(|name| {
        let ptr = egl::get_proc_address(name) as *const ();
        if ptr.is_null() {
            None
        } else {
            Some(ptr)
        }
    });
}

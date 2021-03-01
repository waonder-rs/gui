use std::{
	sync::Arc,
	convert::TryInto
};
use magma::{
	instance::{
		PhysicalDevice
	},
	device,
	Device,
	Format,
	framebuffer::{
		self,
		RenderPass
	},
	image,
	Swapchain
};
use engine::render;

pub struct Surface<W> {
	window: W,
	device: Arc<Device>,
	render_pass: Arc<RenderPass>,
	swapchain: Swapchain<W>,
}

impl<W> Surface<W> {
	pub fn new(device: &Arc<Device>, presentation_queue: device::Queue, inner: W) -> Self {
		panic!("TODO")
	}
}

impl<W> render::Target for Surface<W> {
	fn device(&self) -> &Arc<Device> {
		&self.device
	}

	fn render_pass(&self) -> &Arc<RenderPass> {
		&self.render_pass
	}
}

fn create_render_pass(device: &Arc<Device>, format: Format) -> Arc<framebuffer::RenderPass> {
	let mut attachments = framebuffer::render_pass::Attachments::new();

	let color_attachment = attachments.add(framebuffer::render_pass::Attachment {
		format,
		samples: 1u8.try_into().unwrap(),
		load: framebuffer::render_pass::LoadOp::Clear,
		store: framebuffer::render_pass::StoreOp::Store,
		stencil_load: framebuffer::render_pass::LoadOp::DontCare,
		stencil_store: framebuffer::render_pass::StoreOp::DontCare,
		initial_layout: image::Layout::Undefined,
		final_layout: image::Layout::PresentSrc
	});

	let subpass = framebuffer::render_pass::SubpassRef {
		color_attachments: &[color_attachment.with_layout(image::Layout::ColorAttachmentOptimal)],
		depth_stencil: None,
		input_attachments: &[],
		resolve_attachments: &[],
		preserve_attachments: &[]
	};

	let mut render_pass = framebuffer::RenderPassBuilder::new(&attachments);
	render_pass.add(subpass);

	Arc::new(render_pass.build(device).expect("unable to build render pass"))
}
use std::marker::PhantomData;
use scene::{
	Scene,
	Id
};
use engine::render;

pub struct Generator {
	// ...
}

impl Generator {
	pub fn new() -> Generator {
		Generator {
			// ...
		}
	}
}

impl render::Generator<wonder::Object> for Generator {
	fn view(&self, object: &wonder::Object) -> engine::View {
		match object {
			wonder::Object::Planet(planet) => {
				panic!("TODO")
			}
		}
	}
}
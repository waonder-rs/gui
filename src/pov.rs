use std::{
	collections::HashSet,
	convert::TryInto
};
use scene::{
	Scene,
	Id
};
use engine::render;

pub struct Pov {
	visible_objects: HashSet<Id<wonder::Object>>
}

impl Pov {
	pub fn new() -> Pov {
		Pov {
			visible_objects: HashSet::new()
		}
	}
}

pub struct VisibleObjects<'a> {
	inner: std::collections::hash_set::Iter<'a, Id<wonder::Object>>
}

impl<'a> Iterator for VisibleObjects<'a> {
	type Item = &'a Id<wonder::Object>;

	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next()
	}
}

impl<E> render::PointOfView<wonder::Object, E> for Pov where for<'a> &'a E: TryInto<&'a scene::Event> {
	type Iter<'a> = VisibleObjects<'a>;

	fn cycle(&mut self, scene: &Scene<wonder::Object, E>) {
		for event in scene.events() {
			if let Ok(event) = event.try_into() {
				match event {
					scene::Event::New(object) => {
						self.visible_objects.insert(scene.id(*object).unwrap());
					},
					scene::Event::Drop(object) => {
						self.visible_objects.remove(&scene.id(*object).unwrap());
					}
				}
			}
		}
	}

	fn visible_objects(&self) -> VisibleObjects {
		panic!("TODO visible objects")
	}
}
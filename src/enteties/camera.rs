use amethyst::{
	prelude::*,
	core::transform::Transform,
	renderer::Camera,
};

use crate::resources::GameConfig;

pub fn initialize_camera(world: &mut World, config: &GameConfig) {
	let mut transform = Transform::default();

	let pixel_size = (config.game_width * config.tile_size, config.game_height * config.tile_size);

	transform.set_translation_xyz(pixel_size.0 * 0.5, pixel_size.1 * 0.5, 1.0);

	world
		.create_entity()
		.with(Camera::standard_2d(pixel_size.0, pixel_size.1))
		.with(transform)
		.build();
}
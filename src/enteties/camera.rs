use amethyst::{
	prelude::*,
	core::transform::Transform,
	renderer::Camera,
};

const GAME_HEIGHT: f32 = 10.0;
const GAME_WIDTH: f32 = 10.0;

const GAME_PIXEL_WIDTH: f32 = 32.0 * GAME_WIDTH;
const GAME_PIXEL_HEIGHT: f32 = 32.0 * GAME_HEIGHT;

pub fn initialize_camera(world: &mut World) {
	let mut transform = Transform::default();

	transform.set_translation_xyz(GAME_PIXEL_WIDTH * 0.5, GAME_PIXEL_HEIGHT * 0.5, 1.0);

	world
		.create_entity()
		.with(Camera::standard_2d(GAME_PIXEL_WIDTH, GAME_PIXEL_WIDTH))
		.with(transform)
		.build();
}
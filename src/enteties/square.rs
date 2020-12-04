use amethyst::{
	prelude::*,
	assets::{Handle},
	renderer::{SpriteSheet, SpriteRender},
	core::transform::Transform,
};

use crate::components::TimeKeeper;

const GAME_HEIGHT: f32 = 10.0;
const GAME_PIXEL_HEIGHT: f32 = 32.0 * GAME_HEIGHT;

pub fn initialize_square(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
	let mut transform = Transform::default();

	transform.set_translation_xyz(GAME_PIXEL_HEIGHT * 0.5, GAME_PIXEL_HEIGHT * 0.5, 0.0);

	let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);

	world.create_entity()
		.with(sprite_render)
		.with(transform)
		.with(TimeKeeper::default())
		.build();
}
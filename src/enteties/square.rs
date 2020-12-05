use amethyst::{
	prelude::*,
	assets::{Handle},
	renderer::{SpriteSheet, SpriteRender},
	core::transform::Transform,
};

use crate::{resources::GameConfig, components::TimeKeeper};


pub fn initialize_square(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, config: &GameConfig) {
	let mut transform = Transform::default();

	let pixel_size = (config.game_width * config.tile_size, config.game_height * config.tile_size);

	transform.set_translation_xyz(pixel_size.0 * 0.5, pixel_size.1 * 0.5, 0.0);

	let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);

	world.create_entity()
		.with(sprite_render)
		.with(transform)
		.with(TimeKeeper::default())
		.build();
}
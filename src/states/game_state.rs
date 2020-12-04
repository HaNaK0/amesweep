use amethyst::{
	SimpleState,
	prelude::*
};

use crate::{enteties::{initialize_camera, initialize_square}, resources::load_sprite_sheet};


//TODO move these to a config somehow
// const GAME_HEIGHT: f32 = 10.0;
// const GAME_WIDTH: f32 = 10.0;

// const GAME_PIXEL_WIDTH: f32 = 32.0 * GAME_WIDTH;
// const GAME_PIXEL_HEIGHT: f32 = 32.0 * GAME_HEIGHT;

pub struct GameState;

impl SimpleState for GameState {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;

		initialize_camera(world);
		let sprite_sheet = load_sprite_sheet(world);
		initialize_square(world, sprite_sheet);
	}
}
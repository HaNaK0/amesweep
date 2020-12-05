use amethyst::{
	SimpleState, 
	utils::application_root_dir, 
	prelude::*
};

use log;

use crate::{
	enteties::{initialize_camera, initialize_square}, 
	resources::{load_sprite_sheet, GameConfig},
};

pub struct GameState;

impl SimpleState for GameState {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;

		let game_config = load_config();

		initialize_camera(world, &game_config);
		let sprite_sheet = load_sprite_sheet(world);
		initialize_square(world, sprite_sheet, &game_config);
		log::info!("End of State Start");
	}
}

fn load_config() -> GameConfig {
	let app_root = application_root_dir();
	let config_path = app_root.unwrap().join("config").join("config.ron");

	GameConfig::load(&config_path).unwrap_or_else(|error| {
			log::error!("Failed to load conifg: {}", error);
            GameConfig::default()
        })
}


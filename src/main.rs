use amethyst::{Application, GameDataBuilder, utils::{
	application_dir, 
	application_root_dir,
}};


mod minesweeper;

fn main() -> amethyst::Result<()> {
	amethyst::start_logger(Default::default());

	let app_root = application_root_dir()?; 
	let display_config_path = app_root.join("config").join("display.ron");

	let game_data = GameDataBuilder::default();

	let asset_dir = app_root.join("assets");
	let mut game = Application::new(
		asset_dir, 
		minesweeper::GameState,
		game_data
	)?;

	game.run();
	Ok(())
}
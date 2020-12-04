use amethyst::{Application, GameDataBuilder, core::TransformBundle, renderer::{RenderFlat2D, RenderToWindow, RenderingBundle, types::DefaultBackend}, utils::{
	application_root_dir,
}};

mod minesweeper;
mod systems;

use crate::minesweeper::GameState;

fn main() -> amethyst::Result<()> {
	amethyst::start_logger(Default::default());

	let app_root = application_root_dir()?; 
	let display_config_path = app_root.join("config").join("display.ron");

	let game_data = GameDataBuilder::default()
		.with_bundle(
			RenderingBundle::<DefaultBackend>::new()
				.with_plugin(
					RenderToWindow::from_config_path(display_config_path)?
						.with_clear([5. / 255., 5. / 255., 5. / 255., 1.0])
				)
				.with_plugin(RenderFlat2D::default())
		)?
		.with_bundle(TransformBundle::new())?
		.with(systems::SquareTestSystem, "test_square", &[]);

	let asset_dir = app_root.join("assets");
	let mut game = Application::new(
		asset_dir, 
		GameState,
		game_data
	)?;

	game.run();
	Ok(())
}
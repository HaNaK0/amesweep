use std::time::Duration;

use amethyst::{
	assets::{AssetStorage, Loader, Handle}, 
	core::transform::Transform, 
	ecs::Component,
	ecs::VecStorage, 
	prelude::*, 
	renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture}};

//TODO move these to a config somehow
const GAME_HEIGHT: f32 = 10.0;
const GAME_WIDTH: f32 = 10.0;

const GAME_PIXEL_WIDTH: f32 = 32.0 * GAME_WIDTH;
const GAME_PIXEL_HEIGHT: f32 = 32.0 * GAME_HEIGHT;

//Components 
#[derive(Default)]
pub struct TimeKeeper {
	pub time_passed: Duration
}

impl Component for TimeKeeper {
	type Storage = VecStorage<Self>;
}

pub struct GameState;

impl SimpleState for GameState {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;

		initialize_camera(world);
		let sprite_sheet = load_sprite_sheet(world);
		initialize_square(world, sprite_sheet);
	}
}

fn initialize_camera(world: &mut World) {
	let mut transform = Transform::default();

	transform.set_translation_xyz(GAME_PIXEL_WIDTH * 0.5, GAME_PIXEL_HEIGHT * 0.5, 1.0);

	world
		.create_entity()
		.with(Camera::standard_2d(GAME_PIXEL_WIDTH, GAME_PIXEL_WIDTH))
		.with(transform)
		.build();
}

fn initialize_square(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
	let mut transform = Transform::default();

	transform.set_translation_xyz(GAME_PIXEL_HEIGHT * 0.5, GAME_PIXEL_HEIGHT * 0.5, 0.0);

	let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);

	world.create_entity()
		.with(sprite_render)
		.with(transform)
		.with(TimeKeeper::default())
		.build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
	let texture_handle = {
		let loader = world.read_resource::<Loader>();
		let texture_storage = world.read_resource::<AssetStorage<Texture>>();
		loader.load("sprites/numbers.png", ImageFormat::default(), (), &texture_storage)
	};

	let loader = world.read_resource::<Loader>();
	let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
	loader.load(
		"sprites/numbers.ron", 
		SpriteSheetFormat(texture_handle), 
		(), 
		&sprite_sheet_store
	)
}

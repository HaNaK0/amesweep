use amethyst::{
	prelude::*,
	assets::{AssetStorage, Loader, Handle},
	renderer::{SpriteSheet, SpriteSheetFormat, ImageFormat, Texture}
};

pub fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
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
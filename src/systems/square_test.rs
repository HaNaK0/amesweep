use std::time::Duration;

use amethyst::{
	ecs::{
		Join,
		Read,
		System, 
		WriteStorage,
	}, 
	renderer::SpriteRender, 
	core::{
		timing::Time,
	},
};

use crate::components::TimeKeeper;

pub struct SquareTestSystem ;

impl<'s> System<'s> for SquareTestSystem {
	type SystemData = (
		WriteStorage<'s, SpriteRender>,
		WriteStorage<'s, TimeKeeper>,
		Read<'s, Time>,
	);

	fn run(&mut self, (mut sprite_render, mut time_keeper, time): Self::SystemData) {
		for (sprite_render, time_keeper) in (&mut sprite_render, &mut time_keeper).join() {
			// Update time
			time_keeper.time_passed += time.delta_time();

			//Every second, update the sprite to whoe the next number
			if time_keeper.time_passed.as_secs() >=  1 {
				let sprite_number = sprite_render.sprite_number + 1;
				
				sprite_render.sprite_number = if sprite_number < 9 { sprite_number } else { 0 };

				println!("Sprite number{}", sprite_render.sprite_number);
				time_keeper.time_passed = Duration::from_secs(0);
			}
		}
    }
}
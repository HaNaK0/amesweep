use std::time::Duration;

use amethyst::{
	ecs::{
		Component,
		VecStorage,
	}
};

#[derive(Default)]
pub struct TimeKeeper {
	pub time_passed: Duration
}

impl Component for TimeKeeper {
	type Storage = VecStorage<Self>;
}


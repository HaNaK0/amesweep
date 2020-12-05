use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct GameConfig {
	pub game_width: f32,
	pub game_height: f32,
	pub tile_size: f32,
}

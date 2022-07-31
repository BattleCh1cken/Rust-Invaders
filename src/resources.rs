use bevy::prelude::{Handle, Image};
//Resources
pub struct LevelData {
	pub level_number: i32,
	pub is_new_level: bool,
	pub difficulty: f32,
	pub queued_enemies: i32,
}

pub struct WinSize {
	pub w: f32,
	pub h: f32,
}

pub struct GameTextures {
	pub player: Handle<Image>,
	pub player_laser: Handle<Image>,
	pub enemy: Handle<Image>,
}

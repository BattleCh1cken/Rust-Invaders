use crate::{
	components::{Enemy, Movable, SpriteSize, Stats, Velocity},
	resources::LevelData,
	GameTextures, WinSize, ENEMY_SIZE, ENEMY_SPRITE, SPRITE_SCALE,
};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
		// app.add_startup_system_to_stage(StartupStage::PostStartup, enemy_spawn_system);
		app.add_system(enemy_spawn_system);
	}
}

fn enemy_spawn_system(
	mut commands: Commands,
	game_textures: Res<GameTextures>,
	win_size: Res<WinSize>,
	mut level_data: ResMut<LevelData>,
) {
	while level_data.queued_enemies > 0 {
		let mut rng = thread_rng();
		let w_span = win_size.w / 2. - 100.;
		let h_span = win_size.h / 2. - 100.;
		let x = rng.gen_range(-w_span..w_span);
		let y = rng.gen_range(-h_span..h_span);
		commands
			.spawn_bundle(SpriteBundle {
				texture: game_textures.enemy.clone(),
				transform: Transform {
					translation: Vec3::new(x, y, 10.),
					scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 10.),
					..Default::default()
				},
				..Default::default()
			})
			.insert(Velocity { x: 0., y: 0. })
			.insert(SpriteSize::from(ENEMY_SIZE))
			.insert(Enemy)
			.insert(Stats { health: 4 })
			.insert(Movable { auto_despawn: false });
		level_data.queued_enemies += -1;
		// println!("{}",level_data.queued_enemies);
	}
}

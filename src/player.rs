use crate::components::{FromPlayer, Laser, Movable, Player, SpriteSize, Velocity};
use crate::{
	GameTextures, WinSize, BASE_SPEED, PLAYER_LASER_SIZE, PLAYER_SIZE, SPRITE_SCALE, TIME_STEP,
};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
			.add_system(player_keyboard_input_system)
			.add_system(player_fire_system);
	}
}

fn player_spawn_system(
	mut commands: Commands,
	game_textures: Res<GameTextures>,
	win_size: Res<WinSize>,
) {
	let bottom = -win_size.h / 2.;

	commands
		.spawn_bundle(SpriteBundle {
			texture: game_textures.player.clone(),
			transform: Transform {
				translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.),
				scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
				..Default::default()
			},
			..Default::default()
		})
		.insert(Player)
		.insert(Velocity { x: 0., y: 0. })
		.insert(SpriteSize::from(PLAYER_SIZE))
		.insert(Movable { auto_despawn: false });
}

fn player_keyboard_input_system(
	kb: Res<Input<KeyCode>>,
	mut query: Query<&mut Velocity, With<Player>>,
) {
	for key in kb.get_pressed() {}

	if let Ok(mut velocity) = query.get_single_mut() {
		velocity.x = if kb.pressed(KeyCode::Left) {
			-1.
		} else if kb.pressed(KeyCode::Right) {
			1.
		} else {
			0.
		};
	}
}

fn player_fire_system(
	mut commands: Commands,
	game_textures: Res<GameTextures>,
	kb: Res<Input<KeyCode>>,
	query: Query<&Transform, With<Player>>,
) {
	if let Ok(player_tf) = query.get_single() {
		if kb.just_pressed(KeyCode::Space) {
			let (x, y) = (player_tf.translation.x, player_tf.translation.y);
			commands
				.spawn_bundle(SpriteBundle {
					texture: game_textures.player_laser.clone(),
					transform: Transform {
						translation: Vec3::new(x, y, 10.),
						scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
						..Default::default()
					},
					..Default::default()
				})
				.insert(Velocity { x: 0., y: 1. })
				.insert(SpriteSize::from(PLAYER_LASER_SIZE))
				.insert(Laser)
				.insert(FromPlayer)
				.insert(Movable { auto_despawn: true });
		}
	}
}

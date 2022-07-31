#![allow(unused, dead_code)]
use bevy::prelude::*;
use bevy::{math::Vec3Swizzles, sprite::collide_aabb::collide};
mod components;
mod enemy;
mod player;
use components::{Enemy, FromPlayer, Laser, Movable, Player, SpriteSize, Stats, Velocity};
mod constants;
use constants::*;
mod resources;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use resources::*;

fn main() {
	App::new()
		.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
		.insert_resource(WindowDescriptor {
			title: "Rust Invaders".to_string(),
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
		.add_plugin(PlayerPlugin)
		.add_plugin(EnemyPlugin)
		.add_startup_system(setup)
		.add_system(level_handling_system)
		.add_system(movement_system)
		.add_system(player_laser_hit_enemy_system)
        .add_system(cleanup_dead_enemy_system)
		.run()
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut windows: ResMut<Windows>,
) {
	//Camera
	commands.spawn_bundle(Camera2dBundle::default());
	//Get window size
	let window = windows.get_primary_mut().unwrap();
	let (win_w, win_h) = (window.width(), window.height());
	//window resource
	let win_size = WinSize { w: win_w, h: win_h };
	commands.insert_resource(win_size);

	//texture resource
	let game_textures = GameTextures {
		player: asset_server.load(PLAYER_SPRITE),
		player_laser: asset_server.load(PLAYER_LASER_SPRITE),
		enemy: asset_server.load(ENEMY_SPRITE),
	};
	commands.insert_resource(game_textures);

	//level resource
	let mut level_data = LevelData {
		level_number: 0,
		is_new_level: true,
		difficulty: 1.,
		queued_enemies: 0,
	};
	commands.insert_resource(level_data);
}

fn level_handling_system(mut level_data: ResMut<LevelData>, query: Query<&Enemy>) {
	// level_data.queued_enemies = 3;
	if level_data.is_new_level {
		//Start the new level
		println!("Starting new level");
		level_data.level_number += 1;
		println!("{}", level_data.level_number);
		match level_data.level_number {
			1 => level_data.queued_enemies = 1,
			2 => level_data.queued_enemies = 2,
			3 => level_data.queued_enemies = 3,
			n => level_data.queued_enemies = n,
		}
		level_data.is_new_level = false;
	} else if query.is_empty() {
		//Level is over, start a new one
		level_data.is_new_level = true;
	}

	// println!("{}", level_data.queued_enemies);
}

fn movement_system(
	mut commands: Commands,
	win_size: Res<WinSize>,
	mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
	for (entity, velocity, mut transform, movable) in query.iter_mut() {
		let translation = &mut transform.translation;
		translation.x += velocity.x * TIME_STEP * BASE_SPEED;
		translation.y += velocity.y * TIME_STEP * BASE_SPEED;
		if movable.auto_despawn {
			const MARGIN: f32 = 200.;
			if translation.y > win_size.h / 2. + MARGIN
				|| -translation.y < -win_size.h / 2. - MARGIN
				|| translation.x > win_size.w / 2. + MARGIN
				|| -translation.x < -win_size.w / 2. - MARGIN
			{
				commands.entity(entity).despawn();
			}
		}
	}
}

fn player_laser_hit_enemy_system(
	mut commands: Commands,
	laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
	mut enemy_query: Query<(Entity, &Transform, &SpriteSize, &mut Stats), (With<Enemy>)>,
) {
	for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
		let laser_scale = Vec2::from(laser_tf.scale.xy());

		for (enemy_entity, enemy_tf, enemy_size, mut enemy_stats) in enemy_query.iter_mut() {
			let enemy_scale = Vec2::from(enemy_tf.scale.xy());

			//collision logic
			let collision = collide(
				laser_tf.translation,
				laser_size.0 * laser_scale,
				enemy_tf.translation,
				enemy_size.0 * enemy_scale,
			);
			if let Some(_) = collision {
				println!("{}", enemy_stats.health);
				// commands.entity(enemy_entity).despawn();
				enemy_stats.health -= 1;

				println!("{}", enemy_stats.health);

				commands.entity(laser_entity).despawn();
				println!("collision detected");
			}
		}
	}
}

fn cleanup_dead_enemy_system(
	mut commands: Commands,
	query: Query<(Entity, &Stats), (With<Enemy>)>,
) {
	for (enemy_entity, enemy_stats) in query.iter() {
		if enemy_stats.health <= 0 {
            commands.entity(enemy_entity).despawn();
        }
	}
}

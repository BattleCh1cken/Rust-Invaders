#![allow(unused, dead_code)]
use bevy::prelude::*;
mod player;
use player::PlayerPlugin;
mod components;
use components::{Movable, Player, Velocity};

mod enemy;

//Constant values

//sprite constants
const PLAYER_SPRITE: &str = "player_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const PLAYER_LASER_SPRITE: &str = "player_laser.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);
const ENEMY_SPRITE: &str = "enemy.png";
const ENEMY_SIZE: (f32, f32) = (100., 100.);

const SPRITE_SCALE: f32 = 1.5;

//game constants
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

//Resources

pub struct WinSize {
	pub w: f32,
	pub h: f32,
}

pub struct GameTextures {
	player: Handle<Image>,
	player_laser: Handle<Image>,
	enemy: Handle<Image>,
}

fn main() {
	App::new()
		.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
		.insert_resource(WindowDescriptor {
			title: "Rust Invaders".to_string(),
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
		.add_plugin(PlayerPlugin)
		.add_startup_system(setup)
		.add_system(movement_system) // .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
		.run()
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut windows: ResMut<Windows>,
) {
	//Camera
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	//Get window size
	let window = windows.get_primary_mut().unwrap();
	let (win_w, win_h) = (window.width(), window.height());
	//window resource
	let win_size = WinSize { w: win_w, h: win_h };
	commands.insert_resource(win_size);

	//texture resoure
	let game_textures = GameTextures {
		player: asset_server.load(PLAYER_SPRITE),
		player_laser: asset_server.load(PLAYER_LASER_SPRITE),
		enemy: asset_server.load(ENEMY_SPRITE),
	};
	commands.insert_resource(game_textures);
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

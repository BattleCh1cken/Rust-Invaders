use bevy::prelude::*;

use crate::{GameTextures, ENEMY_SIZE, ENEMY_SPRITE};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {}
}

fn enemy_spawn_system(mut commands: Commands, game_textures: Res<GameTextures>) {
	commands.spawn_bundle({ SpriteBundle {
        texture: game_textures.enemy.clone(),
        ..Default::default() } });
}

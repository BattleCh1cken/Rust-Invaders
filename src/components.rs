use bevy::prelude::*;

//common components
#[derive(Component, Debug)]
pub struct Velocity {
  pub x: f32,
  pub y: f32,
}

#[derive(Component)]
pub struct Movable {
  pub auto_despawn: bool,
}
//player components
#[derive(Component)]
pub struct Player;

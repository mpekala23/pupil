pub mod consts;

use bevy::prelude::*;

/// A simple hitbox
/// Position is relative to the parent's transform
#[derive(Component)]
pub struct Hitbox {
    pub pos: Vec2,
    pub size: Vec2,
}

/// Does this component move?
#[derive(Component)]
pub struct Moveable;

pub fn physics_setup(mut commands: Commands) {}

pub fn physics_update(mut commands: Commands) {}

pub fn register_physics(app: &mut App) {
    app.add_systems(Startup, physics_setup)
        .add_systems(FixedUpdate, physics_update);
}

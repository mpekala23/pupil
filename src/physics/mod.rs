pub mod consts;

use bevy::prelude::*;

use self::consts::GRAVITY;

/// A simple hitbox
/// Position is relative to the parent's transform
#[derive(Component)]
pub struct Hitbox {
    pub pos: Vec2,
    pub size: Vec2,
}

// Simple velocity
#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

/// Does this component move?
#[derive(Component)]
pub struct Moveable;

pub fn physics_setup(mut commands: Commands) {}

pub fn physics_gravity(time: Res<Time>, mut query: Query<&mut Velocity, With<Moveable>>) {
    for mut velocity in query.iter_mut() {
        velocity.y -= GRAVITY * time.delta_seconds();
    }
}

pub fn physics_move(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform), With<Moveable>>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
        if transform.translation.y < 0.0 {
            transform.translation.y = 0.0;
        }
    }
}

pub fn register_physics(app: &mut App) {
    app.add_systems(Startup, physics_setup)
        .add_systems(FixedUpdate, physics_gravity)
        .add_systems(FixedUpdate, physics_move.after(physics_gravity));
}

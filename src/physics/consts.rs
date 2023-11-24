use bevy::ecs::component::Component;

pub const GRAVITY: f32 = 980.0;
pub const COLLISION_THRESHOLD: f32 = 0.001;
#[derive(Component, PartialEq)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down,
}

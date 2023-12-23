//! A roll is a group of armadillos
//! This is where logic exists for spawning many agents at the same time
//! and measuring fitness, spawning the next generation, etc.

use bevy::{prelude::*, utils::HashMap};

#[derive(Component)]
pub struct Roll {
    size: u32,
}

#[derive(Bundle)]
pub struct RollBundle {
    roll: Roll,
}

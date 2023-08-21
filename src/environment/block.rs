use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};

use super::consts::*;
use crate::{agent::eye::Seeable, physics::Hitbox};

#[derive(Component)]
pub struct Block;

#[derive(Bundle)]
pub struct BlockBundle {
    _block: Block,
    _seeable: Seeable,
    spatial: SpatialBundle,
    sprite: Sprite,
    texture: Handle<Image>,
    hitbox: Hitbox,
}
impl BlockBundle {
    pub fn new(pos: Vec2, size: Vec2) -> BlockBundle {
        BlockBundle {
            _block: Block,
            _seeable: Seeable,
            spatial: SpatialBundle {
                transform: Transform {
                    translation: pos.extend(0.0),
                    scale: size.extend(1.0),
                    ..default()
                },
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                ..default()
            },
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            hitbox: Hitbox {
                pos: Vec2 { x: 0.0, y: 0.0 },
                size,
            },
        }
    }
}

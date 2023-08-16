use bevy::prelude::*;

use super::consts::*;
use crate::physics::Hitbox;

#[derive(Bundle)]
struct BlockBundle {
    sprite: SpriteBundle,
    hitbox: Hitbox,
}
impl BlockBundle {
    fn new(size: Vec2) -> BlockBundle {
        BlockBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 0.0),
                    ..default()
                },
                transform: Transform {
                    scale: size.extend(0.0),
                    ..default()
                },
                ..default()
            },
            hitbox: Hitbox {
                pos: Vec2 { x: 0.0, y: 0.0 },
                size,
            },
        }
    }
}

#[derive(Component)]
pub struct Block;
impl Block {
    pub fn spawn(mut commands: Commands, pos: Vec2, size: Vec2) {
        commands
            .spawn((
                SpatialBundle {
                    transform: Transform {
                        translation: pos.extend(0.0),
                        ..default()
                    },
                    visibility: Visibility::Visible,
                    ..default()
                },
                Block,
            ))
            .with_children(|parent| {
                parent.spawn(BlockBundle::new(size));
            });
    }
}

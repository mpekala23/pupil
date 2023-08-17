pub mod consts;

use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE, utils::HashMap};
use consts::*;

use crate::physics::{consts::GRAVITY, Hitbox, Moveable, Velocity};

#[derive(Component)]
pub struct Agent;

#[derive(Component)]
pub struct Observation {
    senses: HashMap<String, f32>,
}

#[derive(Component)]
pub struct Brain {}

#[derive(Bundle)]
pub struct AgentBundle {
    _agent: Agent,
    _movable: Moveable,
    spatial: SpatialBundle,
    sprite: Sprite,
    texture: Handle<Image>,
    observation: Observation,
    hitbox: Hitbox,
    velocity: Velocity,
}
impl AgentBundle {
    fn new(pos: Vec2, size: Vec2) -> AgentBundle {
        AgentBundle {
            _agent: Agent,
            _movable: Moveable,
            spatial: SpatialBundle {
                transform: Transform {
                    //translation: pos.extend(0.0),
                    scale: size.extend(1.0),
                    ..default()
                },
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.5, 1.0, 0.3),
                ..default()
            },
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            observation: Observation {
                senses: HashMap::new(),
            },
            hitbox: Hitbox {
                pos: Vec2 { x: 0.0, y: 0.0 },
                size,
            },
            velocity: Velocity { x: 0.0, y: 0.0 },
        }
    }
}

pub fn agent_setup(mut commands: Commands) {
    commands.spawn(AgentBundle::new(
        Vec2 { x: 0.0, y: 0.0 },
        Vec2 { x: 60.0, y: 60.0 },
    ));
}

pub fn agent_update(mut query: Query<&mut Transform, With<Agent>>) {
    if query.is_empty() {
        return;
    }
    let mut _agent = query.single_mut();
}

pub fn agent_move(mut query: Query<&mut Velocity, With<Agent>>, input: Res<Input<KeyCode>>) {
    if query.is_empty() {
        return;
    };
    let mut velocity = query.single_mut();
    // Horizontal motion
    if input.any_pressed([KeyCode::A, KeyCode::Left]) {
        velocity.x = -MOVE_SPEED;
    } else if input.any_pressed([KeyCode::D, KeyCode::Right]) {
        velocity.x = MOVE_SPEED;
    } else {
        velocity.x = 0.0;
    }
    // Vertical motion
    if input.just_pressed(KeyCode::W) {
        velocity.y = GRAVITY / 2.0;
    }
}

pub fn register_agent(app: &mut App) {
    app.add_systems(Startup, agent_setup)
        .add_systems(FixedUpdate, agent_update)
        .add_systems(FixedUpdate, agent_move);
}

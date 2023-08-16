pub mod consts;

use bevy::{prelude::*, utils::HashMap};
use consts::*;

use crate::physics::{Hitbox, Moveable, Velocity};

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
    sprite: SpriteBundle,
    observation: Observation,
    hitbox: Hitbox,
    velocity: Velocity,
    _movable: Moveable,
}
impl AgentBundle {
    fn new(pos: Vec2, size: Vec2) -> AgentBundle {
        AgentBundle {
            _agent: Agent,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.5, 1.0, 0.3),
                    ..default()
                },
                transform: Transform {
                    scale: size.extend(1.0),
                    ..default()
                },
                ..default()
            },
            observation: Observation {
                senses: HashMap::new(),
            },
            hitbox: Hitbox {
                pos: Vec2 { x: 0.0, y: 0.0 },
                size,
            },
            velocity: Velocity { x: 0.0, y: 0.0 },
            _movable: Moveable,
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

pub fn agent_move(
    mut query: Query<&mut Transform, With<Agent>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    if query.is_empty() {
        return;
    };
    let mut agent = query.single_mut();
    if input.any_pressed([KeyCode::A, KeyCode::Left]) {
        agent.translation.x -= MOVE_SPEED * time.delta_seconds();
    } else if input.any_pressed([KeyCode::D, KeyCode::Right]) {
        agent.translation.x += MOVE_SPEED * time.delta_seconds();
    }
}

pub fn register_agent(app: &mut App) {
    app.add_systems(Startup, agent_setup)
        .add_systems(FixedUpdate, agent_update)
        .add_systems(FixedUpdate, agent_move);
}

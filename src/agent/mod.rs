pub mod consts;

use bevy::{prelude::*, utils::HashMap};
use consts::*;

#[derive(Component)]
pub struct Agent;

#[derive(Component)]
pub struct Observation {
    senses: HashMap<String, f32>,
}

#[derive(Component)]
pub struct Brain {}

#[derive(Component)]
pub struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Bundle)]
pub struct AgentBundle {
    sprite: SpriteBundle,
    velocity: Velocity,
    input: Observation,
    _agent: Agent,
}
impl AgentBundle {
    fn new(pos: Vec2) -> AgentBundle {
        AgentBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 1.0, 0.3),
                    ..default()
                },
                transform: Transform {
                    translation: pos.extend(0.0),
                    scale: Vec3::new(60.0, 60.0, 1.0),
                    ..default()
                },
                ..default()
            },
            velocity: Velocity { x: 0.0, y: 0.0 },
            input: Observation {
                senses: HashMap::new(),
            },
            _agent: Agent,
        }
    }
}

pub fn agent_setup(mut commands: Commands) {
    commands.spawn(AgentBundle::new(Vec2 { x: 0.0, y: 50.0 }));
}

pub fn agent_update(mut query: Query<&mut Transform, With<Agent>>) {
    if query.is_empty() {
        return;
    }
    let mut agent = query.single_mut();
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

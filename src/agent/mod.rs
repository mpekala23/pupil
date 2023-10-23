pub mod brain;
pub mod consts;
pub mod eye;

use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};
use consts::*;

use self::eye::{register_eye, EyeBundle};
use crate::physics::{consts::GRAVITY, Hitbox, Moveable, Velocity};

#[derive(Component)]
pub struct Agent;

#[derive(Component)]
pub struct Brain {}

#[derive(Component, Debug)]
pub struct Senses {
    data: Vec<Option<f32>>,
}

#[derive(Bundle)]
pub struct AgentBundle {
    _agent: Agent,
    _movable: Moveable,
    spatial: SpatialBundle,
    sprite: Sprite,
    texture: Handle<Image>,
    senses: Senses,
    hitbox: Hitbox,
    velocity: Velocity,
}
impl AgentBundle {
    fn new(pos: Vec2, size: Vec2, num_senses: usize) -> AgentBundle {
        AgentBundle {
            _agent: Agent,
            _movable: Moveable,
            spatial: SpatialBundle {
                transform: Transform {
                    translation: pos.extend(0.0),
                    scale: size.extend(1.0),
                    ..default()
                },
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.1, 0.3, 0.1),
                ..default()
            },
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            senses: Senses {
                data: vec![None; num_senses],
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
    let id = commands
        .spawn(AgentBundle::new(
            Vec2 { x: 0.0, y: 200.0 },
            Vec2 { x: 60.0, y: 60.0 },
            2,
        ))
        .id();
    commands.spawn(EyeBundle::new(
        id,
        0,
        Vec2 { x: 0.0, y: 0.0 },
        Vec2 { x: 100.0, y: 10.0 },
        -3.1415926 / 4.0,
    ));
    commands.spawn(EyeBundle::new(
        id,
        1,
        Vec2 { x: 0.0, y: 0.0 },
        Vec2 { x: 100.0, y: 40.0 },
        3.1415926,
    ));
}

pub fn agent_update(mut query: Query<(&mut Transform, &Senses), With<Agent>>) {
    if query.is_empty() {
        return;
    }
    let (_agent, senses) = query.single_mut();
    println!("{:?}", senses);
}

pub fn agent_move(mut query: Query<&mut Velocity, With<Agent>>, input: Res<Input<KeyCode>>) {
    if query.is_empty() {
        return;
    };
    let mut velocity = query.single_mut();
    // Horizontal motion
    if input.any_pressed([KeyCode::A, KeyCode::Left]) {
        velocity.x -= X_ACCELERATION;
    } else if input.any_pressed([KeyCode::D, KeyCode::Right]) {
        velocity.x += X_ACCELERATION;
    } else {
        velocity.x *= 0.92;
    }
    if velocity.x.abs() > MAX_X_MOVE_SPEED {
        velocity.x = if velocity.x > 0.0 { 1.0 } else { -1.0 } * MAX_X_MOVE_SPEED;
    }
    if velocity.x.abs() < 0.1 {
        velocity.x = 0.0;
    }
    // Vertical motion
    if input.just_pressed(KeyCode::W) {
        velocity.y = GRAVITY / 2.0;
    }
}

pub fn register_agent(app: &mut App) {
    app.add_systems(Startup, agent_setup)
        .add_systems(Update, agent_update)
        .add_systems(Update, agent_move);
    register_eye(app);
}

pub mod brain;
pub mod consts;
pub mod eye;

use bevy::prelude::*;
use consts::*;

use self::eye::{register_eye, EyeBundle};
use crate::animation::{Animatable, AnimationManager, AnimationRoot, AnimationVal};
use crate::physics::{consts::GRAVITY, Hitbox, Moveable, Velocity};

#[derive(Component)]
pub struct Agent;

#[derive(Component)]
pub struct Brain {}

#[derive(Component, Debug)]
pub struct Senses {
    data: Vec<Option<f32>>,
}

#[derive(Clone, Hash, Eq, PartialEq, Component)]
pub enum AgentAnimState {
    Idle,
    Walk,
}
impl Animatable for AgentAnimState {}

#[derive(Bundle)]
pub struct AgentBundle {
    _agent: Agent,
    _movable: Moveable,
    spatial: SpatialBundle,
    anim_state: AnimationVal<AgentAnimState>,
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
            anim_state: AnimationVal {
                state: AgentAnimState::Idle,
                invert_x: true,
                invert_y: false,
            },
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

pub fn agent_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let id = commands
        .spawn(AgentBundle::new(
            Vec2 { x: 0.0, y: 200.0 },
            Vec2 { x: 64.0, y: 64.0 },
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
    commands.spawn(AnimationManager::<AgentAnimState>::new(
        id,
        &vec![
            AnimationRoot::<AgentAnimState> {
                state: AgentAnimState::Idle,
                filename: "sprites/narf/Idle.png".to_string(),
                width: 32,
                height: 32,
                length: 5,
            },
            AnimationRoot::<AgentAnimState> {
                state: AgentAnimState::Walk,
                filename: "sprites/narf/Walk.png".to_string(),
                width: 32,
                height: 32,
                length: 8,
            },
        ],
        &asset_server,
        &mut texture_atlases,
    ));
}

pub fn agent_update(mut query: Query<(&mut Transform, &Senses), With<Agent>>) {
    if query.is_empty() {
        return;
    }
    let (_agent, senses) = query.single_mut();
}

pub fn agent_anim_update(
    mut query: Query<(&Velocity, &mut AnimationVal<AgentAnimState>), With<Agent>>,
) {
    for (vel, mut anim_val) in query.iter_mut() {
        if vel.x.abs() > 0.3 {
            anim_val.state = AgentAnimState::Walk;
            anim_val.invert_x = vel.x < 0.0;
        } else {
            anim_val.state = AgentAnimState::Idle;
        }
    }
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
        velocity.x *= 0.82;
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
        .add_systems(Update, agent_move)
        .add_systems(Update, agent_anim_update);
    register_eye(app);
}

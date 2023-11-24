pub mod brain;
pub mod consts;
pub mod eye;

use bevy::prelude::*;
use consts::*;

use self::eye::{register_eye, EyeBundle, SeeBox};
use crate::animation::{Animatable, AnimationManager, AnimationRoot, AnimationVal};
use crate::physics::consts::Dir;
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
    InAir,
}
impl Animatable for AgentAnimState {}

#[derive(Bundle)]
pub struct AgentBundle {
    _agent: Agent,
    _movable: Moveable,
    dir: Dir,
    anim_state: AnimationVal<AgentAnimState>,
    senses: Senses,
    hitbox: Hitbox,
    velocity: Velocity,
}
impl AgentBundle {
    pub fn new(pos: Vec2, size: Vec2, num_senses: usize) -> AgentBundle {
        AgentBundle {
            _agent: Agent,
            _movable: Moveable,
            dir: Dir::Right,
            anim_state: AnimationVal {
                state: AgentAnimState::Idle,
                invert_x: false,
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

pub fn spawn_agent(
    mut commands: Commands,
    pos: Vec2,
    eye_info: Vec<SeeBox>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let size = Vec2 { x: 64.0, y: 64.0 };
    let id = commands
        .spawn(AgentBundle::new(pos, size.clone(), eye_info.len()))
        .id();
    for (ix, see_box) in eye_info.into_iter().enumerate() {
        let eye_id = commands
            .spawn(EyeBundle::new(ix, see_box.pos, see_box.size, see_box.angle))
            .id();
        commands.entity(id).push_children(&[eye_id]);
    }
    commands.entity(id).insert((
        AnimationManager::<AgentAnimState>::new(
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
                AnimationRoot::<AgentAnimState> {
                    state: AgentAnimState::InAir,
                    filename: "sprites/narf/InAir.png".to_string(),
                    width: 32,
                    height: 32,
                    length: 1,
                },
            ],
            &asset_server,
            &mut texture_atlases,
        ),
        SpriteSheetBundle {
            transform: Transform {
                translation: pos.extend(0.0),
                ..default()
            },
            ..default()
        },
    ));
}

pub fn delete_all_agents(commands: &mut Commands, agents_query: Query<Entity, With<Agent>>) {
    for entity in agents_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// pub fn agent_setup(
//     commands: Commands,
//     asset_server: Res<AssetServer>,
//     texture_atlases: ResMut<Assets<TextureAtlas>>,
// ) {
//     spawn_agent(
//         commands,
//         vec![SeeBox {
//             pos: Vec2 { x: 0.0, y: 0.0 },
//             size: Vec2 { x: 100.0, y: 10.0 },
//             angle: -3.1415926 / 4.0,
//             invert_x: false,
//         }],
//         asset_server,
//         texture_atlases,
//     );
// }

pub fn agent_update(mut query: Query<(&mut Transform, &Senses), With<Agent>>) {
    if query.is_empty() {
        return;
    }
    let (_agent, senses) = query.single_mut();
    println!("{:?}", senses);
    println!("{:?}", _agent.translation);
}

pub fn agent_anim_update(
    mut query: Query<(&Velocity, &mut Dir, &mut AnimationVal<AgentAnimState>), With<Agent>>,
) {
    for (vel, mut dir, mut anim_val) in query.iter_mut() {
        // The agent animation state machine! Huzzah!
        if vel.y.abs() > 15.0 {
            // Assume we're in the air
            anim_val.state = AgentAnimState::InAir;
            if vel.x.abs() > 0.1 {
                anim_val.invert_x = vel.x < 0.0;
            }
        } else {
            if vel.x.abs() > MAX_X_MOVE_SPEED * 0.5 {
                anim_val.state = AgentAnimState::Walk;
                anim_val.invert_x = vel.x < 0.0;
            } else {
                anim_val.state = AgentAnimState::Idle;
            }
        }
        if vel.x.abs() > 0.1 {
            if vel.x > 0.0 {
                *dir = Dir::Right;
            } else {
                *dir = Dir::Left;
            }
        }
        // Left / right driven by agent state, since it also affects eyes its weird to
        // keep it embedded in the animation manager
        anim_val.invert_x = *dir == Dir::Left;
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
    app.add_systems(Update, agent_update)
        .add_systems(Update, agent_move)
        .add_systems(Update, agent_anim_update);
    register_eye(app);
}

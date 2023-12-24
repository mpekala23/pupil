pub mod brain;
pub mod consts;
pub mod eye;
pub mod roll;

use bevy::prelude::*;
use consts::*;

use self::eye::{register_eye, EyeBundle, SeeBox};
use self::roll::register_roll;
use crate::animation::{
    Animatable, AnimationManager, AnimationRoot, AnimationVal,
};
use crate::environment::reward::Judgeable;
use crate::meta::consts::{WINDOW_HEIGHT, WINDOW_WIDTH};
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

#[derive(Clone, Hash, Eq, PartialEq, Component, Debug)]
pub enum AgentAnimState {
    Idle,
    Walk,
    InAir,
    Dead,
}
impl Animatable for AgentAnimState {}

#[derive(Bundle)]
pub struct AgentBundle {
    _agent: Agent,
    movable: Moveable,
    judgement: Judgeable,
    dir: Dir,
    anim_state: AnimationVal<AgentAnimState>,
    senses: Senses,
    hitbox: Hitbox,
    velocity: Velocity,
}
impl AgentBundle {
    pub fn new(size: Vec2, num_senses: usize) -> AgentBundle {
        AgentBundle {
            _agent: Agent,
            movable: Moveable {
                gravity_enabled: true,
            },
            judgement: Judgeable { reward: 0.0 },
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
    commands: &mut Commands,
    pos: &Vec2,
    eye_info: Vec<SeeBox>,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let size = Vec2 { x: 64.0, y: 64.0 };
    let id = commands
        .spawn(AgentBundle::new(size.clone(), eye_info.len()))
        .id();
    for (ix, see_box) in eye_info.into_iter().enumerate() {
        let eye_id = commands
            .spawn(EyeBundle::new(
                ix,
                see_box.pos,
                see_box.size,
                see_box.angle,
            ))
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
                AnimationRoot::<AgentAnimState> {
                    state: AgentAnimState::Dead,
                    filename: "sprites/narf/InAir.png".to_string(),
                    width: 32,
                    height: 32,
                    length: 1,
                },
            ],
            asset_server,
            texture_atlases,
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

pub fn delete_all_agents(
    commands: &mut Commands,
    agents_query: Query<Entity, With<Agent>>,
) {
    for entity in agents_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn agent_update(query: Query<(&mut Transform, &Senses), With<Agent>>) {
    if query.is_empty() {
        return;
    }
}

pub fn agent_anim_update(
    mut query: Query<
        (&Velocity, &mut Dir, &mut AnimationVal<AgentAnimState>),
        With<Agent>,
    >,
) {
    for (vel, mut dir, mut anim_val) in query.iter_mut() {
        // The agent animation state machine! Huzzah!
        if anim_val.state == AgentAnimState::Dead {
            // Ignore dead boys
            continue;
        }
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

pub fn agent_move(
    mut query: Query<
        (&mut Velocity, &AnimationVal<AgentAnimState>),
        With<Agent>,
    >,
    input: Res<Input<KeyCode>>,
) {
    if query.is_empty() {
        return;
    };
    for (mut velocity, anim_val) in query.iter_mut() {
        // Ignore dead agents
        if anim_val.state == AgentAnimState::Dead {
            continue;
        }
        // Horizontal motion
        if input.any_pressed([KeyCode::A, KeyCode::Left]) {
            velocity.x -= X_ACCELERATION;
        } else if input.any_pressed([KeyCode::D, KeyCode::Right]) {
            velocity.x += X_ACCELERATION;
        } else {
            velocity.x *= 0.82;
        }
        if velocity.x.abs() > MAX_X_MOVE_SPEED {
            velocity.x =
                if velocity.x > 0.0 { 1.0 } else { -1.0 } * MAX_X_MOVE_SPEED;
        }
        if velocity.x.abs() < 0.1 {
            velocity.x = 0.0;
        }
        // Vertical motion
        if input.just_pressed(KeyCode::W) {
            velocity.y = GRAVITY / 2.0;
        }
    }
}

pub fn check_oob(
    mut query: Query<
        (
            &Transform,
            &mut Moveable,
            &mut Velocity,
            &mut AnimationVal<AgentAnimState>,
        ),
        With<Agent>,
    >,
) {
    if query.is_empty() {
        return;
    };
    for (transform, mut moveable, mut vel, mut anim_val) in query.iter_mut() {
        if transform.translation.x < -WINDOW_WIDTH / 2.0
            || WINDOW_WIDTH / 2.0 <= transform.translation.x
            || transform.translation.y < -WINDOW_HEIGHT / 2.0
            || WINDOW_HEIGHT / 2.0 < transform.translation.y
        {
            moveable.gravity_enabled = false;
            anim_val.state = AgentAnimState::Dead;
            vel.x = 0.0;
            vel.y = 0.0;
        }
    }
}

pub fn register_agent(app: &mut App) {
    app.add_systems(Update, agent_update)
        .add_systems(Update, agent_move)
        .add_systems(Update, agent_anim_update)
        .add_systems(Update, check_oob);
    register_eye(app);
    register_roll(app);
}

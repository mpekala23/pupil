use crate::agent::{delete_all_agents, eye::SeeBox, spawn_agent, Agent};
use bevy::prelude::*;

use self::consts::{WINDOW_HEIGHT, WINDOW_WIDTH};
pub mod consts;

#[derive(PartialEq, Debug)]
pub enum LevelState {
    Designing,
    Testing,
}

#[derive(Resource)]
pub struct MetaState {
    level_state: LevelState,
    spawn_loc: Vec2,
}

pub fn meta_setup(mut commands: Commands) {
    commands.insert_resource(MetaState {
        level_state: LevelState::Designing,
        spawn_loc: Vec2 { x: 100.0, y: 200.0 },
    });
}

pub fn meta_handle_state_switch(
    mut commands: Commands,
    mut meta: ResMut<MetaState>,
    input: Res<Input<KeyCode>>,
    agents_query: Query<Entity, With<Agent>>,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if input.just_pressed(KeyCode::Space) {
        if meta.level_state == LevelState::Designing {
            // Switching to testing
            delete_all_agents(&mut commands, agents_query);
            spawn_agent(
                commands,
                meta.spawn_loc.clone(),
                vec![SeeBox {
                    pos: Vec2 { x: 0.0, y: 0.0 },
                    size: Vec2 { x: 100.0, y: 10.0 },
                    angle: -3.1415926 / 4.0,
                    invert_x: false,
                }],
                asset_server,
                texture_atlases,
            );
            meta.level_state = LevelState::Testing;
        } else {
            // Switching to designing
            delete_all_agents(&mut commands, agents_query);
            meta.level_state = LevelState::Designing;
        }
        println!("{:?}", meta.level_state);
    }
}

pub fn check_oob(mut query: Query<&mut Transform, With<Agent>>) {
    if query.is_empty() {
        return;
    };
    let mut transform = query.single_mut();
    if transform.translation.x < -WINDOW_WIDTH / 2.0
        || WINDOW_WIDTH / 2.0 <= transform.translation.x
        || transform.translation.y < -WINDOW_HEIGHT / 2.0
        || WINDOW_HEIGHT / 2.0 < transform.translation.y
    {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
    }
}

pub fn register_meta(app: &mut App) {
    app.add_systems(Startup, meta_setup);
    app.add_systems(Update, check_oob);
    app.add_systems(Update, meta_handle_state_switch);
}

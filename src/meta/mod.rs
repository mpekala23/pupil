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
    iteration: u32,
}

pub fn meta_setup(mut commands: Commands) {
    commands.insert_resource(MetaState {
        level_state: LevelState::Designing,
        spawn_loc: Vec2 { x: 100.0, y: 200.0 },
        iteration: 0,
    });
}

/// Resets the testing state, setting the iteration back to zero
fn meta_reset_testing(
    commands: &mut Commands,
    meta: &mut ResMut<MetaState>,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    meta.iteration = 0;
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
}

fn meta_continue_designing(
    commands: &mut Commands,
    meta: &mut ResMut<MetaState>,
    agents_query: Query<Entity, With<Agent>>,
) {
    delete_all_agents(commands, agents_query);
    meta.level_state = LevelState::Designing;
}

pub fn meta_handle_state_switch(
    mut commands: Commands,
    mut meta: ResMut<MetaState>,
    input: Res<Input<KeyCode>>,
    agents_query: Query<Entity, With<Agent>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if input.just_pressed(KeyCode::Space) {
        if meta.level_state == LevelState::Designing {
            meta.level_state = LevelState::Testing;
            meta_reset_testing(
                &mut commands,
                &mut meta,
                &asset_server,
                &mut texture_atlases,
            );
        } else {
            meta.level_state = LevelState::Designing;
            meta_continue_designing(&mut commands, &mut meta, agents_query)
        }
    }
}

pub fn register_meta(app: &mut App) {
    app.add_systems(Startup, meta_setup);
    app.add_systems(Update, meta_handle_state_switch);
}

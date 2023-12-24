use crate::agent::{
    delete_all_agents,
    roll::{Roll, RollBundle},
    Agent,
};
use bevy::prelude::*;

pub mod consts;

#[derive(PartialEq, Debug)]
pub enum LevelState {
    Designing,
    Testing,
}

#[derive(Resource)]
pub struct MetaState {
    pub level_state: LevelState,
}

pub fn meta_setup(mut commands: Commands) {
    commands.insert_resource(MetaState {
        level_state: LevelState::Designing,
    });
}

/// Resets the testing state
fn meta_reset_testing(
    commands: &mut Commands,
    _meta: &mut ResMut<MetaState>, // Will eventually be used to pass size + spawn point info
) {
    commands.spawn(RollBundle::new(1, Vec2 { x: -50.0, y: 50.0 }));
}

fn meta_continue_designing(
    commands: &mut Commands,
    meta: &mut ResMut<MetaState>,
    rolls_query: Query<Entity, With<Roll>>,
    agents_query: Query<Entity, With<Agent>>,
) {
    for roll in rolls_query.iter() {
        commands.entity(roll).despawn();
    }
    delete_all_agents(commands, agents_query);
    meta.level_state = LevelState::Designing;
}

pub fn meta_handle_state_switch(
    mut commands: Commands,
    mut meta: ResMut<MetaState>,
    input: Res<Input<KeyCode>>,
    rolls_query: Query<Entity, With<Roll>>,
    agents_query: Query<Entity, With<Agent>>,
) {
    if input.just_pressed(KeyCode::Space) {
        if meta.level_state == LevelState::Designing {
            meta.level_state = LevelState::Testing;
            meta_reset_testing(&mut commands, &mut meta);
        } else {
            meta.level_state = LevelState::Designing;
            meta_continue_designing(
                &mut commands,
                &mut meta,
                rolls_query,
                agents_query,
            )
        }
    }
}

pub fn register_meta(app: &mut App) {
    app.add_systems(Startup, meta_setup);
    app.add_systems(Update, meta_handle_state_switch);
}

//! A roll is a group of armadillos
//! This is where logic exists for spawning many agents at the same time
//! and measuring fitness, spawning the next generation, etc.

use bevy::prelude::*;

use crate::{
    animation::AnimationVal,
    meta::{LevelState, MetaState},
};

use super::{eye::SeeBox, spawn_agent, Agent, AgentAnimState};

#[derive(Component)]
pub struct Roll {
    iteration: u32,
    size: u32,
    spawn_point: Vec2,
}
impl Roll {
    pub fn spawn_generation(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) {
        self.iteration += 1;
        for _ in 0..self.size {
            spawn_agent(
                commands,
                &self.spawn_point,
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
    }
}

#[derive(Bundle)]
pub struct RollBundle {
    roll: Roll,
}
impl RollBundle {
    pub fn new(size: u32, spawn_point: Vec2) -> RollBundle {
        RollBundle {
            roll: Roll {
                iteration: 0,
                size,
                spawn_point,
            },
        }
    }
}

pub fn drive_roll(
    mut commands: Commands,
    meta: ResMut<MetaState>,
    mut roll: Query<&mut Roll>,
    agents: Query<(&AnimationVal<AgentAnimState>, Entity), With<Agent>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if meta.level_state != LevelState::Testing || roll.is_empty() {
        // We only do stuff when we're testing
        return;
    }

    // INVARIANT: Only one roll at a time
    let mut roll = roll.single_mut();

    if agents.is_empty() {
        // We haven't started yet! Spawn in initial agents and that's it
        roll.spawn_generation(
            &mut commands,
            &asset_server,
            &mut texture_atlases,
        );
        return;
    }

    let all_dead = agents
        .iter()
        .all(|(anim_val, _)| anim_val.state == AgentAnimState::Dead);

    if !all_dead {
        // We don't have any work to do
        return;
    }

    // Spawn in new agents and then delete the old
    let old_ids = agents.iter().map(|(_, id)| id);
    roll.spawn_generation(&mut commands, &asset_server, &mut texture_atlases);
    println!("Roll Iteration: {}", roll.iteration);
    for id in old_ids {
        commands.entity(id).despawn_recursive();
    }
}

pub fn register_roll(app: &mut App) {
    app.add_systems(Update, drive_roll);
}

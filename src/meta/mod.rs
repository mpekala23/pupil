use bevy::prelude::*;

pub enum LevelState {
    Designing,
    Testing,
}

#[derive(Resource)]
pub struct MetaState {
    level_state: LevelState,
}

pub fn meta_setup(mut commands: Commands) {
    commands.insert_resource(MetaState {
        level_state: LevelState::Designing,
    });
}

pub fn register_meta(app: &mut App) {
    app.add_systems(Startup, meta_setup);
}

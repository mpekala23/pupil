pub mod block;
pub mod consts;

use bevy::prelude::*;

use self::block::BlockBundle;

pub fn environment_setup(mut commands: Commands) {
    commands.spawn(BlockBundle::new(
        Vec2 { x: 0.0, y: -50.0 },
        Vec2 {
            x: 1000.0,
            y: 100.0,
        },
    ));
    commands.spawn(BlockBundle::new(
        Vec2 { x: 0.0, y: 50.0 },
        Vec2 { x: 100.0, y: 100.0 },
    ));
}

pub fn register_environment(app: &mut App) {
    app.add_systems(Startup, environment_setup);
}

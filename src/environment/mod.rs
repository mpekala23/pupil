pub mod block;
pub mod consts;

use bevy::prelude::*;

use self::block::spawn_block;

pub fn environment_setup(commands: Commands) {
    println!("here");
    spawn_block(
        commands,
        Vec2 { x: 0.0, y: -200.0 },
        Vec2 {
            x: 1000.0,
            y: 100.0,
        },
    )
}

pub fn register_environment(app: &mut App) {
    app.add_systems(Startup, environment_setup);
}

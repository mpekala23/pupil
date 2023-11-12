pub mod agent;
pub mod animation;
pub mod environment;
pub mod meta;
pub mod physics;

use agent::register_agent;
use animation::register_animations;
use bevy::{prelude::*, window::WindowResolution};
use environment::register_environment;
use meta::{
    consts::{WINDOW_HEIGHT, WINDOW_WIDTH},
    register_meta,
};
use physics::register_physics;

pub fn main_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            // present_mode: (),
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            title: "PUPIL".to_string(),
            ..default()
        }),
        ..default()
    }))
    .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
    .add_systems(Startup, main_setup);
    register_animations(&mut app);
    register_environment(&mut app);
    register_meta(&mut app);
    register_physics(&mut app);
    register_agent(&mut app);
    app.run();
}

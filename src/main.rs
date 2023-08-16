pub mod agent;
pub mod environment;
pub mod meta;
pub mod physics;

use agent::register_agent;
use bevy::prelude::*;
use environment::register_environment;
use meta::register_meta;
use physics::register_physics;

pub fn test() {}

#[derive(Component)]
struct Collider;

#[derive(Bundle)]
struct BlockBundle {
    sprite: SpriteBundle,
    collider: Collider,
}
impl BlockBundle {
    fn new(pos: Vec2) -> BlockBundle {
        BlockBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 0.3),
                    ..default()
                },
                transform: Transform {
                    translation: pos.extend(0.0),
                    scale: Vec3::new(600.0, 10.0, 1.0),
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

pub fn main_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(BlockBundle::new(Vec2 { x: 0.0, y: 0.0 }));
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(Startup, main_setup)
        .add_systems(FixedUpdate, test);
    register_agent(&mut app);
    register_environment(&mut app);
    register_meta(&mut app);
    register_physics(&mut app);
    app.run();
}

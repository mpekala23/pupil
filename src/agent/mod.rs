use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    sprite: SpriteBundle,
    velocity: Velocity,
}
impl PlayerBundle {
    fn new(pos: Vec2) -> PlayerBundle {
        PlayerBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 1.0, 0.3),
                    ..default()
                },
                transform: Transform {
                    translation: pos.extend(0.0),
                    scale: Vec3::new(60.0, 60.0, 1.0),
                    ..default()
                },
                ..default()
            },
            velocity: Velocity { x: 0.0, y: 0.0 },
        }
    }
}

pub fn player_setup(mut commands: Commands) {
    commands.spawn(PlayerBundle::new(Vec2 { x: 0.0, y: 50.0 }));
}

pub fn player_update(mut query: Query<&mut Transform, With<Player>>) {
    println!("In player update");
    if query.is_empty() {
        return;
    }
    let mut player = query.single_mut();
}

pub fn register_player_systems(app: &mut App) {
    app.add_systems(Startup, player_setup)
        .add_systems(FixedUpdate, player_update);
}

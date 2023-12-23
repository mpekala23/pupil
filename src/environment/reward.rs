use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};

use crate::physics::physics_collide;

#[derive(Component)]
pub struct DistanceReward;
#[derive(Bundle)]
pub struct DistanceRewardBundle {
    _dr: DistanceReward,
    spatial: SpatialBundle,
    sprite: Sprite,
    texture: Handle<Image>,
}
impl DistanceRewardBundle {
    pub fn new(pos: Vec2) -> DistanceRewardBundle {
        DistanceRewardBundle {
            _dr: DistanceReward,
            spatial: SpatialBundle {
                transform: Transform {
                    translation: pos.extend(0.0),
                    scale: Vec3::new(10.0, 10.0, 1.0),
                    ..default()
                },
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.0, 1.0, 0.0),
                ..default()
            },
            texture: DEFAULT_IMAGE_HANDLE.typed(),
        }
    }
}

#[derive(Component)]
pub struct Judgeable {
    pub reward: f32,
}

pub fn reward_do_judgement(
    mut subjects: Query<(&mut Judgeable, &GlobalTransform)>,
    drs: Query<&GlobalTransform, With<DistanceReward>>,
) {
    for (mut sub_score, sub_trans) in subjects.iter_mut() {
        let mut cum_reward = 0.0;
        for dr_trans in drs.iter() {
            cum_reward +=
                sub_trans.translation().distance(dr_trans.translation());
        }
        sub_score.reward = cum_reward;
        println!("Subscore: {}", sub_score.reward);
    }
}

pub fn register_reward(app: &mut App) {
    app.add_systems(Update, reward_do_judgement.after(physics_collide));
}

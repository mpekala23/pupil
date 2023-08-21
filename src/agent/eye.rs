use crate::physics::{
    are_colliding,
    collisions::{distance_point_to_segment, Triangle},
    consts::GRAVITY,
    physics_move, Hitbox, Moveable, Velocity,
};
use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE, sprite::Anchor, utils::HashMap};

use super::{Agent, Observation};

#[derive(Component)]
pub struct Eye {
    parent: Entity,
}

#[derive(Component)]
pub struct SeeBox {
    pub pos: Vec2,
    pub size: Vec2,
    pub angle: f32,
}
impl SeeBox {
    pub fn two_triangles(&self, t: &Transform) -> (Triangle, Triangle) {
        let trans = t.translation;
        (
            Triangle {
                a: Vec2 {
                    x: trans.x + self.size.x,
                    y: trans.y + self.size.y / 2.0,
                },
                b: Vec2 {
                    x: trans.x,
                    y: trans.y + self.size.y / 2.0,
                },
                c: Vec2 {
                    x: trans.x,
                    y: trans.y - self.size.y / 2.0,
                },
            },
            Triangle {
                a: Vec2 {
                    x: trans.x + self.size.x,
                    y: trans.y + self.size.y / 2.0,
                },
                b: Vec2 {
                    x: trans.x,
                    y: trans.y - self.size.y / 2.0,
                },
                c: Vec2 {
                    x: trans.x + self.size.x,
                    y: trans.y - self.size.y / 2.0,
                },
            },
        )
    }
}

#[derive(Component)]
pub struct Seeable;

#[derive(Bundle)]
pub struct EyeBundle {
    _eye: Eye,
    spatial: SpatialBundle,
    sprite: Sprite,
    texture: Handle<Image>,
    seebox: SeeBox,
}
impl EyeBundle {
    pub fn new(parent: Entity, pos: Vec2, size: Vec2, angle: f32) -> EyeBundle {
        EyeBundle {
            _eye: Eye { parent },
            spatial: SpatialBundle {
                transform: Transform {
                    translation: pos.extend(0.0),
                    scale: size.extend(1.0),
                    ..default()
                },
                ..default()
            },
            sprite: Sprite {
                color: Color::rgba(0.1, 0.3, 0.1, 0.5),
                anchor: Anchor::CenterRight,
                ..default()
            },
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            seebox: SeeBox {
                pos: Vec2 { x: 0.0, y: 0.0 },
                size,
                angle,
            },
        }
    }
}

/// For tethering eyes back to their parents
pub fn eye_tether(
    mut eyes: Query<(&Eye, &SeeBox, &mut Transform), With<Eye>>,
    agents: Query<&Transform, Without<Eye>>,
) {
    for (e, sb, mut t) in eyes.iter_mut() {
        let Ok(parent) = agents.get(e.parent) else { continue };
        t.translation = parent.translation;
        t.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), sb.angle);
    }
}

/// For having eyes try to see things
pub fn eye_see(
    eyes: Query<(Entity, &Eye, &SeeBox, &Transform), With<Eye>>,
    seeable: Query<(&Hitbox, &Seeable, &Transform), With<Seeable>>,
    mut agents: Query<&Observation, Without<Eye>>,
) {
    for (id, e, sb, eye_t) in eyes.iter() {
        let (et1, et2) = sb.two_triangles(eye_t);
        let mut collision = false;
        for (hb, _, see_t) in seeable.iter() {
            let (st1, st2) = hb.two_triangles(see_t);
            if et1.is_colliding_with_triangle(&st1)
                || et1.is_colliding_with_triangle(&st2)
                || et2.is_colliding_with_triangle(&st1)
                || et2.is_colliding_with_triangle(&st2)
            {
                println!("Collision!");
                let segments = hb.segments(see_t);
                let mut min_dist: Option<f32> = None;
                let eye_trans = Vec2 {
                    x: eye_t.translation.x,
                    y: eye_t.translation.y,
                };
                for seg in segments {
                    let dist = distance_point_to_segment(eye_trans, seg);
                    min_dist = match min_dist {
                        None => Some(dist),
                        Some(old_dist) => {
                            if dist < old_dist {
                                Some(dist)
                            } else {
                                Some(old_dist)
                            }
                        }
                    }
                }
                println!("Dist is {}", min_dist.unwrap());
            }
        }
    }
}

pub fn register_eye(app: &mut App) {
    app.add_systems(Update, eye_tether.after(physics_move));
    app.add_systems(Update, eye_see.after(eye_tether));
}

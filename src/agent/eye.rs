use std::f32::consts::PI;

use crate::physics::{collisions::Triangle, consts::Dir, physics_move, rotate, Hitbox};
use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE, sprite::Anchor};

use super::Senses;

#[derive(Component)]
pub struct Eye {
    ix: usize,
}

#[derive(Component)]
pub struct SeeBox {
    pub pos: Vec2,
    pub size: Vec2,
    pub angle: f32,
    pub invert_x: bool,
}
impl SeeBox {
    pub fn two_triangles(&self, trans: Vec2) -> (Triangle, Triangle) {
        let pre_rotation = (
            Triangle {
                a: Vec2 {
                    x: self.size.x,
                    y: self.size.y / 2.0,
                },
                b: Vec2 {
                    x: 0.0,
                    y: self.size.y / 2.0,
                },
                c: Vec2 {
                    x: 0.0,
                    y: -self.size.y / 2.0,
                },
            },
            Triangle {
                a: Vec2 {
                    x: self.size.x,
                    y: self.size.y / 2.0,
                },
                b: Vec2 {
                    x: 0.0,
                    y: -self.size.y / 2.0,
                },
                c: Vec2 {
                    x: self.size.x,
                    y: -self.size.y / 2.0,
                },
            },
        );
        let true_ang = if self.invert_x {
            PI - self.angle
        } else {
            self.angle
        };
        (
            Triangle {
                a: trans + rotate(pre_rotation.0.a, true_ang),
                b: trans + rotate(pre_rotation.0.b, true_ang),
                c: trans + rotate(pre_rotation.0.c, true_ang),
            },
            Triangle {
                a: trans + rotate(pre_rotation.1.a, true_ang),
                b: trans + rotate(pre_rotation.1.b, true_ang),
                c: trans + rotate(pre_rotation.1.c, true_ang),
            },
        )
    }

    pub fn to_scale(&self, scale: f32) -> SeeBox {
        SeeBox {
            pos: self.pos,
            size: self.size * scale,
            angle: self.angle,
            invert_x: self.invert_x,
        }
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
    pub fn new(ix: usize, pos: Vec2, size: Vec2, angle: f32) -> EyeBundle {
        EyeBundle {
            _eye: Eye { ix },
            spatial: SpatialBundle {
                transform: Transform {
                    translation: pos.extend(0.0),
                    scale: size.extend(1.0),
                    rotation: Quat::from_axis_angle(Vec3::new(0., 0., 1.), 3.141592654321 + angle),
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
                invert_x: false,
            },
        }
    }
}

pub fn is_detected(
    sb: &SeeBox,
    pos: Vec2,
    seeable: &Query<(&Hitbox, &Seeable, &Transform), (With<Seeable>, Without<Eye>)>,
) -> bool {
    let (et1, et2) = sb.two_triangles(pos.clone());
    for (hb, _, see_t) in seeable.iter() {
        let (st1, st2) = hb.two_triangles(see_t);
        if et1.is_colliding_with_triangle(&st1)
            || et1.is_colliding_with_triangle(&st2)
            || et2.is_colliding_with_triangle(&st1)
            || et2.is_colliding_with_triangle(&st2)
        {
            return true;
        }
    }
    false
}

/// For having eyes try to see things
pub fn eye_see(
    mut eyes: Query<(&Eye, &mut SeeBox, &Parent, &mut Transform), With<Eye>>,
    mut agents: Query<(&Transform, &mut Senses, &Dir), Without<Eye>>,
    seeable: Query<(&Hitbox, &Seeable, &Transform), (With<Seeable>, Without<Eye>)>,
) {
    for (e, mut sb, parent, mut eye_t) in eyes.iter_mut() {
        let Ok((agent_trans, mut senses, dir)) = agents.get_mut(parent.get()) else {continue;};
        // Change the display based on facing direction
        sb.invert_x = dir == &Dir::Left;
        let true_ang = if sb.invert_x {
            -sb.angle
        } else {
            PI + sb.angle
        };
        eye_t.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), true_ang);
        let pos = Vec2 {
            x: agent_trans.translation.x,
            y: agent_trans.translation.y,
        };
        if !is_detected(&sb, pos, &seeable) {
            senses.data[e.ix] = None;
            continue;
        }
        // Perform binary search with different seeboxes to find distances
        let resolution = 8;
        let mut min = 0.0;
        let mut max = 1.0;
        let mut mid = 0.5;
        for _ in 0..resolution {
            let sized_sb = sb.to_scale(mid);
            if is_detected(&sized_sb, pos, &seeable) {
                max = mid;
            } else {
                min = mid;
            }
            mid = (min + max) / 2.0;
        }
        senses.data[e.ix] = Some(mid);
    }
}

pub fn register_eye(app: &mut App) {
    app.add_systems(Update, eye_see.after(physics_move));
}

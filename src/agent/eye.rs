use crate::physics::{collisions::Triangle, physics_move, rotate, Hitbox};
use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE, sprite::Anchor};

use super::Observation;

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
        let trans = Vec2 {
            x: trans.x,
            y: trans.y,
        };
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
        (
            Triangle {
                a: trans + rotate(pre_rotation.0.a, self.angle),
                b: trans + rotate(pre_rotation.0.b, self.angle),
                c: trans + rotate(pre_rotation.0.c, self.angle),
            },
            Triangle {
                a: trans + rotate(pre_rotation.1.a, self.angle),
                b: trans + rotate(pre_rotation.1.b, self.angle),
                c: trans + rotate(pre_rotation.1.c, self.angle),
            },
        )
    }

    pub fn to_scale(&self, scale: f32) -> SeeBox {
        SeeBox {
            pos: self.pos,
            size: self.size * scale,
            angle: self.angle,
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
        t.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), 3.141592654321 + sb.angle);
    }
}

pub fn is_detected(
    sb: &SeeBox,
    eye_t: &Transform,
    seeable: &Query<(&Hitbox, &Seeable, &Transform), With<Seeable>>,
) -> bool {
    let (et1, et2) = sb.two_triangles(eye_t);
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
    eyes: Query<(Entity, &Eye, &SeeBox, &Transform), With<Eye>>,
    seeable: Query<(&Hitbox, &Seeable, &Transform), With<Seeable>>,
    mut agents: Query<&mut Observation, Without<Eye>>,
) {
    for (id, e, sb, eye_t) in eyes.iter() {
        let Ok(mut agent) = agents.get_mut(e.parent) else {continue;};
        if !is_detected(sb, eye_t, &seeable) {
            agent.senses.insert(id, None);
            continue;
        }
        // Perform binary search with different seeboxes to find distances
        let resolution = 8;
        let mut min = 0.0;
        let mut max = 1.0;
        let mut mid = 0.5;
        for _ in 0..resolution {
            let sized_sb = sb.to_scale(mid);
            if is_detected(&sized_sb, eye_t, &seeable) {
                max = mid;
            } else {
                min = mid;
            }
            mid = (min + max) / 2.0;
        }
        agent.senses.insert(id, Some(mid));
    }
}

pub fn register_eye(app: &mut App) {
    app.add_systems(Update, eye_tether.after(physics_move));
    app.add_systems(Update, eye_see.after(eye_tether));
}

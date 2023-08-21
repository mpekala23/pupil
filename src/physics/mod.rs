pub mod collisions;
pub mod consts;

use bevy::prelude::*;

use crate::physics::consts::COLLISION_THRESHOLD;

use self::{collisions::Triangle, consts::GRAVITY};

/// A simple hitbox
/// Position is relative to the parent's transform
/// TODO? Implement get_bounds(transform: &Transform) that returns left/right/top/bot
/// for a hitbox on a specific transform
#[derive(Component)]
pub struct Hitbox {
    pub pos: Vec2,
    pub size: Vec2,
}
impl Hitbox {
    pub fn two_triangles(&self, t: &Transform) -> (Triangle, Triangle) {
        let trans = t.translation;
        (
            Triangle {
                a: Vec2 {
                    x: trans.x + self.size.x / 2.0,
                    y: trans.y + self.size.y / 2.0,
                },
                b: Vec2 {
                    x: trans.x - self.size.x / 2.0,
                    y: trans.y + self.size.y / 2.0,
                },
                c: Vec2 {
                    x: trans.x - self.size.x / 2.0,
                    y: trans.y - self.size.y / 2.0,
                },
            },
            Triangle {
                a: Vec2 {
                    x: trans.x + self.size.x / 2.0,
                    y: trans.y + self.size.y / 2.0,
                },
                b: Vec2 {
                    x: trans.x - self.size.x / 2.0,
                    y: trans.y - self.size.y / 2.0,
                },
                c: Vec2 {
                    x: trans.x + self.size.x / 2.0,
                    y: trans.y - self.size.y / 2.0,
                },
            },
        )
    }

    pub fn segments(&self, t: &Transform) -> Vec<(Vec2, Vec2)> {
        let trans = t.translation;
        let a = Vec2 {
            x: trans.x - self.size.x / 2.0,
            y: trans.y - self.size.y / 2.0,
        };
        let b = Vec2 {
            x: trans.x - self.size.x / 2.0,
            y: trans.y + self.size.y / 2.0,
        };
        let c = Vec2 {
            x: trans.x + self.size.x / 2.0,
            y: trans.y + self.size.y / 2.0,
        };
        let d = Vec2 {
            x: trans.x + self.size.x / 2.0,
            y: trans.y - self.size.y / 2.0,
        };
        vec![(a, b), (b, c), (c, d), (d, a)]
    }
}

pub fn rotate(v: Vec2, angle: f32) -> Vec2 {
    Vec2 {
        x: v.x * angle.cos() - v.y * angle.sin(),
        y: v.x * angle.sin() + v.y * angle.cos(),
    }
}

// Simple velocity
#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

/// Does this component move?
#[derive(Component)]
pub struct Moveable;

pub fn physics_setup(mut commands: Commands) {}

pub fn physics_gravity(time: Res<Time>, mut query: Query<&mut Velocity, With<Moveable>>) {
    const SUPPOSED_SPF: f32 = 1.0 / 60.0;
    let mut adjust_mult = time.delta_seconds();
    if adjust_mult > SUPPOSED_SPF * 3.0 {
        adjust_mult = SUPPOSED_SPF * 3.0;
    }
    for mut velocity in query.iter_mut() {
        velocity.y -= GRAVITY * adjust_mult;
    }
}

pub fn physics_move(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform), With<Moveable>>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

pub fn get_bounds(hitbox: &Hitbox, transform: &Transform) -> (f32, f32, f32, f32) {
    let left = transform.translation.x + hitbox.pos.x - hitbox.size.x / 2.0;
    let right = left + hitbox.size.x;
    let top = transform.translation.y + hitbox.pos.y + hitbox.size.y / 2.0;
    let bot = top - hitbox.size.y;
    (left, right, top, bot)
}

pub fn are_colliding(h1: &Hitbox, t1: &Transform, h2: &Hitbox, t2: &Transform) -> bool {
    let left1 = t1.translation.x + h1.pos.x - h1.size.x / 2.0;
    let right1 = left1 + h1.size.x;
    let top1 = t1.translation.y + h1.pos.y + h1.size.y / 2.0;
    let bot1 = top1 - h1.size.y;

    let left2 = t2.translation.x + h2.pos.x - h2.size.x / 2.0;
    let right2 = left2 + h2.size.x;
    let top2 = t2.translation.y + h2.pos.y + h2.size.y / 2.0;
    let bot2 = top2 - h2.size.y;

    return !(bot1 >= top2 || top1 <= bot2 || right1 <= left2 || left1 >= right2);
}

pub fn resolve_move_immove_collision(
    h1: &Hitbox,
    t1: &mut Transform,
    v1: &mut Velocity,
    h2: &Hitbox,
    t2: &Transform,
) {
    // Get the bounds
    let (ml, mr, mt, mb) = get_bounds(h1, t1);
    let (il, ir, it, ib) = get_bounds(h2, t2);
    const INF: f32 = 99999.9;

    // WHY RUST WHY NO CMP FOR F32???
    let left_resolve = if mr > il { mr - il } else { INF };
    let mut min_resolve = left_resolve;
    let right_resolve = if ml < ir { ir - ml } else { INF };
    if right_resolve < min_resolve {
        min_resolve = right_resolve;
    }
    let top_resolve = if mb < it { it - mb } else { INF };
    if top_resolve < min_resolve {
        min_resolve = top_resolve;
    }
    let bot_resolve = if mt > ib { mt - ib } else { INF };
    if bot_resolve < min_resolve {
        min_resolve = bot_resolve;
    }

    // Resolve appropriately
    if left_resolve <= min_resolve {
        t1.translation.x -= left_resolve;
        if v1.x > 0.0 {
            v1.x *= -1.0 * 0.4;
        }
    } else if right_resolve <= min_resolve {
        t1.translation.x += right_resolve;
        if v1.x < 0.0 {
            v1.x *= -1.0 * 0.4;
        }
    } else if top_resolve <= min_resolve {
        t1.translation.y += top_resolve;
        if v1.y < 0.0 {
            // Only flip velocity if would move further into this thing
            v1.y *= -1.0 * 0.4;
        }
    } else {
        t1.translation.y -= bot_resolve;
        if v1.y > 0.0 {
            // Only flip velocity if would move further into this thing
            v1.y *= -1.0 * 0.4;
        }
    }
    if v1.x.abs() < COLLISION_THRESHOLD {
        v1.x = 0.0;
    }
    if v1.y.abs() < COLLISION_THRESHOLD {
        v1.y = 0.0;
    }
}

pub fn physics_collide(
    mut q_movable: Query<(&Hitbox, &mut Transform, &mut Velocity), With<Moveable>>,
    q_immovable: Query<(&mut Hitbox, &Transform), Without<Moveable>>,
) {
    // First resolve all collisions between two moveable objects
    // TODO
    // Then resolve all collisions between moveable and immoveable
    for (h1, mut t1, mut v1) in q_movable.iter_mut() {
        for (h2, t2) in q_immovable.iter() {
            if !are_colliding(h1, &t1, h2, t2) {
                continue;
            }
            // println!("Would resolve collision {:?}, {:?}", t1, t2);
            resolve_move_immove_collision(h1, &mut t1, &mut v1, h2, t2);
        }
    }
}

pub fn register_physics(app: &mut App) {
    app.add_systems(Startup, physics_setup)
        .add_systems(Update, physics_gravity)
        .add_systems(Update, physics_move.after(physics_gravity))
        .add_systems(Update, physics_collide.after(physics_move));
}

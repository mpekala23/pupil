use bevy::prelude::*;
use std::fmt;

pub struct Triangle {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2,
}
impl Triangle {
    pub fn get_points(&self) -> Vec<Vec2> {
        vec![self.a, self.b, self.c]
    }

    pub fn get_edges(&self) -> Vec<(Vec2, Vec2)> {
        vec![(self.a, self.b), (self.a, self.c), (self.b, self.c)]
    }

    pub fn get_trips(&self) -> Vec<(Vec2, Vec2, Vec2)> {
        vec![
            (self.a, self.b, self.c),
            (self.b, self.c, self.a),
            (self.c, self.a, self.b),
        ]
    }

    pub fn is_colliding_with_triangle(&self, triangle: &Triangle) -> bool {
        are_triangles_colliding(self, triangle)
    }
}
impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.a, self.b, self.c)
    }
}

// Distance from a point to a segment
pub fn distance_point_to_segment(point: Vec2, segment: (Vec2, Vec2)) -> f32 {
    let (a, b) = segment;
    let e1 = b - a;
    let e2 = a - b;
    let v1 = point - a;
    let v2 = point - b;
    let d1 = e1.x * v1.x + e1.y * v1.y;
    let d2 = e2.x * v2.x + e2.y * v2.y;
    if d1 < 0.0 || d2 < 0.0 {
        // Shortest distance is to one of the endpoints
        let dist2a = (point - a).length();
        let dist2b = (point - b).length();
        if dist2a < dist2b {
            dist2a
        } else {
            dist2b
        }
    } else {
        // Shortest distsance is to the line
        if (a.x - b.x).abs() <= 0.001 {
            // Don't divide by near-zero
            (point.x - a.x).abs()
        } else {
            let slope = (b.y - a.y) / (b.x - a.x);
            let f = slope;
            let g = -1.0;
            let h = a.y - slope * a.x;
            let num = (f * point.x + g * point.y + h).abs();
            let denom = (f * f + g * g).sqrt();
            num / denom
        }
    }
}

/// Determine if two triangles are colliding by searching for a counterexample
pub fn are_triangles_colliding(tri1: &Triangle, tri2: &Triangle) -> bool {
    let pairs = vec![(tri1, tri2), (tri2, tri1)];
    for (t1, t2) in pairs {
        for (a, b, c) in t1.get_trips() {
            // Cross product +/- signals side of the edge b - a
            let mut sum: f32 = 0.0;
            let edge = b - a;
            for point in t2.get_points() {
                let diff = point - a;
                let cross = edge.x * diff.y - edge.y * diff.x;
                if cross > 0.0 {
                    sum += 1.0;
                }
                if cross < 0.0 {
                    sum -= 1.0;
                }
            }
            if sum.abs() < 2.9 {
                // Not all points of the second triangle are on the same side,
                // thus this triangle is not a counter example
                continue;
            }
            // All three points of the second triangle are on the same side of this edge
            // Need to check the third point of the first triangle
            let diff = c - a;
            let cross = edge.x * diff.y - edge.y * diff.x;
            if cross * sum < 0.0 {
                // Signs of crosses disagree, meaning this edge separates the points
                return false;
            }
        }
    }
    true
}

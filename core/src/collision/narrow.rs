use crate::dynamics::Body;
use crate::math::Vec2;

pub struct Contact {
    pub normal: Vec2,
    pub penetration: f64,
}

pub fn circle_circle(pos_a: Vec2, radius_a: f64, pos_b: Vec2, radius_b: f64) -> Option<Contact> {
    let delta = pos_b - pos_a;
    let dist_sq = delta.length_squared();
    let r = radius_a + radius_b;

    if dist_sq >= r * r || dist_sq <= f64::EPSILON {
        return None;
    }

    let dist = dist_sq.sqrt();
    let penetration = r - dist;
    let normal = delta / dist;

    Some(Contact {
        normal,
        penetration,
    })
}

pub fn contact(a: &Body, b: &Body) -> Option<Contact> {
    circle_circle(a.pos, a.radius, b.pos, b.radius)
}

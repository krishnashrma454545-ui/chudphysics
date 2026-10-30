use crate::collision::Shape;
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
    match (&a.shape, &b.shape) {
        (Shape::Circle { radius: r1 }, Shape::Circle { radius: r2 }) => {
            crate::collision::circle_circle(a.pos, *r1, b.pos, *r2)
        }
        _ => None,
    }
}
pub fn narrow_phase(a: &Body, b: &Body) -> Option<crate::collision::Contact> {
    match (&a.shape, &b.shape) {
        // Unpack the radii if both are circles
        (Shape::Circle { radius: r1 }, Shape::Circle { radius: r2 }) => {
            crate::collision::circle_circle(a.pos, *r1, b.pos, *r2)
        }
        // We will add Box/Box and Box/Circle math here later!
        _ => None,
    }
}

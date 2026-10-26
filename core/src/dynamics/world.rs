use super::body::Body;
use crate::math::Vec2;
use tracing::{info, instrument};

pub type Body2D = Body;

pub struct PhysicsWorld {
    pub bodies: Vec<Body>,
    pub gravity: Vec2,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            gravity: Vec2::new(0.0, -9.81),
        }
    }

    pub fn add(&mut self, body: Body) -> usize {
        let idx = self.bodies.len();
        self.bodies.push(body);
        idx
    }

    #[instrument(skip(self), fields(bodies = self.bodies.len()))]
    pub fn step(&mut self, dt: f64) {
        self.integrate(dt);
        self.resolve_collisions();
    }

    fn integrate(&mut self, dt: f64) {
        for body in &mut self.bodies {
            body.vel = body.vel + self.gravity * dt;
            body.pos = body.pos + body.vel * dt;
        }
    }

    #[instrument(skip(self))]
    fn resolve_collisions(&mut self) {
        let n = self.bodies.len();
        for i in 0..n {
            for j in (i + 1)..n {
                self.resolve_pair(i, j);
            }
        }
    }

    fn resolve_pair(&mut self, i: usize, j: usize) {
        let (a, b) = {
            let (left, right) = self.bodies.split_at_mut(j);
            (&mut left[i], &mut right[0])
        };

        let delta = b.pos - a.pos;
        let dist_sq = delta.length_squared();
        let r = a.radius + b.radius;

        if dist_sq >= r * r || dist_sq <= f64::EPSILON {
            return;
        }

        let dist = dist_sq.sqrt();
        let penetration = r - dist;
        let normal = delta / dist;

        let correction = normal * (penetration * 0.5);
        a.pos = a.pos - correction;
        b.pos = b.pos + correction;

        info!(
            a_id = a.id,
            b_id = b.id,
            %penetration,
            "collision resolved"
        );
    }
}

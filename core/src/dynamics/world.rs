use super::body::Body;
use crate::collision::Shape;
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
        self.resolve_boundaries();
        self.resolve_collisions();
    }

    fn integrate(&mut self, dt: f64) {
        for body in &mut self.bodies {
            body.vel = body.vel + self.gravity * dt;
            body.pos = body.pos + body.vel * dt;

            body.angle += body.angular_vel * dt;
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

    fn resolve_boundaries(&mut self) {
        let floor_y = 300.0;

        for body in &mut self.bodies {
            let extent_y = match &body.shape {
                Shape::Circle { radius } => *radius,
                Shape::Aabb { half_extents } => half_extents.1,
                Shape::Triangle { size } => *size / 2.0,
            };

            if body.pos.y + extent_y > floor_y {
                body.pos.y = floor_y - extent_y;
                if body.vel.y > 0.0 {
                    body.vel.y = -body.vel.y * body.restitution;
                }
            }
        }
    }

    fn resolve_pair(&mut self, i: usize, j: usize) {
        let (a, b) = {
            let (left, right) = self.bodies.split_at_mut(j);
            (&mut left[i], &mut right[0])
        };

        //Check if BOTH shapes are circles
        if let (Shape::Circle { radius: r1 }, Shape::Circle { radius: r2 }) = (&a.shape, &b.shape) {
            let r = r1 + r2;
            let delta = b.pos - a.pos;
            let dist_sq = delta.length_squared();

            let total_inv_mass = a.inv_mass + b.inv_mass;
            if total_inv_mass == 0.0 {
                return;
            }

            if dist_sq >= r * r || dist_sq <= f64::EPSILON {
                return;
            }

            let dist = dist_sq.sqrt();
            let penetration = r - dist;
            let normal = delta / dist;

            let correction = normal * (penetration / total_inv_mass);
            a.pos = a.pos - correction * a.inv_mass;
            b.pos = b.pos + correction * b.inv_mass;

            let relative_vel = b.vel - a.vel;
            let vel_along_normal = relative_vel.dot(normal);

            if vel_along_normal > 0.0 {
                return;
            }

            let e = a.restitution.min(b.restitution);
            let j = -(1.0 + e) * vel_along_normal / total_inv_mass;
            let impulse = normal * j;

            a.vel = a.vel - impulse * a.inv_mass;
            b.vel = b.vel + impulse * b.inv_mass;

            let correction = normal * (penetration * 0.5);
            a.pos = a.pos - correction;
            b.pos = b.pos + correction;

            info!(a_id= a.id,b_id =  b.id,%penetration,
                "collision resloved with impluse");
        }

        if let (Shape::Aabb { half_extents: he1 }, Shape::Aabb { half_extents: he2 }) =
            (&a.shape, &b.shape)
        {
            let total_inv_mass = a.inv_mass + b.inv_mass;
            if total_inv_mass == 0.0 {
                return;
            }
            let dx = b.pos.x - a.pos.x;
            let dy = b.pos.y - a.pos.y;

            let overlap_x = (he1.0 + he2.0) - dx.abs();
            let overlap_y = (he1.1 + he2.1) - dy.abs();

            if overlap_x <= 0.0 || overlap_y <= 0.0 {
                return;
            }

            let (normal, penetration) = if overlap_x < overlap_y {
                let n = Vec2::new(if dx > 0.0 { 1.0 } else { 1.0 }, 0.0);
                (n, overlap_x)
            } else {
                let n = Vec2::new(0.0, if dy > 0.0 { 1.0 } else { -1.0 });
                (n, overlap_y)
            };

            let correction = normal * (penetration / total_inv_mass);
            a.pos = a.pos - correction * a.inv_mass;
            b.pos = b.pos + correction * b.inv_mass;

            let relative_vel = b.vel - a.vel;
            let vel_along_normal = relative_vel.dot(normal);

            if vel_along_normal > 0.0 {
                return;
            }

            let e = a.restitution.min(b.restitution);
            let impulse_magnitude = -(1.0 + e) * vel_along_normal / total_inv_mass;

            let tangent = Vec2::new(-normal.y, normal.x);
            let vel_along_tangent = relative_vel.dot(tangent);

            let jt = -vel_along_tangent / total_inv_mass;

            let mu = (a.friction + b.friction) * 0.5;

            let friction_impluse = if jt.abs() < impulse_magnitude * mu {
                jt
            } else {
                -impulse_magnitude * mu * jt.signum()
            };

            let friction_vec = tangent * friction_impluse;
            a.vel = a.vel - friction_vec * a.inv_mass;
            b.vel = b.vel + friction_vec * b.inv_mass;

            let impluse = normal * impulse_magnitude;

            a.vel = a.vel - impluse * a.inv_mass;
            b.vel = b.vel + impluse * b.inv_mass;

            info!(a_id = a.id , b_id = b.id,%penetration,"Box collision resolved");
        }

        if let (Shape::Circle { radius }, Shape::Aabb { half_extents }) = (&a.shape, &b.shape) {
            let closest_x = a
                .pos
                .x
                .clamp(b.pos.x - half_extents.0, b.pos.x + half_extents.0);

            let closest_y = a
                .pos
                .y
                .clamp(b.pos.y - half_extents.1, b.pos.y + half_extents.1);

            let closest_point = Vec2::new(closest_x, closest_y);

            let delta = closest_point - a.pos;
            let dist_sq = delta.length_squared();

            if dist_sq >= radius * radius || dist_sq <= f64::EPSILON {
                return;
            }

            let dist = dist_sq.sqrt();
            let penetration = radius - dist;
            let normal = delta / dist;

            let total_inv_mass = a.inv_mass + b.inv_mass;
            if total_inv_mass == 0.0 {
                return;
            }

            let correction = normal * (penetration / total_inv_mass);
            a.pos = a.pos - correction * a.inv_mass;
            b.pos = b.pos + correction * b.inv_mass;

            let relative_vel = b.vel - a.vel;
            let vel_along_normal = relative_vel.dot(normal);

            let e = a.restitution.min(b.restitution);
            let impluse_magnitude = -(1.0 + e) * vel_along_normal / total_inv_mass;
            let impluse = normal * impluse_magnitude;

            a.vel = a.vel - impluse * a.inv_mass;

            b.vel = b.vel + impluse * b.inv_mass;

            info!(a_id = a.id, b_id = b.id, %penetration,"CIRCLE-BOX collision resloved")
        }
        // If they are boxes or triangles, we ignore them for now!
    }
}

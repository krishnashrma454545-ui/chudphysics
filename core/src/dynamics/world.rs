use super::body::Body;
use crate::collision::Shape;
use crate::math::Vec2;
use tracing::instrument;

pub type Body2D = Body;

/// Largest vertex count of any convex polygon shape (AABB = 4, triangle = 3).
const MAX_POLY_VERTICES: usize = 4;

/// A contact between two bodies, already oriented from `a` to `b`.
struct Manifold {
    /// Unit normal pointing from `a` towards `b`.
    normal: Vec2,
    /// Overlap along `normal`, always > 0.
    penetration: f64,
    /// Contact point relative to `a.pos` (the torque arm of `a`).
    offset_a: Vec2,
    /// Contact point relative to `b.pos` (the torque arm of `b`).
    offset_b: Vec2,
}

impl Manifold {
    /// The same contact, seen from the other body.
    fn flipped(self) -> Self {
        Self {
            normal: -self.normal,
            penetration: self.penetration,
            offset_a: self.offset_b,
            offset_b: self.offset_a,
        }
    }
}

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

        let Some(contact) = narrow_phase(a, b) else {
            return;
        };

        let total_inv_mass = a.inv_mass + b.inv_mass;
        if total_inv_mass == 0.0 {
            return;
        }

        let normal = contact.normal;
        let offset_a = contact.offset_a;
        let offset_b = contact.offset_b;

        // Positional correction: push both bodies apart along the normal.
        let correction = normal * (contact.penetration / total_inv_mass);
        a.pos = a.pos - correction * a.inv_mass;
        b.pos = b.pos + correction * b.inv_mass;

        // AABBs and triangles are simulated axis aligned (the renderer and the
        // broad phase treat them the same way), so they have no rotational
        // response: only circles pick up spin from an off centre contact.
        let spin_a = spin_inertia(a);
        let spin_b = spin_inertia(b);

        let relative_vel = point_velocity(b, offset_b) - point_velocity(a, offset_a);
        let vel_along_normal = relative_vel.dot(normal);
        if vel_along_normal > 0.0 {
            return; // already separating
        }

        let r_a_cross_n = cross(offset_a, normal);
        let r_b_cross_n = cross(offset_b, normal);
        let inv_mass_sum = total_inv_mass
            + r_a_cross_n * r_a_cross_n * spin_a
            + r_b_cross_n * r_b_cross_n * spin_b;
        if inv_mass_sum <= 0.0 {
            return;
        }

        let restitution = a.restitution.min(b.restitution);
        let normal_impulse = -(1.0 + restitution) * vel_along_normal / inv_mass_sum;
        apply_impulse(
            a,
            b,
            normal * normal_impulse,
            offset_a,
            offset_b,
            spin_a,
            spin_b,
        );

        // Friction: Coulomb clamped against the normal impulse, applied at the
        // contact point so it can also scrub off spin.
        let tangent = Vec2::new(-normal.y, normal.x);
        let relative_vel = point_velocity(b, offset_b) - point_velocity(a, offset_a);
        let vel_along_tangent = relative_vel.dot(tangent);

        let r_a_cross_t = cross(offset_a, tangent);
        let r_b_cross_t = cross(offset_b, tangent);
        let inv_mass_sum_t = total_inv_mass
            + r_a_cross_t * r_a_cross_t * spin_a
            + r_b_cross_t * r_b_cross_t * spin_b;
        if inv_mass_sum_t <= 0.0 {
            return;
        }

        let friction = (a.friction + b.friction) * 0.5;
        let max_friction = normal_impulse * friction;
        let tangent_impulse =
            (-vel_along_tangent / inv_mass_sum_t).clamp(-max_friction, max_friction);

        apply_impulse(
            a,
            b,
            tangent * tangent_impulse,
            offset_a,
            offset_b,
            spin_a,
            spin_b,
        );
    }
}

/// Contact between two bodies, if their shapes overlap. `normal` points from `a`
/// to `b`; `None` means the pair is separated.
fn narrow_phase(a: &Body, b: &Body) -> Option<Manifold> {
    match (&a.shape, &b.shape) {
        (Shape::Circle { .. }, Shape::Circle { .. }) => circle_circle(a, b),
        (Shape::Circle { .. }, _) => circle_polygon(a, b),
        (_, Shape::Circle { .. }) => circle_polygon(b, a).map(Manifold::flipped),
        _ => polygon_polygon(a, b),
    }
}

fn circle_circle(a: &Body, b: &Body) -> Option<Manifold> {
    let (Shape::Circle { radius: r_a }, Shape::Circle { radius: r_b }) = (&a.shape, &b.shape)
    else {
        return None;
    };
    let (r_a, r_b) = (*r_a, *r_b);

    let sum = r_a + r_b;
    let delta = b.pos - a.pos;
    let dist_sq = delta.length_squared();
    if dist_sq >= sum * sum || dist_sq <= f64::EPSILON {
        return None; // separated, or concentric with no usable normal
    }

    let dist = dist_sq.sqrt();
    let normal = delta / dist;

    Some(Manifold {
        normal,
        penetration: sum - dist,
        offset_a: normal * r_a,
        offset_b: -normal * r_b,
    })
}

/// Circle against a convex polygon (`polygon` may be either body's shape).
fn circle_polygon(circle: &Body, polygon: &Body) -> Option<Manifold> {
    let Shape::Circle { radius } = circle.shape else {
        return None;
    };

    let (local, len) = polygon_vertices(&polygon.shape);
    if len == 0 {
        return None;
    }
    let verts = world_vertices(&local, len, polygon.pos);

    // Closest point on the polygon's boundary, and the edge owning it.
    let mut closest = verts[0];
    let mut best_dist_sq = f64::INFINITY;
    let mut best_edge = 0;
    for k in 0..len {
        let point = closest_point_on_segment(circle.pos, verts[k], verts[(k + 1) % len]);
        let dist_sq = (point - circle.pos).length_squared();
        if dist_sq < best_dist_sq {
            best_dist_sq = dist_sq;
            closest = point;
            best_edge = k;
        }
    }

    let centre = centroid(&verts, len);
    let dist = best_dist_sq.sqrt();

    // Convex: the centre is inside when it is behind every edge plane.
    let mut inside = true;
    for k in 0..len {
        if (circle.pos - verts[k]).dot(outward_normal(&verts, len, k, centre)) > 0.0 {
            inside = false;
            break;
        }
    }

    let (normal, penetration) = if inside {
        // The centre is inside: escape through the closest edge, and the normal
        // still points from the circle towards the polygon.
        if dist <= f64::EPSILON {
            (-outward_normal(&verts, len, best_edge, centre), radius)
        } else {
            ((circle.pos - closest) / dist, radius + dist)
        }
    } else {
        if dist >= radius {
            return None; // separated
        }
        ((closest - circle.pos) / dist, radius - dist)
    };

    let contact = circle.pos + normal * radius;

    Some(Manifold {
        normal,
        penetration,
        offset_a: normal * radius,
        offset_b: contact - polygon.pos,
    })
}

/// Convex polygon against convex polygon, using the separating axis theorem.
fn polygon_polygon(a: &Body, b: &Body) -> Option<Manifold> {
    let (local_a, len_a) = polygon_vertices(&a.shape);
    let (local_b, len_b) = polygon_vertices(&b.shape);
    if len_a == 0 || len_b == 0 {
        return None;
    }
    let verts_a = world_vertices(&local_a, len_a, a.pos);
    let verts_b = world_vertices(&local_b, len_b, b.pos);

    let mut normal = Vec2::ZERO;
    let mut penetration = f64::INFINITY;

    // Every edge normal of both polygons is a candidate separating axis.
    for (verts, len) in [(&verts_a, len_a), (&verts_b, len_b)] {
        for k in 0..len {
            let edge = verts[(k + 1) % len] - verts[k];
            let axis = Vec2::new(-edge.y, edge.x).normalized();
            if axis == Vec2::ZERO {
                continue; // degenerate edge: no axis
            }

            let (min_a, max_a) = project(&verts_a, len_a, axis);
            let (min_b, max_b) = project(&verts_b, len_b, axis);
            let overlap = max_a.min(max_b) - min_a.max(min_b);
            if overlap <= 0.0 {
                return None; // separating axis found
            }

            if overlap < penetration {
                penetration = overlap;
                normal = axis;
            }
        }
    }

    if normal == Vec2::ZERO {
        return None; // both polygons degenerate
    }
    // The axis sign is arbitrary; orient it from `a` to `b`.
    if normal.dot(b.pos - a.pos) < 0.0 {
        normal = -normal;
    }

    Some(Manifold {
        normal,
        penetration,
        // Neither polygon rotates, so no torque arm is needed.
        offset_a: Vec2::ZERO,
        offset_b: Vec2::ZERO,
    })
}

/// Body space vertices of a convex polygon shape, plus how many are used.
/// Circles report zero vertices.
fn polygon_vertices(shape: &Shape) -> ([Vec2; MAX_POLY_VERTICES], usize) {
    match shape {
        Shape::Circle { .. } => ([Vec2::ZERO; MAX_POLY_VERTICES], 0),
        Shape::Aabb { half_extents } => {
            let (hx, hy) = *half_extents;
            (
                [
                    Vec2::new(-hx, -hy),
                    Vec2::new(hx, -hy),
                    Vec2::new(hx, hy),
                    Vec2::new(-hx, hy),
                ],
                MAX_POLY_VERTICES,
            )
        }
        // Matches the triangle the renderer draws: apex up, base down.
        Shape::Triangle { size } => {
            let half = *size * 0.5;
            (
                [
                    Vec2::new(0.0, -half),
                    Vec2::new(-half, half),
                    Vec2::new(half, half),
                    Vec2::ZERO,
                ],
                3,
            )
        }
    }
}

fn world_vertices(
    local: &[Vec2; MAX_POLY_VERTICES],
    len: usize,
    pos: Vec2,
) -> [Vec2; MAX_POLY_VERTICES] {
    let mut out = [Vec2::ZERO; MAX_POLY_VERTICES];
    for k in 0..len {
        out[k] = pos + local[k];
    }
    out
}

/// Interior point of a convex polygon; used to orient edge normals outwards.
fn centroid(verts: &[Vec2; MAX_POLY_VERTICES], len: usize) -> Vec2 {
    let mut sum = Vec2::ZERO;
    for v in verts.iter().take(len) {
        sum = sum + *v;
    }
    sum / len as f64
}

/// Outward unit normal of the edge from vertex `k` to vertex `k + 1`.
fn outward_normal(verts: &[Vec2; MAX_POLY_VERTICES], len: usize, k: usize, centre: Vec2) -> Vec2 {
    let start = verts[k];
    let edge = verts[(k + 1) % len] - start;
    let mut normal = Vec2::new(-edge.y, edge.x).normalized();
    if normal == Vec2::ZERO {
        return normal;
    }
    if (start + edge * 0.5 - centre).dot(normal) < 0.0 {
        normal = -normal;
    }
    normal
}

fn project(verts: &[Vec2; MAX_POLY_VERTICES], len: usize, axis: Vec2) -> (f64, f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for v in verts.iter().take(len) {
        let p = v.dot(axis);
        min = min.min(p);
        max = max.max(p);
    }
    (min, max)
}

fn closest_point_on_segment(p: Vec2, a: Vec2, b: Vec2) -> Vec2 {
    let ab = b - a;
    let len_sq = ab.length_squared();
    if len_sq <= f64::EPSILON {
        return a;
    }
    let t = ((p - a).dot(ab) / len_sq).clamp(0.0, 1.0);
    a + ab * t
}

/// Scalar 2D cross product `a x b`.
fn cross(a: Vec2, b: Vec2) -> f64 {
    a.x * b.y - a.y * b.x
}

/// Velocity of the point `offset` (from the centre of mass) on a body.
fn point_velocity(body: &Body, offset: Vec2) -> Vec2 {
    body.vel + Vec2::new(-body.angular_vel * offset.y, body.angular_vel * offset.x)
}

/// Rotational response of a body. Polygons are simulated axis aligned here, so
/// they never spin; circles rotate freely.
fn spin_inertia(body: &Body) -> f64 {
    match body.shape {
        Shape::Circle { .. } => body.inv_inertia,
        Shape::Aabb { .. } | Shape::Triangle { .. } => 0.0,
    }
}

fn apply_impulse(
    a: &mut Body,
    b: &mut Body,
    impulse: Vec2,
    offset_a: Vec2,
    offset_b: Vec2,
    spin_a: f64,
    spin_b: f64,
) {
    a.vel = a.vel - impulse * a.inv_mass;
    b.vel = b.vel + impulse * b.inv_mass;
    a.angular_vel -= spin_a * cross(offset_a, impulse);
    b.angular_vel += spin_b * cross(offset_b, impulse);
}

use core::dynamics::{Body, PhysicsWorld};
use core::math::Vec2;

fn main() {
    let mut world = PhysicsWorld::new();
    world.add(Body {
        id: 1,
        pos: Vec2::new(0.0, 10.0),
        vel: Vec2::ZERO,
        radius: 1.0,
    });
    world.add(Body {
        id: 2,
        pos: Vec2::new(0.0, 8.5),
        vel: Vec2::ZERO,
        radius: 1.0,
    });

    world.step(0.016);
    println!(
        "Physics simulation stepped successfully! Bodies: {}",
        world.bodies.len()
    );
}

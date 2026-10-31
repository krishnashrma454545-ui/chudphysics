use core::collision::shapes::Shape;
use core::dynamics::PhysicsWorld;
use core::dynamics::body::Body;
use core::math::vec2::Vec2;

use macroquad::prelude::*;

#[macroquad::main("physical Visual Test")]
async fn main() {
    let mut world = PhysicsWorld::new();
    world.gravity = Vec2::new(0.0, 9.81);
    let floor_y = 300.0;

    let r = 30.0;
    let mass = 1.0;
    world.bodies.push(Body {
        id: 1,
        pos: Vec2::new(-10.0, -200.0),
        vel: Vec2::new(0.0, 0.0),
        shape: Shape::Circle { radius: r },
        restitution: 0.5,
        friction: 0.5,
        mass,
        inv_mass: 1.0,
        angle: 0.0,
        angular_vel: 0.0,
        inertia: 0.5 * mass * (r * r),
        inv_inertia: 1.0 / (0.5 * mass * (r * r)),
    });

    let he = 30.0;
    let mass = 1.0;
    let w = he * 2.0;
    let inertia = (mass * (w * w + w * w)) / 12.0;
    world.bodies.push(Body {
        id: 2,
        pos: Vec2::new(0.0, -300.0),
        vel: Vec2::new(0.0, 0.0),
        shape: Shape::Aabb {
            half_extents: (he, he),
        },
        restitution: 0.5,
        friction: 0.5,
        mass,
        inv_mass: 1.0 / mass,
        angle: 0.0,
        angular_vel: 0.0,
        inertia,
        inv_inertia: 1.0 / inertia,
    });

    let size = 40.0;
    let mass = 3.0;

    world.bodies.push(Body {
        id: 3,
        pos: Vec2::new(100.0, -400.0),
        vel: Vec2::new(-1.0, 0.0),
        shape: Shape::Triangle { size },
        restitution: 0.8,
        friction: 0.5,
        mass,
        inv_mass: 1.0 / mass,
        angle: 0.0,
        angular_vel: 0.0,
        inertia: 500.0,
        inv_inertia: 1.0 / 500.0,
    });

    let mut accumulator = 0.0;
    let fixed_dt = 1.0 / 60.0;

    loop {
        clear_background(BLACK);

        let frame_time = get_frame_time() as f64;
        accumulator += frame_time;

        while accumulator >= fixed_dt {
            world.step(fixed_dt);
            accumulator -= fixed_dt;
        }

        let screen_floor = screen_height() / 2.0 + floor_y as f32;
        draw_line(0.0, screen_floor, screen_width(), screen_floor, 4.0, WHITE);

        for body in &world.bodies {
            let draw_x = screen_width() / 2.0 + body.pos.x as f32;
            let draw_y = screen_height() / 2.0 + body.pos.y as f32;

            match &body.shape {
                Shape::Circle { radius } => {
                    draw_circle(draw_x, draw_y, *radius as f32, GREEN);

                    let line_end_x = draw_x + (*radius as f32) * body.angle.cos() as f32;
                    let line_end_y = draw_y + (*radius as f32) * body.angle.sin() as f32;
                    draw_line(draw_x, draw_y, line_end_x, line_end_y, 2.0, WHITE);
                }

                Shape::Aabb { half_extents } => {
                    let w = (half_extents.0 * 2.0) as f32;
                    let h = (half_extents.1 * 2.0) as f32;

                    draw_rectangle(draw_x - w / 2.0, draw_y - h / 2.0, w, h, BLUE);
                }
                Shape::Triangle { size } => {
                    let s = *size as f32;
                    let p1 = vec2(draw_x, draw_y - s / 2.0);
                    let p2 = vec2(draw_x - s / 2.0, draw_y + s / 2.0);
                    let p3 = vec2(draw_x + s / 2.0, draw_y + s / 2.0);
                    draw_triangle(p1, p2, p3, RED);
                }
            }
        }
        next_frame().await;
    }
}

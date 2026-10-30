use core::collision::shapes::Shape;
use core::dynamics::PhysicsWorld;
use core::dynamics::body::Body;
use core::math::vec2::Vec2;

use macroquad::prelude::*;

#[macroquad::main("Physics Visual Test")]
async fn main() {
    let mut world = PhysicsWorld::new();
    world.gravity = Vec2::new(0.0, 9.81);
    let floor_y = 300.0;

    world.bodies.push(Body {
        id: 1,
        pos: Vec2::new(-100.0, -200.0),
        vel: Vec2::new(1.0, 0.0),
        shape: Shape::Circle { radius: 20.0 },
        restitution: 1.0,
    });

    world.bodies.push(Body {
        id: 2,
        pos: Vec2::new(0.0, -300.0),
        vel: Vec2::new(0.0, 0.0),
        shape: Shape::Aabb {
            half_extents: (30.0, 30.0),
        },
        restitution: 0.5,
    });

    loop {
        clear_background(BLACK);
        world.step(1.0 / 60.0);

        let screen_floor = screen_height() / 2.0 + floor_y as f32;
        draw_line(0.0, screen_floor, screen_width(), screen_floor, 4.0, WHITE);

        for body in &world.bodies {
            let draw_x = screen_width() / 2.0 + body.pos.x as f32;
            let draw_y = screen_height() / 2.0 + body.pos.y as f32;

            match &body.shape {
                Shape::Circle { radius } => {
                    draw_circle(draw_x, draw_y, *radius as f32, GREEN);
                }
                Shape::Aabb { half_extents } => {
                    let w = (half_extents.1 * 2.0) as f32;
                    let h = (half_extents.1 * 2.0) as f32;
                    draw_rectangle(draw_x - w / 2.0, draw_y - h / 2.0, w, h, BLUE);
                }
                Shape::Triangle { size } => {
                    let s = *size as f32;
                    let p1 = vec2(draw_x, draw_y - s / 2.0);
                    let p2 = vec2(draw_x + s / 2.0, draw_y + s / 2.0);
                    let p3 = vec2(draw_x + s / 2.0, draw_y + s / 2.0);
                    draw_triangle(p1, p2, p3, RED);
                }
            }
        }
        next_frame().await;
    }
}

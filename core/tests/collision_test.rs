use core::collision::broad::BroadPhase;
use core::collision::narrow::circle_circle;
use core::dynamics::Body;
use core::math::Vec2;

#[test]
fn test_circle_collision_detected() {
    let a_pos = Vec2::new(0.0, 0.0);
    let b_pos = Vec2::new(1.5, 0.0);
    let contact = circle_circle(a_pos, 1.0, b_pos, 1.0);

    assert!(contact.is_some());
    let c = contact.unwrap();
    assert!((c.penetration - 0.5).abs() < 1e-6);
}

#[test]
fn test_circle_no_collision_when_separated() {
    let a_pos = Vec2::new(0.0, 0.0);
    let b_pos = Vec2::new(5.0, 0.0);
    let contact = circle_circle(a_pos, 1.0, b_pos, 1.0);

    assert!(contact.is_none());
}

#[test]
fn test_broadphase_finds_candidates() {
    let mut broad = BroadPhase::new(2.0);
    let bodies = vec![
        Body {
            id: 0,
            pos: Vec2::new(0.0, 0.0),
            vel: Vec2::ZERO,
            radius: 1.0,
        },
        Body {
            id: 1,
            pos: Vec2::new(1.0, 0.0),
            vel: Vec2::ZERO,
            radius: 1.0,
        },
        Body {
            id: 2,
            pos: Vec2::new(100.0, 100.0),
            vel: Vec2::ZERO,
            radius: 1.0,
        },
    ];

    let pairs = broad.candidates(&bodies);
    assert_eq!(pairs, vec![(0, 1)]);
}

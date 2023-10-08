// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use collision::{Shape, test_static_vs_static, DummyVec, shape::Project};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,    setup )
        .add_systems(Update,     (update_inactive, update_active, check_colliders))
        .add_systems(PostUpdate, render)
        .add_systems(Last, cleanup)
        .run();
}

#[derive(Component)]
pub struct StaticCollider(Shape);

impl StaticCollider {

    const RECT:    collision::shape::Rect    = collision::shape::Rect::new(Vec2::ZERO, Vec2::new(100.0, 100.0));
    const CIRCLE:  collision::shape::Circle  = collision::shape::Circle::new(Vec2::ZERO, 50.0);
    const CAPSULE: collision::shape::Capsule = collision::shape::Capsule::new(Vec2::ZERO, 25.0, 100.0);
    const LINE:    collision::shape::Line    = collision::shape::Line::from_raw(Vec2::ZERO, Vec2::new(100.0, 100.0), Vec2::new(-std::f32::consts::FRAC_1_SQRT_2, std::f32::consts::FRAC_1_SQRT_2));
    const SLOPE:   collision::shape::Slope   = collision::shape::Slope::from_raw(Vec2::ZERO, 100.0, 100.0, 0.00707106);

    pub fn new(origin: Vec2) -> Self {
        Self(Self::CIRCLE.with_offset(origin).into())
    }

    pub fn next(&mut self) {
        self.0 = match self.0 {
            Shape::Point(s)   => Self::LINE.with_offset(s).into(),
            Shape::Line(s)    => Self::CIRCLE.with_offset(s.start()).into(),
            Shape::Circle(s)  => Self::RECT.with_offset(s.origin).into(),
            Shape::Rect(s)    => Self::CAPSULE.with_offset(s.min).into(),
            Shape::Capsule(s) => Self::SLOPE.with_offset(s.start).into(),
            Shape::Slope(s)   => s.origin().into(),
        }
    }

}

#[derive(Component)]
pub struct ColliderPair {
    shape_1: Entity,
    shape_2: Entity,
}

#[derive(Component)]
pub struct ActiveCollider;

fn setup(mut commands: Commands) {

    commands.spawn(Camera2dBundle::default());

    commands.spawn((StaticCollider::new(Vec2::X * -200.0), ActiveCollider));
    commands.spawn( StaticCollider::new(Vec2::X *  200.0));
}

fn update_inactive(
    mut q_inactive: Query<&mut StaticCollider, Without<ActiveCollider>>, 
    keys: Res<Input<KeyCode>>,
    time: Res<Time>
 ) {
    let mut offset = Vec2::ZERO;

    if keys.pressed(KeyCode::I) {
        offset += Vec2::Y;
    }

    if keys.pressed(KeyCode::J) {
        offset -= Vec2::X;
    }

    if keys.pressed(KeyCode::K) {
        offset -= Vec2::Y;
    }

    if keys.pressed(KeyCode::L) {
        offset += Vec2::X;
    }

    for mut collider in q_inactive.iter_mut() {
        if keys.just_pressed(KeyCode::Backslash) {
            collider.next();
        }

        if offset != Vec2::ZERO {
            offset *= 200.0 * time.delta_seconds();
            collider.0 = collider.0.with_offset(offset);
        }
    }
}

fn update_active(
    mut q_active:   Query<&mut StaticCollider,    With<ActiveCollider>>, 
    keys: Res<Input<KeyCode>>, 
    time: Res<Time>
) {

    let mut collider = q_active.get_single_mut().unwrap();

    if keys.just_pressed(KeyCode::Tab) {
        collider.next();
    }

    let mut offset = Vec2::ZERO;

    if keys.pressed(KeyCode::W) {
        offset += Vec2::Y;
    }

    if keys.pressed(KeyCode::A) {
        offset -= Vec2::X;
    }

    if keys.pressed(KeyCode::S) {
        offset -= Vec2::Y;
    }

    if keys.pressed(KeyCode::D) {
        offset += Vec2::X;
    }

    if offset != Vec2::ZERO {
        offset *= 200.0 * time.delta_seconds();
        collider.0 = collider.0.with_offset(offset);
    }

}

fn check_colliders(q: Query<(Entity, &StaticCollider)>, mut commands: Commands) {
    
    for (i, (shape_1, StaticCollider(shape_a))) in q.iter().enumerate() {
        for (shape_2, StaticCollider(shape_b)) in q.iter().skip(i+1) {
            if test_static_vs_static::<false>(shape_a, shape_b, &mut DummyVec::default()) {
                commands.spawn(ColliderPair{shape_1, shape_2});
            }
        }
    }

}

fn render(mut gizmos: Gizmos, q: Query<(Entity, &StaticCollider)>, q_overlaps: Query<&ColliderPair>) {
    for (entity, StaticCollider(shape)) in q.iter() {
        let overlapped = q_overlaps.iter().any(|v| entity == v.shape_1 || entity == v.shape_2);
        let color =  if overlapped { Color::RED } else { Color::GREEN };
        match shape {
            Shape::Point(s) => {
                gizmos.circle_2d(*s, 1.0, color);
            },
            Shape::Line(s) => {
                gizmos.line_2d(s.start(), s.end(), color);
            },
            Shape::Circle(s) => {
                gizmos.circle_2d(s.origin, s.radius, color);
            },
            Shape::Rect(s) => {
                let size = s.max - s.min;

                gizmos.rect_2d(s.min + size*0.5, 0.0, size, color);
            },
            Shape::Capsule(s) => {
                let mid = (s.start + s.end())*0.5;
                gizmos.rect_2d(mid, 0.0, Vec2::new(2.0*s.radius, s.height), color);
                gizmos.circle_2d(s.start, s.radius, color);
                gizmos.circle_2d(s.end(), s.radius, color);
            },
            Shape::Slope(s) => {
                let points = s.points();
                gizmos.linestrip_2d([points[0], points[1], points[2], points[0]], color);
            },
        }

    }
}

fn cleanup(q: Query<Entity, With<ColliderPair>>, mut commands: Commands) {
    for entity in q.iter() {
        commands.entity(entity).despawn();
    }

}

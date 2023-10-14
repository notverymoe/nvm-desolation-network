// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use collision_2::{RectRoundedData, RectData, CircleData, ShapeData, Shape, RayCaster, RaycastTarget, Projection};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,    setup )
        .add_systems(Update,     (update_raycaster, update_static, check_colliders).chain())
        .add_systems(PostUpdate, render)
        .run();
}

#[derive(Component)]
pub struct StaticCollider(Shape);

impl StaticCollider {

    const RECT:         RectData        = RectData::new(Vec2::new(100.0, 100.0));
    const CIRCLE:       CircleData      = CircleData::new(50.0);
    const RECT_ROUNDED: RectRoundedData = RectRoundedData::new(Vec2::new(100.0, 100.0), 50.0);

    pub fn new(origin: Vec2) -> Self {
        Self(Shape::new(origin, Self::CIRCLE))
    }

    pub fn next(&mut self) {
        self.0.data = match self.0.data {
            ShapeData::Circle(_)      => Self::RECT.into(),
            ShapeData::Rect(_)        => Self::RECT_ROUNDED.into(),
            ShapeData::RectRounded(_) => Self::CIRCLE.into(),
        }
    }

}

#[derive(Component)]
pub struct RayCasterCollider {
    origin: Vec2,
    target: Vec2,

    hits: Vec<(Entity, Projection)>,
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(StaticCollider::new(Vec2::X * 200.0));
    commands.spawn(RayCasterCollider{origin: -Vec2::X * 200.0, target: Vec2::ZERO, hits: Vec::default()});
}

fn update_static(
    mut q: Query<&mut StaticCollider>, 
    keys: Res<Input<KeyCode>>
) {
    if keys.just_pressed(KeyCode::Backslash) {
        for mut collider in q.iter_mut() {
            collider.next();
        }
    }
}

fn update_raycaster(
    mut q: Query<&mut RayCasterCollider>, 
    keys: Res<Input<KeyCode>>,
    time: Res<Time>
 ) {
    let mut caster = q.get_single_mut().unwrap();
    let mut offset_origin  = Vec2::ZERO;
    let mut offset_target = Vec2::ZERO;

    if keys.pressed(KeyCode::W) {
        offset_origin += Vec2::Y;
    }

    if keys.pressed(KeyCode::A) {
        offset_origin -= Vec2::X;
    }

    if keys.pressed(KeyCode::S) {
        offset_origin -= Vec2::Y;
    }

    if keys.pressed(KeyCode::D) {
        offset_origin += Vec2::X;
    }

    if offset_origin != Vec2::ZERO {
        offset_origin *= 200.0 * time.delta_seconds();
        caster.origin += offset_origin;
    }

    if keys.pressed(KeyCode::I) {
        offset_target += Vec2::Y;
    }

    if keys.pressed(KeyCode::J) {
        offset_target -= Vec2::X;
    }

    if keys.pressed(KeyCode::K) {
        offset_target -= Vec2::Y;
    }

    if keys.pressed(KeyCode::L) {
        offset_target += Vec2::X;
    }

    if offset_target != Vec2::ZERO {
        offset_target *= 200.0 * time.delta_seconds();
        caster.target += offset_target;
    }
}

fn check_colliders(
    mut q_caster:  Query<&mut RayCasterCollider>,
    q_static: Query<(Entity, &StaticCollider)>,
) {
    for mut caster in q_caster.iter_mut() {
        caster.hits.clear();
        let ray = RayCaster::new(caster.origin, (caster.target - caster.origin).normalize());
        for (shape_id, StaticCollider(shape)) in q_static.iter() {
            if let Some(projection) = shape.raycast(&ray) {
                caster.hits.push((shape_id, projection));
            }
        }
    }

}

fn render(
    mut gizmos: Gizmos, 
    q_caster:  Query<&RayCasterCollider>,
    q_static: Query<(Entity, &StaticCollider)>,
) {
    for (shape_id, StaticCollider(shape)) in q_static.iter() {
        render_shape(&mut gizmos, shape, if q_caster.iter().any(|v| v.hits.iter().any(|v| shape_id == v.0)) {
            Color::RED
        } else {
            Color::GREEN
        });
    }

    for caster in q_caster.iter() {
        let dir = (caster.target - caster.origin).normalize();

        gizmos.circle_2d(caster.origin, 10.0, Color::GREEN);
        gizmos.circle_2d(caster.target, 10.0, Color::GREEN);
        gizmos.line_2d(caster.origin - dir*6000.0, caster.target + dir*6000.0, if caster.hits.is_empty() { Color::LIME_GREEN } else { Color::PINK });

        for (_, hit) in caster.hits.iter() {
            let [start, end] = hit.get_points(dir).map(|v| caster.origin + v);
            gizmos.line_2d(start, end, Color::PURPLE);
        }
        
    }
}

fn render_shape(gizmos: &mut Gizmos, shape: &Shape, color: Color) {
    match shape.data {
        ShapeData::Circle(s) => {
            gizmos.circle_2d(shape.origin, s.radius, color);
        },
        ShapeData::Rect(s) => {
            gizmos.rect_2d(shape.origin, 0.0, s.size*2.0, color);
        },
        ShapeData::RectRounded(s) => {
            gizmos.arc_2d(shape.origin + Vec2::new( s.size.x,  s.size.y), f32::to_radians( 45.0), f32::to_radians(90.0), s.radius, color);
            gizmos.arc_2d(shape.origin + Vec2::new(-s.size.x,  s.size.y), f32::to_radians(315.0), f32::to_radians(90.0), s.radius, color);
            gizmos.arc_2d(shape.origin + Vec2::new(-s.size.x, -s.size.y), f32::to_radians(225.0), f32::to_radians(90.0), s.radius, color);
            gizmos.arc_2d(shape.origin + Vec2::new( s.size.x, -s.size.y), f32::to_radians(135.0), f32::to_radians(90.0), s.radius, color);

            gizmos.line_2d(
                shape.origin + Vec2::new( s.size.x, s.size.y + s.radius ),
                shape.origin + Vec2::new(-s.size.x, s.size.y + s.radius ),
                color
            );

            gizmos.line_2d(
                shape.origin + Vec2::new(-(s.size.x + s.radius),  s.size.y),
                shape.origin + Vec2::new(-(s.size.x + s.radius), -s.size.y),
                color
            );

            gizmos.line_2d(
                shape.origin + Vec2::new(-s.size.x, -(s.size.y + s.radius)),
                shape.origin + Vec2::new( s.size.x, -(s.size.y + s.radius)),
                color
            );

            gizmos.line_2d(
                shape.origin + Vec2::new(s.size.x + s.radius, -s.size.y),
                shape.origin + Vec2::new(s.size.x + s.radius,  s.size.y),
                color
            );
        },
    }
}


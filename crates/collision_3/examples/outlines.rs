// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};
use collision_3::{DebugShape, Ball, BoxAligned, BoxAlignedRound, Ramp, RampRound, RampBoxy, RampBoxyRound, BoxOriented, BoxOrientedRound, BoxOrientedBoxy, BoxOrientedBoxyRound, DebugShapeData, RayCaster, RaycastTarget, RayIntersection};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup,    setup )
        .add_systems(Update,     (update_raycaster, check_colliders).chain())
        .add_systems(PostUpdate, render)
        .run();
}

trait ShapeMarkerTrait: DebugShape + RaycastTarget + Send + Sync {}
impl<T: DebugShape + RaycastTarget + Send + Sync + 'static> ShapeMarkerTrait for T {}

#[derive(Component)]
pub struct Shape(Box<dyn ShapeMarkerTrait>);

impl Shape {
    fn new(v: impl ShapeMarkerTrait + 'static) -> Self {
        Self(Box::new(v))
    }
}

#[derive(Component)]
pub struct RayCasterCollider {
    origin: Vec2,
    target: Vec2,
    hits: Vec<(Entity, [RayIntersection; 2])>,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(RayCasterCollider{origin: -Vec2::X * 200.0, target: Vec2::ZERO, hits: Vec::default()});

    commands.spawn(Shape::new(Ball::new(Vec2::ZERO, 50.0)));

    commands.spawn(Shape::new(BoxAligned::new(Vec2::X*-200.0, Vec2::new(100.0, 50.0))));
    commands.spawn(Shape::new(BoxAlignedRound::new(Vec2::X*-200.0 + Vec2::Y*200.0, Vec2::new(100.0, 50.0), 25.0)));

    commands.spawn(Shape::new(BoxOriented::new(Vec2::X*-500.0 + Vec2::Y*25.0, Vec2::new(100.0, 50.0), Vec2::new(2.0, 1.0).normalize())));
    commands.spawn(Shape::new(BoxOrientedRound::new(Vec2::X*-500.0 + Vec2::Y*200.0, Vec2::new(100.0, 50.0), Vec2::new(2.0, 1.0).normalize(), 25.0)));
    commands.spawn(Shape::new( BoxOrientedBoxy::new(Vec2::X*550.0 + Vec2::Y*350.0, Vec2::new(100.0, 50.0), Vec2::new(2.0, 1.0).normalize(), Vec2::new(50.0, 25.0))));
    commands.spawn(Shape::new(BoxOrientedBoxyRound::new(Vec2::X*-650.0 + Vec2::Y*350.0, Vec2::new(100.0, 50.0), Vec2::new(2.0, 1.0).normalize(), Vec2::new(50.0, 25.0), 25.0)));

    commands.spawn(Shape::new(Ramp::new(Vec2::X*200.0 + Vec2::Y*200.0 + Vec2::new( 20.0,  20.0), Vec2::new( 2.0, -1.0).normalize(), 200.0)));
    commands.spawn(Shape::new(Ramp::new(Vec2::X*200.0 + Vec2::Y*200.0 + Vec2::new(-20.0,  20.0), Vec2::new(-2.0, -1.0).normalize(), 200.0)));
    commands.spawn(Shape::new(Ramp::new(Vec2::X*200.0 + Vec2::Y*200.0 + Vec2::new(-20.0, -20.0), Vec2::new(-2.0,  1.0).normalize(), 200.0)));
    commands.spawn(Shape::new(Ramp::new(Vec2::X*200.0 + Vec2::Y*200.0 + Vec2::new( 20.0, -20.0), Vec2::new( 2.0,  1.0).normalize(), 200.0)));

    commands.spawn(Shape::new(RampBoxy::new(Vec2::X*200.0 - Vec2::Y*250.0 + Vec2::new( 110.0,  45.0), Vec2::new( 2.0, -1.0).normalize(), 200.0, Vec2::new(50.0, 25.0))));
    commands.spawn(Shape::new(RampBoxy::new(Vec2::X*200.0 - Vec2::Y*250.0 + Vec2::new(-110.0,  45.0), Vec2::new(-2.0, -1.0).normalize(), 200.0, Vec2::new(50.0, 25.0))));
    commands.spawn(Shape::new(RampBoxy::new(Vec2::X*200.0 - Vec2::Y*250.0 + Vec2::new(-110.0, -45.0), Vec2::new(-2.0,  1.0).normalize(), 200.0, Vec2::new(50.0, 25.0))));
    commands.spawn(Shape::new(RampBoxy::new(Vec2::X*200.0 - Vec2::Y*250.0 + Vec2::new( 110.0, -45.0), Vec2::new( 2.0,  1.0).normalize(), 200.0, Vec2::new(50.0, 25.0))));

    commands.spawn(Shape::new(RampRound::new(Vec2::X*500.0 + Vec2::new( 30.0,  30.0), Vec2::new( 2.0, -1.0).normalize(), 200.0, 25.0)));
    commands.spawn(Shape::new(RampRound::new(Vec2::X*500.0 + Vec2::new(-30.0,  30.0), Vec2::new(-2.0, -1.0).normalize(), 200.0, 25.0)));
    commands.spawn(Shape::new(RampRound::new(Vec2::X*500.0 + Vec2::new(-30.0, -30.0), Vec2::new(-2.0,  1.0).normalize(), 200.0, 25.0)));
    commands.spawn(Shape::new(RampRound::new(Vec2::X*500.0 + Vec2::new( 30.0, -30.0), Vec2::new( 2.0,  1.0).normalize(), 200.0, 25.0)));

    commands.spawn(Shape::new(RampBoxyRound::new(Vec2::X*-500.0 - Vec2::Y*250.0 + Vec2::new( 115.0,  50.0), Vec2::new( 2.0, -1.0).normalize(), 200.0, Vec2::new(50.0, 25.0), 25.0)));
    commands.spawn(Shape::new(RampBoxyRound::new(Vec2::X*-500.0 - Vec2::Y*250.0 + Vec2::new(-115.0,  50.0), Vec2::new(-2.0, -1.0).normalize(), 200.0, Vec2::new(50.0, 25.0), 25.0)));
    commands.spawn(Shape::new(RampBoxyRound::new(Vec2::X*-500.0 - Vec2::Y*250.0 + Vec2::new(-115.0, -50.0), Vec2::new(-2.0,  1.0).normalize(), 200.0, Vec2::new(50.0, 25.0), 25.0)));
    commands.spawn(Shape::new(RampBoxyRound::new(Vec2::X*-500.0 - Vec2::Y*250.0 + Vec2::new( 115.0, -50.0), Vec2::new( 2.0,  1.0).normalize(), 200.0, Vec2::new(50.0, 25.0), 25.0)));

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
    q_static: Query<(Entity, &Shape)>,
) {
    for mut caster in q_caster.iter_mut() {
        caster.hits.clear();
        let ray = RayCaster::new(caster.origin, (caster.target - caster.origin).normalize());
        for (shape_id, Shape(shape)) in q_static.iter() {
            if let Some(projection) = shape.raycast(&ray) {
                caster.hits.push((shape_id, projection));
            }
        }
    }

}

fn render(
    mut gizmos: Gizmos, 
    q_shapes: Query<(Entity, &Shape)>,
    q_caster:  Query<&RayCasterCollider>,
) {

    let caster = q_caster.single();
    gizmos.line_2d(caster.origin, caster.target, if caster.hits.is_empty() { Color::GREEN } else { Color::LIME_GREEN });
    for hit in caster.hits.iter() {
        gizmos.circle_2d(hit.1[0].point, 10.0, Color::PURPLE       );
        gizmos.circle_2d(hit.1[1].point, 10.0, Color::MIDNIGHT_BLUE);
        gizmos.line_2d(hit.1[0].point, hit.1[1].point, Color::BLACK);
    }

    for (entity, Shape(shape)) in q_shapes.iter() {
        let data = shape.get_debug_shape_data();
        let colour = if caster.hits.iter().any(|v| v.0 == entity) { Color::RED } else { Color::PINK };
        match data {
            collision_3::DebugShapeData::Circle { origin, radius } => { 
                gizmos.circle_2d(origin, radius, colour); 
            },
            collision_3::DebugShapeData::Polygon { .. } => {
                let DebugShapeData::Polygon { points, .. } = &data else { unreachable!() };
                gizmos.linestrip_2d((0..points.len()).chain(std::iter::once(0)).map(|i| points[i]), colour);
                for ([from, to, norm], offset) in data.iter_segments() {
                    let off = to - from;
                    let off_n = off.normalize();
                    let near = offset + from + off_n*10.0;
                    let far  = offset + to   - off_n*10.0;

                    let mid = offset + from + off*0.5;
                    gizmos.line_2d(mid, mid + norm*20.0, Color::BLUE);
                    gizmos.circle_2d(near, 5.0, Color::CYAN);
                    gizmos.circle_2d(far, 5.0, Color::TEAL);
                }
            },
            collision_3::DebugShapeData::PolygonRound { radius, .. } => {
                for ([from, to, norm], offset) in data.iter_segments() {
                    let offset = norm * offset;
                    if radius > 0.0 {
                        gizmos.circle_2d(from, radius, Color::GREEN);
                    }
                    gizmos.line_2d(offset + from, offset + to, colour);
                    let mid = offset + (from + to)*0.5;
                    gizmos.line_2d(mid, mid + norm*20.0, Color::BLUE);
                }
            },
        };
    }

}
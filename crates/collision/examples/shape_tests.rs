// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};
use nvm_collision::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup,    setup )
        .add_systems(Update,     (update_static, update_raycaster, check_colliders).chain())
        .add_systems(PostUpdate, render)
        .run();
}

#[derive(Component)]
pub struct Shape(ShapeStatic, usize);

impl Shape {
    fn new() -> Self {
        Self(Self::get_shape_at_index(0), 0)
    }

    fn next(&mut self) {
        let next = (self.1+1) % 13;
        self.0 = Self::get_shape_at_index(next);
        self.1 = next;
    }

    fn get_shape_at_index(idx: usize) -> ShapeStatic {
        match idx {
             1 => BoxAligned::new(Vec2::ZERO, Vec2::new(100.0, 50.0)).into(),
             2 => BoxAlignedRound::new(Vec2::ZERO, Vec2::new(100.0, 50.0), 25.0).into(),
             3 => BoxOriented::new(Vec2::ZERO, Vec2::new(100.0, 50.0), Vec2::new(2.0, 1.0).normalize()).into(),
             4 => BoxOrientedRound::new(Vec2::ZERO, Vec2::new(100.0, 50.0), Vec2::new(2.0, 1.0).normalize(), 25.0).into(),
             5 => Ramp::new(Vec2::ZERO, Vec2::new( 2.0, -1.0).normalize(), 200.0).into(),
             6 => Ramp::new(Vec2::ZERO, Vec2::new(-2.0, -1.0).normalize(), 200.0).into(),
             7 => Ramp::new(Vec2::ZERO, Vec2::new(-2.0,  1.0).normalize(), 200.0).into(),
             8 => Ramp::new(Vec2::ZERO, Vec2::new( 2.0,  1.0).normalize(), 200.0).into(),
             9 => RampRound::new(Vec2::ZERO, Vec2::new( 2.0, -1.0).normalize(), 200.0, 25.0).into(),
            10 => RampRound::new(Vec2::ZERO, Vec2::new(-2.0, -1.0).normalize(), 200.0, 25.0).into(),
            11 => RampRound::new(Vec2::ZERO, Vec2::new(-2.0,  1.0).normalize(), 200.0, 25.0).into(),
            12 => RampRound::new(Vec2::ZERO, Vec2::new( 2.0,  1.0).normalize(), 200.0, 25.0).into(),
             _ => Ball::new(Vec2::ZERO, 50.0).into(),
        }
    }
}

#[derive(Component)]
pub struct RayCasterCollider {
    origin:    Vec2,
    direction: Vec2,
    hits: Vec<(Entity, [RayIntersection; 2])>,
    is_cube: bool,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(RayCasterCollider{origin: -Vec2::X * 200.0, direction: Vec2::X, hits: Vec::default(), is_cube: false});
    commands.spawn(Shape::new());
}

fn update_static(
    mut q: Query<&mut Shape>, 
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
    let mut offset_origin = Vec2::ZERO;
    let mut offset_target = 0.0;

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

    if keys.pressed(KeyCode::Q) {
        offset_target += 1.0;
    }

    if keys.pressed(KeyCode::E) {
        offset_target -= 1.0;
    }

    if keys.just_pressed(KeyCode::Tab) {
        caster.is_cube = !caster.is_cube;
    }

    if keys.pressed(KeyCode::ShiftLeft) {
        offset_origin *= 2.0;
        offset_target *= 2.0;
    }

    if offset_origin != Vec2::ZERO {
        offset_origin *= 150.0 * time.delta_seconds();
        caster.origin += offset_origin;
    }


    if offset_target != 0.0 {
        offset_target *= time.delta_seconds();
        caster.direction = caster.direction.rotate(Vec2::from_angle(offset_target)).normalize();
    }
}

fn make_caster_shape(origin: Vec2, is_cube: bool) -> ShapeMoving {
    if is_cube { 
        BoxAligned::new(origin, Vec2::new(100.0, 50.0)).into() 
    } else { 
        Ball::new(origin, 50.0).into() 
    }
}

fn check_colliders(
    mut q_caster:  Query<&mut RayCasterCollider>,
    q_static: Query<(Entity, &Shape)>,
) {
    for mut caster in q_caster.iter_mut() {
        caster.hits.clear();
        let ray = RayCaster::new(caster.origin, caster.direction);
        let caster_shape = make_caster_shape(caster.origin, caster.is_cube);
        for (shape_id, Shape(target_shape, _)) in q_static.iter() {
            let combined = ShapeCombined::between_moving_and_static(&caster_shape, target_shape);
            if let Some(projection) = combined.raycast(&ray) {
                caster.hits.push((shape_id, projection));
            }
        }
    }

}

fn render_shape(gizmos: &mut Gizmos, shape: &impl DebugShape, colour: Color) {
    let data = shape.get_debug_shape_data();
    match data {
        DebugShapeData::Circle { origin, radius } => { 
            gizmos.circle_2d(origin, radius, colour); 
        },
        DebugShapeData::Polygon { .. } => {
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
        DebugShapeData::PolygonRound { radius, .. } => {
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


fn render(
    mut gizmos: Gizmos, 
    q_shapes: Query<(Entity, &Shape)>,
    q_caster:  Query<&RayCasterCollider>,
) {

    let caster = q_caster.single();
    let caster_shape: ShapeMoving = make_caster_shape(caster.origin, caster.is_cube);

    render_shape(&mut gizmos, &caster_shape, Color::GREEN);

    let first_hit = caster.hits.iter().reduce(|p, c| if p.1[0].distance < c.1[0].distance { p } else { c });
    if let Some(first_hit) = first_hit {
        let hit_shape: ShapeMoving = make_caster_shape(first_hit.1[0].point, caster.is_cube);
        render_shape(&mut gizmos, &hit_shape, Color::RED);
    }


    gizmos.circle_2d(caster.origin, 10.0, Color::ORANGE_RED);
    gizmos.line_2d(caster.origin, caster.origin + caster.direction * 10000.0, if caster.hits.is_empty() { Color::GREEN } else { Color::LIME_GREEN });
    for hit in caster.hits.iter() {
        gizmos.circle_2d(hit.1[0].point, 10.0, Color::PURPLE       );
        gizmos.circle_2d(hit.1[1].point, 10.0, Color::MIDNIGHT_BLUE);
        gizmos.line_2d(hit.1[0].point, hit.1[1].point, Color::BLACK);
    }

    for (entity, Shape(shape, _)) in q_shapes.iter() {
        let colour = if caster.hits.iter().any(|v| v.0 == entity) { Color::RED } else { Color::PINK };
        render_shape(&mut gizmos, shape, colour);

        let combined = ShapeCombined::between_moving_and_static(&caster_shape, shape);
        render_shape(&mut gizmos, &combined, colour);
    }

}

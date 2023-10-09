// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use collision::{Shape, shape::Project, Sweep, SolverSweep};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,    setup )
        .add_systems(Update,     (update_sweep, update_static, check_colliders).chain())
        .add_systems(PostUpdate, render)
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
pub struct ColliderSweep {
    shape: StaticCollider,
    motion:   Vec2,
    time_of_impact: Option<f32>,
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(StaticCollider::new(Vec2::ZERO));
    commands.spawn(ColliderSweep{
        shape: StaticCollider::new(Vec2::X * -200.0),
        motion:   Vec2::X * 400.0,
        time_of_impact: Some(f32::INFINITY),
    });
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

fn update_sweep(
    mut q: Query<&mut ColliderSweep>, 
    keys: Res<Input<KeyCode>>,
    time: Res<Time>
 ) {
    let mut sweep = q.get_single_mut().unwrap();
    let mut offset_start  = Vec2::ZERO;
    let mut offset_motion = Vec2::ZERO;

    if keys.just_pressed(KeyCode::Tab) {
        sweep.shape.next();
    }

    if keys.pressed(KeyCode::W) {
        offset_start += Vec2::Y;
    }

    if keys.pressed(KeyCode::A) {
        offset_start -= Vec2::X;
    }

    if keys.pressed(KeyCode::S) {
        offset_start -= Vec2::Y;
    }

    if keys.pressed(KeyCode::D) {
        offset_start += Vec2::X;
    }

    if offset_start != Vec2::ZERO {
        offset_start *= 200.0 * time.delta_seconds();
        sweep.shape.0 = sweep.shape.0.with_offset(offset_start);
    }

    if keys.pressed(KeyCode::I) {
        offset_motion += Vec2::Y;
    }

    if keys.pressed(KeyCode::J) {
        offset_motion -= Vec2::X;
    }

    if keys.pressed(KeyCode::K) {
        offset_motion -= Vec2::Y;
    }

    if keys.pressed(KeyCode::L) {
        offset_motion += Vec2::X;
    }

    if offset_motion != Vec2::ZERO {
        offset_motion *= 200.0 * time.delta_seconds();
        sweep.motion += offset_motion;
    }
}

fn check_colliders(
    mut q_sweep:  Query<&mut ColliderSweep>,
    q_static: Query<&StaticCollider>,
) {
    for mut sweep_info in q_sweep.iter_mut() {
        let sweep = Sweep::new(sweep_info.shape.0, sweep_info.motion);
        let mut solver = SolverSweep::new(sweep);
        for StaticCollider(shape) in q_static.iter() {
            solver.test_static_pen(shape);
        }
        sweep_info.time_of_impact = if solver.is_empty() { None } else { solver.find_time_of_impact().map(|v| v.1) };
    }

}

fn render(
    mut gizmos: Gizmos, 
    q_sweep:  Query<&ColliderSweep>,
    q_static: Query<&StaticCollider>,
) {
    for StaticCollider(shape) in q_static.iter() {
        render_shape(&mut gizmos, shape, Color::BLUE);
    }

    for sweep_info in q_sweep.iter() {
        let color_start = if sweep_info.time_of_impact.is_some() { Color::RED  } else { Color::GREEN      };
        let colour_end  = if sweep_info.time_of_impact.is_some() { Color::PINK } else { Color::LIME_GREEN };
        render_shape(&mut gizmos, &sweep_info.shape.0, color_start);
        render_shape(&mut gizmos, &sweep_info.shape.0.with_offset(sweep_info.motion), colour_end);
        if let Some(toi) = sweep_info.time_of_impact {
            let start = sweep_info.shape.0.offset();
            let offset = sweep_info.motion + sweep_info.motion.normalize()*toi;
            gizmos.line_2d(start, start + offset, Color::PURPLE);
            render_shape(&mut gizmos, &sweep_info.shape.0.with_offset(offset), Color::PURPLE);
        }
    }
}

fn render_shape(gizmos: &mut Gizmos, shape: &Shape, color: Color) {
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


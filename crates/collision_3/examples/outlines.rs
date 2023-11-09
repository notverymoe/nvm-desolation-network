// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};
use collision_3::{DebugShape, Ball, BoxAligned, BoxAlignedRound, Ramp, RampRound, RampBoxy, RampBoxyRound, BoxOriented, BoxOrientedRound, BoxOrientedBoxy, BoxOrientedBoxyRound, DebugShapeData};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup,    setup )
        .add_systems(PostUpdate, render)
        .run();
}

#[derive(Component)]
pub struct Shape(Box<dyn DebugShape + Send + Sync + 'static>);

impl Shape {

    pub fn new(v: impl DebugShape + Send + Sync + 'static) -> Self {
        Self(Box::new(v))
    }

}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

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

fn render(mut gizmos: Gizmos, q_shapes: Query<&Shape>) {

    for Shape(shape) in q_shapes.iter() {
        let data = shape.get_debug_shape_data();
        match data {
            collision_3::DebugShapeData::Circle { origin, radius } => { 
                gizmos.circle_2d(origin, radius, Color::RED); 
            },
            collision_3::DebugShapeData::Polygon { .. } => {
                let DebugShapeData::Polygon { points, .. } = &data else { unreachable!() };
                gizmos.linestrip_2d((0..points.len()).chain(std::iter::once(0)).map(|i| points[i]), Color::RED);
                for ([from, to, norm], offset) in data.iter_segments() {
                    gizmos.line_2d(offset + from, offset + to, Color::RED);
                    let mid = offset + (from + to)*0.5;
                    gizmos.line_2d(mid, mid + norm*20.0, Color::BLUE);
                }
            },
            collision_3::DebugShapeData::PolygonRound { radius, .. } => {
                for ([from, to, norm], offset) in data.iter_segments() {
                    let offset = norm * offset;
                    if radius > 0.0 {
                        gizmos.circle_2d(from, radius, Color::GREEN);
                    }
                    gizmos.line_2d(offset + from, offset + to, Color::RED);
                    let mid = offset + (from + to)*0.5;
                    gizmos.line_2d(mid, mid + norm*20.0, Color::BLUE);
                }
            },
        };
    }

}
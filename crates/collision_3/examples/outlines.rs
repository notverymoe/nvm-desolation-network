// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};
use collision_3::{CollisionDebugShape, Circle, Rect, RectRounded, Slope, SlopeRounded, SlopeRected, SlopeRectedRounded, OrientedRect, OrientedRectRounded, OrientedRectRected, OrientedRectRectedRounded};

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
pub struct Shape(Box<dyn CollisionDebugShape + Send + Sync + 'static>);

impl Shape {

    pub fn new(v: impl CollisionDebugShape + Send + Sync + 'static) -> Self {
        Self(Box::new(v))
    }

}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(Shape::new(Circle::new(Vec2::ZERO, 50.0)));

    commands.spawn(Shape::new(       Rect::new(Vec2::X*-200.0, Vec2::new(100.0, 50.0))));
    commands.spawn(Shape::new(RectRounded::new(Vec2::X*-200.0 + Vec2::Y*200.0, Vec2::new(100.0, 50.0), 25.0)));

    commands.spawn(Shape::new(OrientedRect::new(Vec2::X*-500.0 + Vec2::Y*25.0, Vec2::new(100.0, 50.0), Vec2::new(2.0, 1.0).normalize())));
    commands.spawn(Shape::new(OrientedRectRounded::new(Vec2::X*-500.0 + Vec2::Y*200.0, Vec2::new(100.0, 50.0), Vec2::new(2.0, 1.0).normalize(), 25.0)));
    commands.spawn(Shape::new( OrientedRectRected::new(Vec2::X*550.0 + Vec2::Y*350.0, Vec2::new(100.0, 50.0), Vec2::new(2.0, 1.0).normalize(), Vec2::new(50.0, 25.0))));
    commands.spawn(Shape::new(OrientedRectRectedRounded::new(Vec2::X*-650.0 + Vec2::Y*350.0, Vec2::new(100.0, 50.0), Vec2::new(2.0, 1.0).normalize(), Vec2::new(50.0, 25.0), 25.0)));

    commands.spawn(Shape::new(Slope::new(Vec2::X*200.0 + Vec2::Y*200.0 + Vec2::new( 20.0,  20.0), Vec2::new( 2.0, -1.0).normalize(), 200.0)));
    commands.spawn(Shape::new(Slope::new(Vec2::X*200.0 + Vec2::Y*200.0 + Vec2::new(-20.0,  20.0), Vec2::new(-2.0, -1.0).normalize(), 200.0)));
    commands.spawn(Shape::new(Slope::new(Vec2::X*200.0 + Vec2::Y*200.0 + Vec2::new(-20.0, -20.0), Vec2::new(-2.0,  1.0).normalize(), 200.0)));
    commands.spawn(Shape::new(Slope::new(Vec2::X*200.0 + Vec2::Y*200.0 + Vec2::new( 20.0, -20.0), Vec2::new( 2.0,  1.0).normalize(), 200.0)));

    commands.spawn(Shape::new(SlopeRected::new(Vec2::X*200.0 - Vec2::Y*250.0 + Vec2::new( 110.0,  45.0), Vec2::new( 2.0, -1.0).normalize(), 200.0, Vec2::new(50.0, 25.0))));
    commands.spawn(Shape::new(SlopeRected::new(Vec2::X*200.0 - Vec2::Y*250.0 + Vec2::new(-110.0,  45.0), Vec2::new(-2.0, -1.0).normalize(), 200.0, Vec2::new(50.0, 25.0))));
    commands.spawn(Shape::new(SlopeRected::new(Vec2::X*200.0 - Vec2::Y*250.0 + Vec2::new(-110.0, -45.0), Vec2::new(-2.0,  1.0).normalize(), 200.0, Vec2::new(50.0, 25.0))));
    commands.spawn(Shape::new(SlopeRected::new(Vec2::X*200.0 - Vec2::Y*250.0 + Vec2::new( 110.0, -45.0), Vec2::new( 2.0,  1.0).normalize(), 200.0, Vec2::new(50.0, 25.0))));

    commands.spawn(Shape::new(SlopeRounded::new(Vec2::X*500.0 + Vec2::new( 30.0,  30.0), Vec2::new( 2.0, -1.0).normalize(), 200.0, 25.0)));
    commands.spawn(Shape::new(SlopeRounded::new(Vec2::X*500.0 + Vec2::new(-30.0,  30.0), Vec2::new(-2.0, -1.0).normalize(), 200.0, 25.0)));
    commands.spawn(Shape::new(SlopeRounded::new(Vec2::X*500.0 + Vec2::new(-30.0, -30.0), Vec2::new(-2.0,  1.0).normalize(), 200.0, 25.0)));
    commands.spawn(Shape::new(SlopeRounded::new(Vec2::X*500.0 + Vec2::new( 30.0, -30.0), Vec2::new( 2.0,  1.0).normalize(), 200.0, 25.0)));

    commands.spawn(Shape::new(SlopeRectedRounded::new(Vec2::X*-500.0 - Vec2::Y*250.0 + Vec2::new( 115.0,  50.0), Vec2::new( 2.0, -1.0).normalize(), 200.0, Vec2::new(50.0, 25.0), 25.0)));
    commands.spawn(Shape::new(SlopeRectedRounded::new(Vec2::X*-500.0 - Vec2::Y*250.0 + Vec2::new(-115.0,  50.0), Vec2::new(-2.0, -1.0).normalize(), 200.0, Vec2::new(50.0, 25.0), 25.0)));
    commands.spawn(Shape::new(SlopeRectedRounded::new(Vec2::X*-500.0 - Vec2::Y*250.0 + Vec2::new(-115.0, -50.0), Vec2::new(-2.0,  1.0).normalize(), 200.0, Vec2::new(50.0, 25.0), 25.0)));
    commands.spawn(Shape::new(SlopeRectedRounded::new(Vec2::X*-500.0 - Vec2::Y*250.0 + Vec2::new( 115.0, -50.0), Vec2::new( 2.0,  1.0).normalize(), 200.0, Vec2::new(50.0, 25.0), 25.0)));

    
}

fn render(mut gizmos: Gizmos, q_shapes: Query<&Shape>) {

    for Shape(shape) in q_shapes.iter() {
        match shape.get_debug_render_data() {
            collision_3::RenderData::Circle { origin, radius } => { gizmos.circle_2d(origin, radius, Color::RED); },
            collision_3::RenderData::Polygon { points, normals } => {
                gizmos.linestrip_2d((0..points.len()).chain(std::iter::once(0)).map(|i| points[i]), Color::RED);
                for i in 0..points.len() {
                    let from = points[i];
                    let to   = points[(i+1) % points.len()];
                    let mid  = (from + to)*0.5;
                    let norm = normals[i];
                    gizmos.line_2d(mid, mid + norm*20.0, Color::BLUE);
                }
            },
            collision_3::RenderData::RoundedPoly { points, normals, radius } => {
                gizmos.linestrip_2d((0..points.len()).chain(std::iter::once(0)).map(|i| points[i]), Color::RED);
                for i in 0..points.len() {
                    let from = points[i];
                    gizmos.circle_2d(from, radius, Color::GREEN);

                    
                    let to   = points[(i+1) % points.len()];
                    let mid  = (from + to)*0.5;
                    let norm = normals[i];
                    gizmos.line_2d(mid, mid + norm*20.0, Color::BLUE);
                }
                // TODO offset polygon
            },
        };


        
    }

}
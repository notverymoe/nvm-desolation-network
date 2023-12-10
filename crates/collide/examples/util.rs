// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;
use nvm_collide::prelude::*;

pub trait ShapeMarkerTrait: DebugShape + RaycastTarget + Send + Sync {}
impl<T: DebugShape + RaycastTarget + Send + Sync + 'static> ShapeMarkerTrait for T {}

pub fn render_shape(gizmos: &mut Gizmos, shape: &dyn ShapeMarkerTrait, colour: Color) {
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

#[allow(dead_code)]
fn main() {}
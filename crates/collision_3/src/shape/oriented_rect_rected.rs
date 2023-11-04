// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::{Vec2, Vec2Swizzles};

use crate::{RaycastTarget, RayCaster, RayIntersection, CollisionDebugShape, RenderData};
use tinyvec::{ArrayVec, array_vec};

pub struct OrientedRectRected {
    pub origin:     Vec2,
    pub size:       Vec2,
    pub direction:  Vec2,
    pub outer_size: Vec2,
}

impl OrientedRectRected {
    pub fn new(origin: Vec2, size: Vec2, direction: Vec2, outer_size: Vec2) -> Self {
        Self{origin, size, direction, outer_size}
    }
}

impl RaycastTarget for OrientedRectRected {
    fn raycast(&self, _ray: RayCaster) -> Option<[RayIntersection; 2]> {
        todo!();
    }
}

impl CollisionDebugShape for OrientedRectRected {
    fn get_debug_render_data(&self) -> RenderData {
        let points = get_polygon_data_for_oriented_rect_rected(self.origin, self.size, self.direction, self.outer_size);
        RenderData::Polygon {  
            normals: (0..points.len()).map(|i| {
                let start = points[i];
                let end   = points[(i+1)%points.len()];
                (start - end).normalize().perp()
            }).collect::<Vec<_>>().into_boxed_slice(),
            points: points.to_vec().into_boxed_slice(),
        }
    }
}

pub fn get_polygon_data_for_oriented_rect_rected(
    origin:     Vec2,
    size:       Vec2,
    direction:  Vec2,
    outer_size: Vec2,
) -> ArrayVec<[Vec2; 12]> {
    if direction.y == 0.0 {
        let combined = size + outer_size;
        array_vec![
            [Vec2; 12] =>
            origin + Vec2::new( combined.x,  combined.y).rotate(direction),
            origin + Vec2::new(-combined.x,  combined.y).rotate(direction),
            origin + Vec2::new(-combined.x, -combined.y).rotate(direction),
            origin + Vec2::new( combined.x, -combined.y).rotate(direction)
        ]
    } else if direction.x == 0.0 {
        let combined = size.yx() + outer_size;
        array_vec![
            [Vec2; 12] =>
            origin + Vec2::new( combined.x,  combined.y).rotate(direction),
            origin + Vec2::new(-combined.x,  combined.y).rotate(direction),
            origin + Vec2::new(-combined.x, -combined.y).rotate(direction),
            origin + Vec2::new( combined.x, -combined.y).rotate(direction)
        ]
    } else {
        quick_and_dirty(
            &[
                origin + Vec2::new( size.x,  size.y).rotate(direction),
                origin + Vec2::new(-size.x,  size.y).rotate(direction),
                origin + Vec2::new(-size.x, -size.y).rotate(direction),
                origin + Vec2::new( size.x, -size.y).rotate(direction),
            ], 
            &[
                direction.perp(),
                -direction,
                -direction.perp(),
                direction
            ],
            outer_size
        )
    }
}

fn quick_and_dirty(points: &[Vec2; 4], norms: &[Vec2; 4], size: Vec2) -> ArrayVec<[Vec2; 12]> {
    

    let rect_points = [
        Vec2::new( size.x,  size.y),
        Vec2::new(-size.x,  size.y),
        Vec2::new(-size.x, -size.y),
        Vec2::new( size.x, -size.y),
    ];

    let mut result = ArrayVec::<[Vec2; 12]>::default();
    for i in 0..points.len() {
        let p  =  points[i];
        let n1 =   norms[i];
        let n0 = -n1.perp();
        let offset_0 = rect_points.iter().map(|&v| (v, n0.dot(v))).max_by(|(_, x), (_, y)| x.total_cmp(y)).unwrap().0;
        let offset_1 = rect_points.iter().map(|&v| (v, n1.dot(v))).max_by(|(_, x), (_, y)| x.total_cmp(y)).unwrap().0;

        result.extend([
            p + offset_0,
            p + offset_1
        ]);
    }


    result
}

// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, CollisionDebugShape, RenderData};

pub struct Slope {
    pub origin:    Vec2,
    pub direction: Vec2,
    pub length:    f32,
}

impl Slope {
    pub fn new(origin: Vec2, direction: Vec2, length: f32) -> Self {
        Self{origin, direction, length}
    }
}

impl RaycastTarget for Slope {
    fn raycast(&self, ray: RayCaster) -> Option<[RayIntersection; 2]> {
        let (points, normals, lengths) = get_polygon_data_for_slope(self.direction, self.length);
        ray.test_polygon(self.origin, &points, &normals, &lengths)
    }
}

impl CollisionDebugShape for Slope {
    fn get_debug_render_data(&self) -> RenderData {
        let (points, normals, _lengths) = get_polygon_data_for_slope(self.direction, self.length);
        RenderData::Polygon { 
            points:  Box::new(points.map(|v| self.origin + v)), 
            normals: Box::new(normals),
        }
    }
}

pub(crate) fn get_polygon_data_for_slope(direction: Vec2, length: f32) -> ([Vec2; 3], [Vec2; 3], [f32; 3]) {
    let size   = Vec2::new(direction.x, -direction.y) * length;
    let normal = direction.perp();

    let right = Vec2::X * size.x.signum();
    let up    = Vec2::Y * size.y.signum();

    // Ordering for CCW polygon 
    if (size.x >= 0.0) == (size.y >= 0.0) {
        (
            [Vec2::ZERO, Vec2::new(size.x, 0.0), Vec2::new(0.0, size.y)],
            [       -up,                 normal,                 -right],
            [    size.x,                 length,                 size.y],
        )
    } else {
        (
            [Vec2::ZERO, Vec2::new(0.0, size.y),  Vec2::new(size.x, 0.0)],
            [    -right,                -normal,                     -up],
            [    size.y,                 length,                  size.x],
        )
    }
}
// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, RenderData, CollisionDebugShape, get_polygon_data_for_rect_slope};

pub struct SlopeRectedRounded {
    pub origin:    Vec2,
    pub direction: Vec2,
    pub length:    f32,
    pub size:      Vec2,
    pub radius:    f32,
}

impl SlopeRectedRounded {
    pub fn new(origin: Vec2, direction: Vec2, length: f32, size: Vec2, radius: f32) -> Self {
        Self{origin, direction, length, size, radius}
    }
}

impl RaycastTarget for SlopeRectedRounded {
    fn raycast(&self, ray: RayCaster) -> Option<[RayIntersection; 2]> {
        let (points, normals, lengths) = get_polygon_data_for_rect_slope(self.direction, self.length, self.size);
        ray.test_polygon_rounded(self.origin, &points, &normals, &lengths, self.radius)
    }
}

impl CollisionDebugShape for SlopeRectedRounded {
    fn get_debug_render_data(&self) -> RenderData {
        let (points, normals, _lengths) = get_polygon_data_for_rect_slope(self.direction, self.length, self.size);
        RenderData::RoundedPoly { 
            radius:  self.radius,
            points:  Box::new(points.map(|v| self.origin + v)), 
            normals: Box::new(normals),
        }
    }
}

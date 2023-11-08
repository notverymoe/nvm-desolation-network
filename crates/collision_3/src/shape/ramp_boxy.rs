// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, RenderData, CollisionDebugShape, get_polygon_data_for_ramp_boxy};

pub struct RampBoxy {
    pub origin:    Vec2,
    pub direction: Vec2,
    pub length:    f32,
    pub size:      Vec2,
}

impl RampBoxy {
    pub fn new(origin: Vec2, direction: Vec2, length: f32, size: Vec2) -> Self {
        Self{origin, direction, length, size}
    }
}

impl RaycastTarget for RampBoxy {
    fn raycast(&self, ray: RayCaster) -> Option<[RayIntersection; 2]> {
        let (points, normals, lengths) = get_polygon_data_for_ramp_boxy(self.direction, self.length, self.size);
        ray.test_polygon(self.origin, &points, &normals, &lengths)
    }
}

impl CollisionDebugShape for RampBoxy {
    fn get_debug_render_data(&self) -> RenderData {
        let (points, normals, _lengths) = get_polygon_data_for_ramp_boxy(self.direction, self.length, self.size);
        RenderData::Polygon { 
            points:  Box::new(points.map(|v| self.origin + v)), 
            normals: Box::new(normals),
        }
    }
}
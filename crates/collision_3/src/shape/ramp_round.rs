// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, CollisionDebugShape, RenderData, get_polygon_data_for_ramp};

pub struct RampRound {
    pub origin:    Vec2,
    pub direction: Vec2,
    pub length:    f32,
    pub radius:    f32,
}

impl RampRound {
    pub fn new(origin: Vec2, direction: Vec2, length: f32, radius: f32) -> Self {
        Self{origin, direction, length, radius}
    }
}

impl RaycastTarget for RampRound {
    fn raycast(&self, ray: RayCaster) -> Option<[RayIntersection; 2]> {
        let (points, normals, lengths) = get_polygon_data_for_ramp(self.direction, self.length);
        ray.test_polygon_rounded(self.origin, &points, &normals, &lengths, self.radius)
    }
}

impl CollisionDebugShape for RampRound {
    fn get_debug_render_data(&self) -> RenderData {
        let (points, normals, _lengths) = get_polygon_data_for_ramp(self.direction, self.length);
        RenderData::RoundedPoly { 
            radius:  self.radius,
            points:  Box::new(points.map(|v| self.origin + v)), 
            normals: Box::new(normals),
        }
    }
}

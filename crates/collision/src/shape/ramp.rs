// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, DebugShape, DebugShapeData, get_polygon_data_for_ramp, HasBoundingBox, BoxAligned};

pub struct Ramp {
    pub origin:    Vec2,
    pub direction: Vec2,
    pub length:    f32,
}

impl Ramp {
    pub fn new(origin: Vec2, direction: Vec2, length: f32) -> Self {
        Self{origin, direction, length}
    }

    pub fn get_normal(&self) -> Vec2 {
        let size = Vec2::new(self.direction.x, -self.direction.y) * self.length;
        if (size.x >= 0.0) == (size.y >= 0.0) {
            self.direction.perp()
        } else {
            -self.direction.perp()
        }
    }
}

impl HasBoundingBox for Ramp {
    fn bounding_box(&self) -> BoxAligned {
        BoxAligned::new(self.origin, self.direction*self.length*0.5)
    }
}

impl RaycastTarget for Ramp {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        let (points, normals, lengths) = get_polygon_data_for_ramp(self.direction, self.length);
        ray.test_polygon(self.origin, &points, &normals, &lengths)
    }
}

impl DebugShape for Ramp {
    fn get_debug_shape_data(&self) -> DebugShapeData {
        let (points, normals, _lengths) = get_polygon_data_for_ramp(self.direction, self.length);
        DebugShapeData::polygon(
            Box::new(points.map(|v| self.origin + v)), 
            Box::new(normals),
        )
    }
}
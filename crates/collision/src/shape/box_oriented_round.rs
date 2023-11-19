// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, DebugShape, DebugShapeData, BoxAligned, HasBoundingBox};

pub struct BoxOrientedRound {
    pub origin:    Vec2,
    pub size:      Vec2,
    pub direction: Vec2,
    pub radius:    f32,
}

impl BoxOrientedRound {
    pub fn new(origin: Vec2, size: Vec2, direction: Vec2, radius: f32) -> Self {
        Self{origin, size, direction, radius}
    }
}

impl HasBoundingBox for BoxOrientedRound {
    fn bounding_box(&self) -> BoxAligned {
        let bound_x = Vec2::new( self.size.x,  self.size.y).rotate(self.direction).abs();
        let bound_y = Vec2::new(-self.size.x,  self.size.y).rotate(self.direction).abs();
        let size = bound_x.max(bound_y);
        BoxAligned::new(self.origin, Vec2::new(size.x + self.radius, size.y + self.radius))
    }
}

impl RaycastTarget for BoxOrientedRound {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        let points = [
            self.origin + Vec2::new( self.size.x,  self.size.y).rotate(self.direction),
            self.origin + Vec2::new(-self.size.x,  self.size.y).rotate(self.direction),
            self.origin + Vec2::new(-self.size.x, -self.size.y).rotate(self.direction),
            self.origin + Vec2::new( self.size.x, -self.size.y).rotate(self.direction),
        ];

        let normals = [
            self.direction.perp(),
            -self.direction,
            -self.direction.perp(),
            self.direction
        ];

        let lengths = [
            2.0*self.size.x,
            2.0*self.size.y,
            2.0*self.size.x,
            2.0*self.size.y
        ];

        ray.test_polygon_rounded_at_origin(&points, &normals, &lengths, self.radius)
    }
}

impl DebugShape for BoxOrientedRound {
    fn get_debug_shape_data(&self) -> DebugShapeData {
        DebugShapeData::polygon_round(
            Box::new([
                self.origin + Vec2::new( self.size.x,  self.size.y).rotate(self.direction),
                self.origin + Vec2::new(-self.size.x,  self.size.y).rotate(self.direction),
                self.origin + Vec2::new(-self.size.x, -self.size.y).rotate(self.direction),
                self.origin + Vec2::new( self.size.x, -self.size.y).rotate(self.direction),
            ]), 
            Box::new([
                 self.direction.perp(),
                -self.direction,
                -self.direction.perp(),
                 self.direction
            ]),
            self.radius,
        )
    }
}

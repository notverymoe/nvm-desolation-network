// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, DebugShape, DebugShapeData};

pub struct BoxOriented {
    pub origin:    Vec2,
    pub size:      Vec2,
    pub direction: Vec2,
}

impl BoxOriented {
    pub fn new(origin: Vec2, size: Vec2, direction: Vec2) -> Self {
        Self{origin, size, direction}
    }
}

impl RaycastTarget for BoxOriented {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        let points = [
            self.origin + Vec2::new( self.size.x,  self.size.y).rotate(self.direction),
            self.origin + Vec2::new(-self.size.x,  self.size.y).rotate(self.direction),
            self.origin + Vec2::new(-self.size.x, -self.size.y).rotate(self.direction),
            self.origin + Vec2::new( self.size.x, -self.size.y).rotate(self.direction),
        ];

        let normals = [
            self.direction.perp(),
            self.direction,
            -self.direction.perp(),
            -self.direction
        ];

        let lengths = [
            self.size.x,
            self.size.y,
            self.size.x,
            self.size.x
        ];

        ray.test_polygon_at_origin(&points, &normals, &lengths)
    }
}

impl DebugShape for BoxOriented {
    fn get_debug_shape_data(&self) -> DebugShapeData {
        DebugShapeData::polygon( 
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
        )
    }
}

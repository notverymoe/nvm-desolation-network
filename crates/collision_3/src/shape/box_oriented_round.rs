// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, DebugShape, DebugShapeData};

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

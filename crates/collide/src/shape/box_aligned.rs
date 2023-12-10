// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::prelude::{RaycastTarget, RayCaster, RayIntersection, DebugShape, DebugShapeData, HasBoundingBox};

#[derive(Debug, Clone, Copy)]
pub struct BoxAligned {
    pub origin: Vec2,
    pub size:   Vec2,
}

impl BoxAligned {
    pub fn new(origin: Vec2, size: Vec2) -> Self {
        Self{origin, size}
    }

    pub fn bounds(&self) -> [Vec2; 2] {
        [
            self.origin - self.size, 
            self.origin + self.size,
        ]
    }
}

impl HasBoundingBox for BoxAligned {
    fn bounding_box(&self) -> BoxAligned {
        *self
    }
}

impl RaycastTarget for BoxAligned {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        ray.test_rect(self.origin, self.size)
    }
}

impl DebugShape for BoxAligned {
    fn get_debug_shape_data(&self) -> DebugShapeData {
        DebugShapeData::polygon( 
            Box::new([
                self.origin + Vec2::new( self.size.x,  self.size.y),
                self.origin + Vec2::new(-self.size.x,  self.size.y),
                self.origin + Vec2::new(-self.size.x, -self.size.y),
                self.origin + Vec2::new( self.size.x, -self.size.y),
            ]), 
            Box::new([
                 Vec2::Y,
                -Vec2::X,
                -Vec2::Y,
                 Vec2::X
            ]),
        )
    }
}

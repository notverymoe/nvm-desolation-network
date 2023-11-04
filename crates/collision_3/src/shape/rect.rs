// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, CollisionDebugShape, RenderData};

pub struct Rect {
    pub origin: Vec2,
    pub size:   Vec2,
}

impl Rect {
    pub fn new(origin: Vec2, size: Vec2) -> Self {
        Self{origin, size}
    }
}

impl RaycastTarget for Rect {
    fn raycast(&self, ray: RayCaster) -> Option<[RayIntersection; 2]> {
        ray.test_rect(self.origin, self.size)
    }
}

impl CollisionDebugShape for Rect {
    fn get_debug_render_data(&self) -> RenderData {
        RenderData::Polygon { 
            points: Box::new([
                self.origin + Vec2::new( self.size.x,  self.size.y),
                self.origin + Vec2::new(-self.size.x,  self.size.y),
                self.origin + Vec2::new(-self.size.x, -self.size.y),
                self.origin + Vec2::new( self.size.x, -self.size.y),
            ]), 
            normals: Box::new([
                 Vec2::Y,
                -Vec2::X,
                -Vec2::Y,
                 Vec2::X
            ]),
        }
    }
}

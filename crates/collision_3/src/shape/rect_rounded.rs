// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, RenderData, CollisionDebugShape};

pub struct RectRounded {
    pub origin: Vec2,
    pub size:   Vec2,
    pub radius: f32,
}

impl RectRounded {
    pub fn new(origin: Vec2, size: Vec2, radius: f32) -> Self {
        Self{origin, size, radius}
    }
}

impl RaycastTarget for RectRounded {
    fn raycast(&self, ray: RayCaster) -> Option<[RayIntersection; 2]> {
        ray.test_rect_rounded(self.origin, self.size, self.radius)
    }
}

impl CollisionDebugShape for RectRounded {
    fn get_debug_render_data(&self) -> RenderData {
        RenderData::RoundedPoly { 
            radius: self.radius,
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


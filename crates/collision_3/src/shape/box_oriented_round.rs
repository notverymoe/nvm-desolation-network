// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, CollisionDebugShape, RenderData};

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
    fn raycast(&self, _ray: RayCaster) -> Option<[RayIntersection; 2]> {
        todo!();
    }
}

impl CollisionDebugShape for BoxOrientedRound {
    fn get_debug_render_data(&self) -> RenderData {
        RenderData::RoundedPoly { 
            radius: self.radius,
            points: Box::new([
                self.origin + Vec2::new( self.size.x,  self.size.y).rotate(self.direction),
                self.origin + Vec2::new(-self.size.x,  self.size.y).rotate(self.direction),
                self.origin + Vec2::new(-self.size.x, -self.size.y).rotate(self.direction),
                self.origin + Vec2::new( self.size.x, -self.size.y).rotate(self.direction),
            ]), 
            normals: Box::new([
                 self.direction.perp(),
                -self.direction,
                -self.direction.perp(),
                 self.direction
            ]),
        }
    }
}

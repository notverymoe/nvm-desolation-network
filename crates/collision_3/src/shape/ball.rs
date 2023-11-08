// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, CollisionDebugShape, RenderData};

pub struct Ball {
    pub origin: Vec2,
    pub radius: f32,
}

impl Ball {

    pub fn new(origin: Vec2, radius: f32) -> Self {
        Self{origin, radius}
    } 

}

impl RaycastTarget for Ball {
    fn raycast(&self, ray: RayCaster) -> Option<[RayIntersection; 2]> {
        ray.test_circle(self.origin, self.radius)
    }
}

impl CollisionDebugShape for Ball {
    fn get_debug_render_data(&self) -> RenderData {
        RenderData::Circle{
            origin: self.origin,
            radius: self.radius
        }
    }
}
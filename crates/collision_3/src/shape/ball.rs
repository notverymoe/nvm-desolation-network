// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, DebugShape, DebugShapeData};

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

impl DebugShape for Ball {
    fn get_debug_shape_data(&self) -> DebugShapeData {
        DebugShapeData::circle(self.origin, self.radius)
    }
}
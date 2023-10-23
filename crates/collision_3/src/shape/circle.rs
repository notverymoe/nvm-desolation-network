// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection};

pub struct Circle {
    pub origin: Vec2,
    pub radius: f32,
}

impl RaycastTarget for Circle {
    fn raycast(&self, ray: RayCaster) -> Option<[RayIntersection; 2]> {
        ray.test_circle(self.origin, self.radius)
    }
}
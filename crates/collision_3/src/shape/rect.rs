// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection};

pub struct Rect {
    pub origin: Vec2,
    pub size:   Vec2,
}

impl RaycastTarget for Rect {
    fn raycast(&self, ray: RayCaster) -> Option<[RayIntersection; 2]> {
        ray.test_rect(self.origin, self.size)
    }
}

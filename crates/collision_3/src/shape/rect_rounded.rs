// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection};

pub struct RectRounded {
    pub origin: Vec2,
    pub size:   Vec2,
    pub radius: f32,
}

impl RaycastTarget for RectRounded {
    fn raycast(&self, ray: RayCaster) -> Option<[RayIntersection; 2]> {
        ray.test_rect_rounded(self.origin, self.size, self.radius)
    }
}


// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, get_polygon_data_for_slope};

pub struct SlopeRounded {
    pub origin:    Vec2,
    pub direction: Vec2,
    pub length:    f32,
    pub radius:    f32,
}

impl RaycastTarget for SlopeRounded {
    fn raycast(&self, ray: RayCaster) -> Option<[RayIntersection; 2]> {
        let (points, normals, lengths) = get_polygon_data_for_slope(self.direction, self.length);
        ray.test_polygon_rounded(self.origin, &points, &normals, &lengths, self.radius)
    }
}

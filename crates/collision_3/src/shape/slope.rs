// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection};

pub struct Slope {
    pub origin:    Vec2,
    pub direction: Vec2,
    pub length:    f32,
}

impl RaycastTarget for Slope {
    fn raycast(&self, ray: RayCaster) -> Option<[RayIntersection; 2]> {
        let (points, normals, lengths) = get_polygon_data_for_slope(self.direction, self.length);
        ray.test_polygon(self.origin, &points, &normals, &lengths)
    }
}

pub(crate) fn get_polygon_data_for_slope(direction: Vec2, length: f32) -> ([Vec2; 3], [Vec2; 3], [f32; 3]) {
    let size = direction.perp() * length;
    // Ordering for CCW polygon 
    if (size.x >= 0.0) == (size.y >= 0.0) {
        (
            [Vec2::ZERO,  Vec2::new(size.x, 0.0), Vec2::new(0.0, size.y)],
            [   Vec2::Y,        direction.perp(),                Vec2::X],
            [    size.x,                  length,                 size.y],
        )
    } else {
        (
            [Vec2::ZERO,  Vec2::new(0.0, size.y),  Vec2::new(size.x, 0.0)],
            [   Vec2::X,        direction.perp(),                 Vec2::Y],
            [    size.y,                  length,                  size.x],
        )
    }
}
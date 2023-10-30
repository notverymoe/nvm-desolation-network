// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, get_polygon_data_for_slope};

pub struct SlopeRected {
    pub origin:    Vec2,
    pub direction: Vec2,
    pub length:    f32,
    pub size:      Vec2,
}

impl RaycastTarget for SlopeRected {
    fn raycast(&self, ray: RayCaster) -> Option<[RayIntersection; 2]> {
        let (points, normals, lengths) = get_polygon_data_for_slope(self.direction, self.length);
        let slope_origin = self.origin + self.direction.perp().signum()*self.size;
        let slope_h_size = self.direction * self.length*0.5;
        // TODO OPT we should just be able to just construct a rected slope polygon
        RayIntersection::find_polygon_entry_exit_pairs([
            ray.test_polygon(slope_origin, &points, &normals, &lengths),
            ray.test_rect(Vec2::new(slope_h_size.x, 0.0), Vec2::new(slope_h_size.x + self.size.x, self.size.y)),
            ray.test_rect(Vec2::new(0.0, slope_h_size.y), Vec2::new(self.size.x, slope_h_size.y + self.size.y)),
        ].into_iter().flatten())
    }
}
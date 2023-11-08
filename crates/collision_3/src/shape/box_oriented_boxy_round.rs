// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, CollisionDebugShape, RenderData, PolygonSmallRound};
use super::get_polygon_data_for_oriented_rect_rected;

pub struct BoxOrientedBoxyRound(PolygonSmallRound);

impl BoxOrientedBoxyRound {
    pub fn new(origin: Vec2, size: Vec2, direction: Vec2, outer_size: Vec2, radius: f32) -> Self {
        Self(PolygonSmallRound::new_from_points(get_polygon_data_for_oriented_rect_rected(origin, size, direction, outer_size), radius))
    }
}

impl RaycastTarget for BoxOrientedBoxyRound {
    fn raycast(&self, ray: RayCaster) -> Option<[RayIntersection; 2]> {
        self.0.raycast(ray)
    }
}

impl CollisionDebugShape for BoxOrientedBoxyRound {
    fn get_debug_render_data(&self) -> RenderData {
        self.0.get_debug_render_data()
    }
}

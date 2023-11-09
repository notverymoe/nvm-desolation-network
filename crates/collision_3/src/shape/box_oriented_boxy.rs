// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, DebugShape, DebugShapeData, get_polygon_data_for_oriented_rect_rected, PolygonSmall};

pub struct BoxOrientedBoxy(PolygonSmall);

impl BoxOrientedBoxy {
    pub fn new(origin: Vec2, size: Vec2, direction: Vec2, outer_size: Vec2) -> Self {
        Self(PolygonSmall::new_from_points(get_polygon_data_for_oriented_rect_rected(origin, size, direction, outer_size)))
    }
}

impl RaycastTarget for BoxOrientedBoxy {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        self.0.raycast(ray)
    }
}

impl DebugShape for BoxOrientedBoxy {
    fn get_debug_shape_data(&self) -> DebugShapeData {
        self.0.get_debug_shape_data()
    }
}

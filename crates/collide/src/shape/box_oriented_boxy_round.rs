// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::prelude::{RaycastTarget, RayCaster, RayIntersection, DebugShape, DebugShapeData, PolygonSmallRound, ShapeCommon, BoxAligned};
use super::get_polygon_data_for_oriented_rect_rected;

pub struct BoxOrientedBoxyRound(PolygonSmallRound);

impl BoxOrientedBoxyRound {
    pub fn new(origin: Vec2, size: Vec2, direction: Vec2, outer_size: Vec2, radius: f32) -> Self {
        Self(PolygonSmallRound::new_from_points(get_polygon_data_for_oriented_rect_rected(origin, size, direction, outer_size), radius))
    }
}

impl ShapeCommon for BoxOrientedBoxyRound {
    fn bounding_box(&self) -> BoxAligned {
        self.0.bounding_box()
    }

    fn origin(&self) -> Vec2 {
        self.0.origin()
    }

    fn set_origin(&mut self, origin: Vec2) {
        self.0.set_origin(origin);
    }
}

impl RaycastTarget for BoxOrientedBoxyRound {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        self.0.raycast(ray)
    }
}

impl DebugShape for BoxOrientedBoxyRound {
    fn get_debug_shape_data(&self) -> DebugShapeData {
        self.0.get_debug_shape_data()
    }
}

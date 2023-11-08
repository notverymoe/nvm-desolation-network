// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, RenderData, CollisionDebugShape, get_polygon_data_for_ramp_boxy, PolygonSmallRound, PolygonSmall};

pub struct RampBoxyRound(PolygonSmallRound);

impl RampBoxyRound {
    pub fn new(origin: Vec2, direction: Vec2, length: f32, size: Vec2, radius: f32) -> Self {
        let (points, normals, lengths) = get_polygon_data_for_ramp_boxy(direction, length, size);
        Self(PolygonSmallRound::new(PolygonSmall::new(points.map(|v| origin + v), normals, lengths), radius))
    }
}

impl RaycastTarget for RampBoxyRound {
    fn raycast(&self, ray: RayCaster) -> Option<[RayIntersection; 2]> {
        self.0.raycast(ray)
    }
}

impl CollisionDebugShape for RampBoxyRound {
    fn get_debug_render_data(&self) -> RenderData {
        self.0.get_debug_render_data()
    }
}

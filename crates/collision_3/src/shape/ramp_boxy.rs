// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, DebugShape, DebugShapeData, get_polygon_data_for_ramp_boxy, PolygonSmall};

pub struct RampBoxy(PolygonSmall);

impl RampBoxy {
    pub fn new(origin: Vec2, direction: Vec2, length: f32, size: Vec2) -> Self {
        let (points, normals, lengths) = get_polygon_data_for_ramp_boxy(direction, length, size);
        Self(PolygonSmall::new(points.map(|v| origin + v), normals, lengths))
    }
}

impl RaycastTarget for RampBoxy {
    fn raycast(&self, ray: RayCaster) -> Option<[RayIntersection; 2]> {
        self.0.raycast(ray)
    }
}

impl DebugShape for RampBoxy {
    fn get_debug_shape_data(&self) -> DebugShapeData {
        self.0.get_debug_shape_data()
    }
}

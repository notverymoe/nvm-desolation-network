// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, CollisionDebugShape, RayCaster, RayIntersection, RenderData, PolygonSmall};

pub struct PolygonSmallRound {
    inner: PolygonSmall,
    radius: f32,
}

impl PolygonSmallRound {
    pub fn new(polygon: PolygonSmall, radius: f32) -> Self {
        Self{inner: polygon, radius}
    }

    pub fn new_from_points(points: impl IntoIterator<Item = Vec2>, radius: f32) -> Self {
        Self::new(PolygonSmall::new_from_points(points), radius)
    }
}

impl RaycastTarget for PolygonSmallRound {
    fn raycast(&self, ray: RayCaster) -> Option<[RayIntersection; 2]> {
        ray.test_polygon_rounded_at_origin(&self.inner.points, &self.inner.normals, &self.inner.lengths, self.radius)
    }
}

impl CollisionDebugShape for PolygonSmallRound {
    fn get_debug_render_data(&self) -> RenderData {
        RenderData::RoundedPoly {  
            points:  self.inner.points.to_vec().into_boxed_slice(),
            normals: self.inner.normals.to_vec().into_boxed_slice(),
            radius: self.radius,
        }
    }
}

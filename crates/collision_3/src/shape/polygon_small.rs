// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use tinyvec::ArrayVec;

use crate::{RaycastTarget, CollisionDebugShape, RayCaster, RayIntersection, RenderData};

pub struct PolygonSmall {
    pub(super) points:  ArrayVec<[Vec2; 12]>,
    pub(super) normals: ArrayVec<[Vec2; 12]>,
    pub(super) lengths: ArrayVec<[ f32; 12]>,
}

impl PolygonSmall {
    pub fn new_from_points(points: impl IntoIterator<Item = Vec2>) -> Self {
        let points:      ArrayVec<[Vec2; 12]> = ArrayVec::from_iter(points);
        let mut normals: ArrayVec<[Vec2; 12]> = Default::default();
        let mut lengths: ArrayVec<[ f32; 12]> = Default::default();

        for i in 0..points.len() {
            let start = points[i];
            let end   = points[(i+1)%points.len()];
            let offset = start-end;
            let length = offset.length();
            let normal = offset/length;
            lengths.push(length);
            normals.push(normal);
        }

        Self{points, normals, lengths}
    }
}

impl RaycastTarget for PolygonSmall {
    fn raycast(&self, ray: RayCaster) -> Option<[RayIntersection; 2]> {
        ray.test_polygon_at_origin(&self.points, &self.normals, &self.lengths)
    }
}

impl CollisionDebugShape for PolygonSmall {
    fn get_debug_render_data(&self) -> RenderData {
        RenderData::Polygon {  
            points:  self.points.to_vec().into_boxed_slice(),
            normals: self.normals.to_vec().into_boxed_slice(),
        }
    }
}

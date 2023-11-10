// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use tinyvec::ArrayVec;

use crate::{RaycastTarget, DebugShape, RayCaster, RayIntersection, DebugShapeData};

pub const POLYGON_SMALL_CAPACITY: usize = 8;

pub struct PolygonSmall {
    pub(super) points:  ArrayVec<[Vec2; POLYGON_SMALL_CAPACITY]>,
    pub(super) normals: ArrayVec<[Vec2; POLYGON_SMALL_CAPACITY]>,
    pub(super) lengths: ArrayVec<[ f32; POLYGON_SMALL_CAPACITY]>,
}

impl PolygonSmall {
    pub fn new(points: impl IntoIterator<Item = Vec2>, normals: impl IntoIterator<Item = Vec2>, lengths: impl IntoIterator<Item = f32>) -> Self {
        Self{
            points:  ArrayVec::from_iter(points ),
            normals: ArrayVec::from_iter(normals),
            lengths: ArrayVec::from_iter(lengths),
        }
    }

    pub fn new_from_points(points: impl IntoIterator<Item = Vec2>) -> Self {
        let points:      ArrayVec<[Vec2; POLYGON_SMALL_CAPACITY]> = ArrayVec::from_iter(points);
        let mut normals: ArrayVec<[Vec2; POLYGON_SMALL_CAPACITY]> = Default::default();
        let mut lengths: ArrayVec<[ f32; POLYGON_SMALL_CAPACITY]> = Default::default();

        for i in 0..points.len() {
            let start = points[i];
            let end   = points[(i+1)%points.len()];
            let offset = end-start;
            let length = offset.length();
            let normal = -(offset/length).perp();
            lengths.push(length);
            normals.push(normal);
        }

        Self{points, normals, lengths}
    }
}

impl RaycastTarget for PolygonSmall {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        ray.test_polygon_at_origin(&self.points, &self.normals, &self.lengths)
    }
}

impl DebugShape for PolygonSmall {
    fn get_debug_shape_data(&self) -> DebugShapeData {
        DebugShapeData::polygon(  
            self.points.to_vec().into_boxed_slice(),
            self.normals.to_vec().into_boxed_slice(),
        )
    }
}
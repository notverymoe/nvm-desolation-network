// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, CollisionDebugShape, RenderData, get_polygon_data_for_oriented_rect_rected};

pub struct BoxOrientedBoxy {
    pub origin:     Vec2,
    pub size:       Vec2,
    pub direction:  Vec2,
    pub outer_size: Vec2,
}

impl BoxOrientedBoxy {
    pub fn new(origin: Vec2, size: Vec2, direction: Vec2, outer_size: Vec2) -> Self {
        Self{origin, size, direction, outer_size}
    }
}

impl RaycastTarget for BoxOrientedBoxy {
    fn raycast(&self, _ray: RayCaster) -> Option<[RayIntersection; 2]> {
        todo!();
    }
}

impl CollisionDebugShape for BoxOrientedBoxy {
    fn get_debug_render_data(&self) -> RenderData {
        let points = get_polygon_data_for_oriented_rect_rected(self.origin, self.size, self.direction, self.outer_size);
        RenderData::Polygon {  
            normals: (0..points.len()).map(|i| {
                let start = points[i];
                let end   = points[(i+1)%points.len()];
                (start - end).normalize().perp()
            }).collect::<Vec<_>>().into_boxed_slice(),
            points: points.to_vec().into_boxed_slice(),
        }
    }
}

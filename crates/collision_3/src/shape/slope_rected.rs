// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, RayIntersection, RenderData, CollisionDebugShape};

pub struct SlopeRected {
    pub origin:    Vec2,
    pub direction: Vec2,
    pub length:    f32,
    pub size:      Vec2,
}

impl SlopeRected {
    pub fn new(origin: Vec2, direction: Vec2, length: f32, size: Vec2) -> Self {
        Self{origin, direction, length, size}
    }
}

impl RaycastTarget for SlopeRected {
    fn raycast(&self, ray: RayCaster) -> Option<[RayIntersection; 2]> {
        let (points, normals, lengths) = get_polygon_data_for_rect_slope(self.direction, self.length, self.size);
        ray.test_polygon(self.origin, &points, &normals, &lengths)
    }
}

impl CollisionDebugShape for SlopeRected {
    fn get_debug_render_data(&self) -> RenderData {
        let (points, normals, _lengths) = get_polygon_data_for_rect_slope(self.direction, self.length, self.size);
        RenderData::Polygon { 
            points:  Box::new(points.map(|v| self.origin + v)), 
            normals: Box::new(normals),
        }
    }
}

pub fn get_polygon_data_for_rect_slope(direction: Vec2, length: f32, rect_size_abs: Vec2) -> ([Vec2; 5], [Vec2; 5], [f32; 5]) {

    let cross_dir = Vec2::new(direction.x, -direction.y);
    let    normal = direction.perp();
    let  tri_size = cross_dir * length;
    let rect_size = cross_dir * rect_size_abs;
    let aabb_size = rect_size_abs + tri_size.abs();

    let origin = -rect_size*2.0;

    let point_vert_out = Vec2::new(origin.x, tri_size.y);
    let point_vert_in  = Vec2::new(     0.0, tri_size.y);

    let point_horz_out = Vec2::new(tri_size.x, origin.y);
    let point_horz_in  = Vec2::new(tri_size.x,      0.0);

    let right = Vec2::X * tri_size.x.signum();
    let up    = Vec2::Y * tri_size.y.signum();

    if (tri_size.x >= 0.0) == (tri_size.y >= 0.0) {
        (
            [     origin,  point_vert_out, point_vert_in,   point_horz_in, point_horz_out],
            [     -right,              up,        normal,           right,            -up],
            [aabb_size.x, rect_size_abs.y,        length, rect_size_abs.x,    aabb_size.y],
        )
    } else {
        (
            [     origin,  point_horz_out, point_horz_in,   point_vert_in, point_vert_out],
            [        -up,           right,       -normal,              up,         -right],
            [aabb_size.y, rect_size_abs.x,        length, rect_size_abs.y,    aabb_size.x],
        )
    }

}
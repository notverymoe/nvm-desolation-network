// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::Vec2;

use crate::{HasBoundingBox, shape::Rect, Projection, SATShape};

#[derive(Clone, Copy)]
pub struct RectOriented {
    pub origin:  Vec2,
    pub extents: Vec2,
    pub up:  Vec2,
}

impl SATShape for RectOriented {

    const CAN_SMEAR_PROJECTION: bool = true;

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        Projection::from_points_iter(axis, self.points())
    }

    fn get_points(&self, out_points: &mut Vec<Vec2>) {
        out_points.extend_from_slice(&self.points())
    }

    fn get_axes(&self, out_axes: &mut Vec<Vec2>, out_cache: &mut Vec<Projection>) {
        // OPT
        let norms  = [self.up.perp(), self.up];
        let points = self.points();
        out_axes.extend_from_slice(&norms);
        out_cache.extend_from_slice(&[
            Projection::from_points_iter(norms[0], points),
            Projection::from_points_iter(norms[1], points),
        ])
    }

    fn get_axes_derived(&self, _other: &[Vec2], _out_axes: &mut Vec<Vec2>) {
        // no derived axes
    }

    fn with_offset(mut self, offset: Vec2) -> Self {
        self.origin += offset;
        self
    }

}

impl HasBoundingBox for RectOriented {

    fn get_bounding_box(&self) -> Rect {
        // TODO prove this works
        let offset_x = (self.extents.x * self.up       ).abs();
        let offset_y = (self.extents.y * self.up.perp()).abs();
        Rect{
            min: self.origin - offset_x - offset_y,
            max: self.origin + offset_x + offset_y,
        }
    }

}

impl RectOriented {

    pub fn points(&self) -> [Vec2; 4] {
        let offset_x = self.extents.x * self.up;
        let offset_y = self.extents.y * self.up.perp();
        [
            self.origin - offset_x - offset_y, // BL
            self.origin + offset_x - offset_y, // BR
            self.origin + offset_x + offset_y, // TR
            self.origin - offset_x + offset_y, // TL
        ]
    }

}
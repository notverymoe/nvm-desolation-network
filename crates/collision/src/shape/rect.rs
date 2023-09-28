// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::Vec2;

use crate::{HasBoundingBox, shape::RectOriented, Projection, SATShape};

#[derive(Clone, Copy)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl SATShape for Rect {

    const CAN_SMEAR_PROJECTION: bool = true;

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        Projection::from_points_iter(axis, self.points())
    }

    fn get_points(&self, out_points: &mut Vec<Vec2>) {
        out_points.extend_from_slice(&self.points())
    }

    fn get_axes(&self, out_axes: &mut Vec<Vec2>, out_cache: &mut Vec<Projection>) {
        out_axes.extend_from_slice(&[Vec2::X, Vec2::Y]);
        out_cache.extend_from_slice(&[
            Projection([self.min.x, self.max.x]),
            Projection([self.min.y, self.max.y]),
        ])
    }

    fn get_axes_derived(&self, _other: &[Vec2], _out_axes: &mut Vec<Vec2>) {
        // no derived axes
    }

    fn with_offset(mut self, offset: Vec2) -> Self {
        self.min += offset;
        self.max += offset;
        self
    }

}

impl HasBoundingBox for Rect {

    fn get_bounding_box(&self) -> Rect {
        *self
    }

}

impl Rect {

    pub fn points(&self) -> [Vec2; 4] {
        [
            Vec2::new(self.min.x, self.min.y),
            Vec2::new(self.max.x, self.min.y),
            Vec2::new(self.max.x, self.max.y),
            Vec2::new(self.min.x, self.max.y),
        ]
    }

    pub fn expand_point(&mut self, point: Vec2) {
        self.min = self.min.min(point);
        self.max = self.max.max(point);
    }

    pub fn expand_shape(&mut self, other: &impl HasBoundingBox) {
        let other = other.get_bounding_box();
        self.expand_point(other.min);
        self.expand_point(other.max);
    }

    pub fn get_extents_centered(&self) -> [Vec2; 2] {
        let offset = self.max - self.min;
        let extents = offset * 0.5;
        let origin = self.min + extents;
        [origin, extents]
    }

    pub fn with_orientation(self, up: Vec2) -> RectOriented {
        let [origin, extents] = self.get_extents_centered();
        RectOriented{origin, extents, up}
    }

}

impl From<RectOriented> for Rect {
    fn from(value: RectOriented) -> Self {
        Rect{
            min: value.origin - value.extents,
            max: value.origin + value.extents
        }
    }
}
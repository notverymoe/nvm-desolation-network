// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::Vec2;

use crate::{HasBoundingBox, shape::Rect, Projection, SATShape, VecLike};

#[derive(Clone, Copy)]
pub struct Line {
    pub start: Vec2,
    pub end:   Vec2,
}

impl SATShape for Line {

    const CAN_SMEAR_PROJECTION: bool = true;

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        Projection::from_points_iter(axis, [self.start, self.end])
    }

    fn get_points(&self, out_points: &mut impl VecLike<Vec2>) {
        out_points.extend_from_slice(&[self.start, self.end])
    }

    fn get_axes(&self, out_axes: &mut impl VecLike<Vec2>, out_projections: &mut impl VecLike<Projection>) {
        let direction = (self.end - self.start).normalize();
        out_axes.extend_from_slice(&[direction, direction.perp()]);
        out_projections.extend_from_slice(&[
            Projection([direction.dot(self.start), direction.dot(self.end)]),
            Projection::new(direction.perp().dot(self.start)),
        ])
    }

    fn get_axes_derived(&self, _other: &[Vec2], _out_axes: &mut impl VecLike<Vec2>) {
        // no derived axes
    }

    fn with_offset(mut self, offset: Vec2) -> Self {
        self.start += offset;
        self.end   += offset;
        self
    }

}

impl HasBoundingBox for Line {

    fn get_bounding_box(&self) -> Rect {
        let (min, max) = {
            if self.start.length_squared() <= self.end.length_squared() {
                (self.start, self.end)
            } else {
                (self.end, self.start)
            }
        };
        Rect { min, max }
    }

}
// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::Vec2;

use crate::{HasBoundingBox, shape::Rect, Projection, SATShape, VecLike};

#[derive(Clone, Copy)]
pub struct CapsuleOriented {
    pub origin: Vec2,
    pub height: f32,
    pub radius: f32,
    pub up:     Vec2,
}

impl SATShape for CapsuleOriented {

    const CAN_SMEAR_PROJECTION: bool = false;

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        Projection([
            axis.dot(self.origin),
            axis.dot(self.origin + self.up*self.height)
        ]).inflated_by(self.radius)
    }

    fn get_points(&self, out_points: &mut impl VecLike<Vec2>) {
        out_points.extend_from_slice(&[
            self.origin,
            self.origin + self.up*self.height
        ]);
    }

    fn get_axes(&self, out_axes: &mut impl VecLike<Vec2>, out_projections: &mut impl VecLike<Projection>) {
        out_axes.push(self.up.perp());
        let origin_dp = self.up.perp().dot(self.origin);
        out_projections.push(Projection([origin_dp-self.radius, origin_dp+self.radius]));
    }

    fn get_axes_derived(&self, other: &[Vec2], out_axes: &mut impl VecLike<Vec2>) {
        let end = self.origin + self.up*self.height;
        out_axes.extend(other.iter().flat_map(|&p| [p - self.origin, p - end]))
    }

    fn with_offset(mut self, offset: Vec2) -> Self {
        self.origin += offset;
        self
    }

}

impl HasBoundingBox for CapsuleOriented {

    fn get_bounding_box(&self) -> Rect {
        Rect{
            min: self.origin - Vec2::new(self.radius, self.radius),
            max: self.origin + Vec2::new(self.radius, self.radius) + self.up*self.height,
        }
    }

}
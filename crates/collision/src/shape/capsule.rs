// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::Vec2;

use crate::{HasBoundingBox, shape::Rect, Projection, Shape, VecLike};

use super::CapsuleOriented;

#[derive(Clone, Copy)]
pub struct Capsule {
    pub origin: Vec2,
    pub height: f32,
    pub radius: f32,
}

impl Shape for Capsule {

    const CAN_SMEAR_PROJECTION: bool = false;

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        Projection([
            axis.dot(self.origin),
            axis.dot(self.origin + Vec2::new(0.0, self.height))
        ]).inflated_by(self.radius)
    }

    fn get_points(&self, out_points: &mut impl VecLike<Vec2>) {
        out_points.extend_from_slice(&[
            self.origin,
            self.origin + Vec2::new(0.0, self.height)
        ]);
    }

    fn get_axes(&self, out_axes: &mut impl VecLike<Vec2>, out_projections: &mut impl VecLike<Projection>) {
        out_axes.push(Vec2::X);
        out_projections.push(Projection([self.origin.x-self.radius, self.origin.x+self.radius]));
    }

    fn get_axes_derived(&self, other: &[Vec2], out_axes: &mut impl VecLike<Vec2>) {
        let end = self.origin + Vec2::new(0.0, self.height);
        out_axes.extend(other.iter().flat_map(|&p| [
            (p - self.origin).normalize(), 
            (p - end).normalize()
        ]))
    }

    fn with_offset(mut self, offset: Vec2) -> Self {
        self.origin += offset;
        self
    }

}

impl HasBoundingBox for Capsule {

    fn get_bounding_box(&self) -> Rect {
        Rect{
            min: Vec2::new(self.origin.x-self.radius, self.origin.y-self.radius),
            max: Vec2::new(self.origin.x+self.radius, self.origin.y+self.radius+self.height),
        }
    }

}

impl Capsule {

    pub fn with_orientation(self, up: Vec2) -> CapsuleOriented {
        let Capsule{origin, height, radius} = self;
        CapsuleOriented{origin, height, radius, up}
    }
}

impl From<CapsuleOriented> for Capsule {
    fn from(value: CapsuleOriented) -> Self {
        let CapsuleOriented{origin, height, radius, up: _} = value;
        Self{origin, height, radius}
    }
}
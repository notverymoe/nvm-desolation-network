// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::Vec2;

use crate::{HasBoundingBox, shape::Rect, Projection, Shape, VecLike};

use super::{CapsuleOriented, Capsule};

#[derive(Clone, Copy)]
pub struct Circle {
    pub origin: Vec2,
    pub radius: f32,
}

impl Shape for Circle {

    const CAN_SMEAR_PROJECTION: bool = false;

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        let origin = axis.dot(self.origin);
        Projection([origin - self.radius, origin + self.radius])
    }

    fn get_points(&self, out_points: &mut impl VecLike<Vec2>) {
        out_points.push(self.origin)
    }

    fn get_axes(&self, _out_axes: &mut impl VecLike<Vec2>, _out_projections: &mut impl VecLike<Projection>) {
        // Has no axes itself
    }

    fn get_axes_derived(&self, other: &[Vec2], out_axes: &mut impl VecLike<Vec2>) {
        out_axes.extend(other.iter().map(|&v| (v - self.origin).normalize()));
    }

    fn with_offset(mut self, offset: Vec2) -> Self {
        self.origin += offset;
        self
    }

}

impl HasBoundingBox for Circle {

    fn get_bounding_box(&self) -> Rect {
        let offset = Vec2::new(self.radius, self.radius);
        Rect { 
            min: self.origin - offset, 
            max: self.origin + offset,
        }
    }

}

impl Circle {

    pub fn extrude_into_capsule(self, height: f32) -> Capsule {
        let Circle{radius, origin} = self;
        Capsule { origin, radius, height }
    }

    pub fn sweep_into_capsule(self, height: f32, up: Vec2) -> CapsuleOriented {
        let Circle{radius, origin} = self;
        CapsuleOriented { origin, radius, height, up }
    }

}
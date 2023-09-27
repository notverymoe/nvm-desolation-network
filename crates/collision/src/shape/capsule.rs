// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{Sweepable, HasBoundingBox, shape::Rect, Projection, SATShape};

#[derive(Clone, Copy)]
pub struct Capsule {
    pub start:  Vec2,
    pub end:    Vec2,
    pub radius: f32,
}

impl SATShape for Capsule {
    fn project_on_axis(&self, axis: Vec2) -> Projection {
        Projection::from_points_iter(axis, [self.start, self.end]).inflated_by(self.radius)
    }

    fn get_points(&self, out_points: &mut Vec<Vec2>) {
        out_points.extend_from_slice(&[self.start, self.end])
    }

    fn get_axes(&self, out_axes: &mut Vec<Vec2>, out_cache: &mut Vec<Projection>) {
        let direction = (self.end - self.start).normalize();
        out_axes.extend_from_slice(&[direction, direction.perp()]);
        out_cache.extend_from_slice(&[
            Projection([direction.dot(self.start), direction.dot(self.end)]),
            Projection::new(direction.perp().dot(self.start)),
        ])
    }

    fn get_axes_derived(&self, other: &[Vec2], out_axes: &mut Vec<Vec2>) {
        out_axes.extend(
            other.iter().flat_map(|&v| [
                (v - self.start).normalize(), 
                (v - self.end  ).normalize()
            ]),
        );
    }
}

impl Sweepable for Capsule {

    const CAN_SMEAR_PROJECTION: bool = false;

    fn with_offset(mut self, offset: Vec2) -> Self {
        self.start += offset;
        self.end   += offset;
        self
    }

}

impl HasBoundingBox for Capsule {

    fn get_bounding_box(&self) -> Rect {
        let offset = Vec2::new(self.radius, self.radius);
        let (min, max) = {
            if self.start.length_squared() <= self.end.length_squared() {
                (self.start, self.end)
            } else {
                (self.end, self.start)
            }
        };

        Rect { 
            min: min - offset, 
            max: max + offset,
        }
    }

}
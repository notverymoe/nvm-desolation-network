// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::SATShape;

pub trait Sweepable: Copy + SATShape {
    const CAN_SMEAR_PROJECTION: bool;
    fn with_offset(self, offset: Vec2) -> Self;
}

pub struct Sweep<T: Sweepable> {
    motion: Vec2,
    start: T,
    end:   T,
}

impl<T: Sweepable> Sweep<T> {
    pub fn new(shape: T, motion: Vec2) -> Self {
        Self{motion, start: shape, end: shape.with_offset(motion)}
    }

    pub fn start(&self) -> &T {
        &self.start
    }

    pub fn end(&self) -> &T {
        &self.end
    }

    pub fn motion(&self) -> &Vec2 {
        &self.motion
    }
}

impl<T: Sweepable> SATShape for Sweep<T> {
    fn project_on_axis(&self, axis: Vec2) -> crate::Projection {
        let result = self.start.project_on_axis(axis);
        if T::CAN_SMEAR_PROJECTION {
            result.smear_by(axis.dot(self.motion))
        } else {
            result.merged_with(self.end.project_on_axis(axis))
        }
    }

    fn get_points(&self, out_points: &mut Vec<Vec2>) {
        self.start.get_points(out_points);
        self.end.get_points(out_points);
    }

    fn get_axes(&self, out_axes: &mut Vec<Vec2>, out_cache: &mut Vec<crate::Projection>) {
        self.start.get_axes(out_axes, out_cache);
    }

    fn get_axes_derived(&self, other: &[Vec2], out_axes: &mut Vec<Vec2>) {
        if !T::CAN_SMEAR_PROJECTION {
            self.start.get_axes_derived(other, out_axes);
            self.end.get_axes_derived(other, out_axes);
        }
        // Smearable projections can't have derived axes, no-op
    }
}
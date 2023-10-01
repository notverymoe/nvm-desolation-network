// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{Shape, VecLike, Contact};

pub struct Sweep<T: Shape + Copy> {
    motion: Vec2,
    start: T,
    end:   T,
}

impl<T: Shape + Copy> Sweep<T> {
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

    pub fn get_contact_along_motion(&self, contact: Contact) -> Contact {
        contact.reproject(self.motion.normalize())
    }
}

impl<T: Shape + Copy> Shape for Sweep<T> {
    const CAN_SMEAR_PROJECTION: bool = T::CAN_SMEAR_PROJECTION;

    fn project_on_axis(&self, axis: Vec2) -> crate::Projection {
        let result = self.start.project_on_axis(axis);
        if T::CAN_SMEAR_PROJECTION {
            result.smeared_by(axis.dot(self.motion))
        } else {
            result.merged_with(self.end.project_on_axis(axis))
        }
    }

    fn get_points(&self, out_points: &mut impl VecLike<Vec2>) {
        self.start.get_points(out_points);
        self.end.get_points(out_points);
    }

    fn get_axes(&self, out_axes: &mut impl VecLike<Vec2>, out_projections: &mut impl VecLike<crate::Projection>) {
        self.start.get_axes(out_axes, out_projections);
    }

    fn get_axes_derived(&self, other: &[Vec2], out_axes: &mut impl VecLike<Vec2>) {
        if !T::CAN_SMEAR_PROJECTION {
            self.start.get_axes_derived(other, out_axes);
            self.end.get_axes_derived(other, out_axes);
        }
        // Smearable projections can't have derived axes, no-op
    }

    fn with_offset(self, offset: Vec2) -> Self {
        Self { 
            motion: self.motion, 
            start:  self.start.with_offset(offset), 
            end:    self.end.with_offset(offset) 
        }
    }
}
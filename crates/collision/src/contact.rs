// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{Projection, shape::Project};

#[derive(Clone, Copy, Default)]
pub struct Contact {
    pub axis:        Vec2,
    pub contact_min: f32,
    pub contact_max: f32,
}

impl Contact {

    pub fn reproject(&self, axis: Vec2) -> Self {
        let dp = axis.dot(self.axis);
        Self { axis, contact_min: self.contact_min*dp, contact_max: self.contact_max*dp }
    }

    pub fn from_overlap(axis: Vec2, a: Projection, b: Projection) -> Self {
        let contact_a = b.min() - a.max();
        let contact_b = b.max() - a.min();
        let [contact_min, contact_max] = if contact_a.abs() < contact_b.abs() {
            [contact_a,  contact_b]
        } else {
            [contact_b, contact_a ]
        };
        Self{axis, contact_min, contact_max}
    }

    pub fn from_swept_overlap(motion_dir: Vec2, axis: Vec2, a: Projection, b: Projection) -> Self {
        let contact_a = b.min() - a.max();
        let contact_b = b.max() - a.min();
        let [contact_min, contact_max] = if motion_dir.dot(axis) >= 0.0 {
            [contact_a,  contact_b]
        } else {
            [contact_b, contact_a ]
        };
        Self{axis, contact_min, contact_max}
    }

    pub fn is_penetration(&self) -> bool {
        self.contact_min.signum() != self.contact_max.signum()
    }

    pub fn from_projections(axis: Vec2, a: &impl Project, b: &impl Project) -> Self {
        Self::from_overlap(axis, a.project_on_axis(axis), b.project_on_axis(axis))
    }

    pub fn from_swept_projections(motion_dir: Vec2, axis: Vec2, a: &impl Project, b: &impl Project) -> Self {
        Self::from_swept_overlap(motion_dir, axis, a.project_on_axis(axis), b.project_on_axis(axis))
    }

    pub fn reverse(&mut self) {
        self.contact_min = -self.contact_min;
        self.contact_max = -self.contact_max;
    }

}

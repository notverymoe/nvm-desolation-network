// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{shape::{Shape, Project}, Projection};

pub struct Sweep {
    pub start:    Shape,
    pub end:      Shape,
    pub motion:   Vec2,
    pub test_dir: Vec2,
    pub test_dp:  Projection,
}

impl Project for Sweep {
    fn project_aabb(&self) -> [Projection; 2] {
        let [x, y] = self.start.project_aabb();
        [
            x.smeared_by(self.motion.x),
            y.smeared_by(self.motion.y),
        ]
    }

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        self.start.project_on_axis(axis).smeared_by(axis.dot(self.motion))
    }
}
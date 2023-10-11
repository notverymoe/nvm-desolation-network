// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::projection::{Projection, ProjectOnAxis};

#[derive(Debug, Clone, Copy)]
pub struct RectRoundedData {
    pub size:   Vec2,
    pub radius: f32,
}

impl ProjectOnAxis for RectRoundedData {

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        // Don't ask, this works, it's magic. See RectData for some info.
        let axis_dp = axis.abs().dot(self.size) + self.radius;
        Projection([-axis_dp, axis_dp])
    }

}
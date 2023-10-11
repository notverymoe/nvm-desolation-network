// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{shape::ShapeData, projection::{ProjectOnAxis, Projection}};

pub struct MinkowskiData([ShapeData; 2]);

impl ProjectOnAxis for MinkowskiData {
    fn project_on_axis(&self, axis: Vec2) -> Projection {
        let dp = self.0.map(|v| v.project_on_axis(axis));
        dp[0].swept_by(dp[1])
    }
}

pub struct MinkowskiShape {
    pub origin: Vec2,
    pub data:   MinkowskiData,
}

impl ProjectOnAxis for MinkowskiShape {
    fn project_on_axis(&self, axis: Vec2) -> Projection {
        self.data.project_on_axis(axis).offset_by(axis.dot(self.origin))
    }
}
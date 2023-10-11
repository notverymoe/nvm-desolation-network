// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::projection::{Projection, ProjectOnAxis};

#[derive(Debug, Clone, Copy)]
pub struct CircleData {
    pub radius: f32,
}

impl ProjectOnAxis for CircleData {

    fn project_on_axis(&self, _axis: Vec2) -> Projection {
        Projection([-self.radius, self.radius])
    }

}

impl CircleData {

    pub fn raycast(&self, origin: Vec2, axis: Vec2) -> Option<[f32; 2]> {
        let adj = axis.perp_dot(origin);
        if self.radius < adj { return None; }
        let offset = self.radius*(1.0-(adj/self.radius).powi(2)).sqrt();
        Some([-offset, offset])
    }

}
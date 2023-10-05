// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use super::{Projection, Project};

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub origin: Vec2,
    pub radius: f32,
}

impl Project for Circle {
    fn project_aabb(&self) -> [Projection; 2] {
        [
            Projection([self.origin.x - self.radius, self.origin.x + self.radius]),
            Projection([self.origin.y - self.radius, self.origin.y + self.radius]),
        ]
    }

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        let origin = axis.dot(self.origin);
        Projection([origin - self.radius, origin + self.radius])
    }
}
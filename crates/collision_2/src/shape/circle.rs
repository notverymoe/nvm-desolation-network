// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{projection::{Projection, ProjectOnAxis}, ray::{Ray, RaycastTarget}};

#[derive(Debug, Clone, Copy)]
pub struct CircleData {
    pub radius: f32,
}

impl ProjectOnAxis for CircleData {

    fn project_on_axis(&self, _axis: Vec2) -> Projection {
        Projection([-self.radius, self.radius])
    }

}

impl RaycastTarget for CircleData {

    fn raycast(&self, ray: &Ray) -> Option<Projection> {
        ray.find_circle_intersection_at_origin(self.radius)
    }

}
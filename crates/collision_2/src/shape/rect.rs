// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{projection::{Projection, ProjectOnAxis}, ray::{RaycastTarget, Ray}};

#[derive(Debug, Clone, Copy)]
pub struct RectData {
    pub size: Vec2,
}

impl ProjectOnAxis for RectData {
    fn project_aabb(&self) -> [Projection; 2] {
        [
            Projection([-self.size.x, self.size.x]),
            Projection([-self.size.y, self.size.y]),
        ]
    }

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        // Axis points towards a particular corner, Vec2::abs() will 
        // make it point towards Self::size's corner without changing
        // the relative position.
        let axis_dp = axis.abs().dot(self.size);
        Projection([-axis_dp, axis_dp])
    }

}

impl RaycastTarget for RectData {

    fn raycast(&self, ray: &Ray) -> Option<Projection> {
        ray.find_rect_intersection(-self.size, self.size)
    }

}
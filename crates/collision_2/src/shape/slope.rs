// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, Projection, ProjectOnAxis};

#[derive(Debug, Clone, Copy)]
pub struct SlopeData {
    pub direction: Vec2,
    pub length:    f32,
}

impl SlopeData {

    pub fn new(size: Vec2) -> Self {
        let length    =  size.length();
        let direction = -size.perp()/length;
        Self{direction, length}
    }

    pub fn size(&self) -> Vec2 {
        self.direction.perp() * self.length
    }

    pub fn points(&self) -> [Vec2; 3] {
        let size = self.size();
        [
            Vec2::ZERO,
            Vec2::new(0.0, size.y),
            Vec2::new(size.x, 0.0),
        ]
    }

}

impl ProjectOnAxis for SlopeData {
    fn project_aabb(&self) -> [Projection; 2] {
        let size = self.size();
        [
            Projection::new_unsorted(0.0, size.x),
            Projection::new_unsorted(0.0, size.y),
        ]
    }

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        // Axis points towards a particular corner, Vec2::abs() will 
        // make it point towards Self::size's corner without changing
        // the relative position.

        Projection::from_points_iter(axis, self.points())
    }
}


impl RaycastTarget for SlopeData {
    fn raycast(&self, ray: &RayCaster) -> Option<Projection> {
        let size = self.size();
        [
            ray.find_bounded_ray_intersection(Vec2::ZERO, size.x.signum() * Vec2::X, size.x.abs()),
            ray.find_bounded_ray_intersection(Vec2::ZERO, size.y.signum() * Vec2::Y, size.y.abs()),
            ray.find_bounded_ray_intersection(Vec2::new(0.0, size.y), self.direction, self.length),
        ].iter().filter_map(|v| *v).reduce(|c, v| c.merged_with(v))
    }
}
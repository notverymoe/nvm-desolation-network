// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{RaycastTarget, RayCaster, Projection, ProjectOnAxis, NormalAtPoint};

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

    pub fn points_sorted(&self) -> [Vec2; 3] {
        let size = self.size();
        // Ordering for CCW polygon 
        if (size.x >= 0.0) == (size.y >= 0.0) {
            [Vec2::ZERO,  Vec2::new(size.x, 0.0), Vec2::new(0.0, size.y)]
        } else {
            [Vec2::ZERO, Vec2::new(0.0, size.y),  Vec2::new(size.x, 0.0)]
        }
    }

    pub fn normal(&self) -> Vec2 {
        self.direction.perp()
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

impl NormalAtPoint for SlopeData {
    fn normal_at(&self, point: Vec2) -> Vec2 {
        let size = self.size();

        let n = [
            -size.x.signum() * Vec2::X,
            -size.y.signum() * Vec2::Y,
            self.normal(),
        ];

        let dp = [
            -size.x.signum() * point.x,
            -size.y.signum() * point.y,
            n[2].dot(point) - self.length*0.5,
        ];

        let [dp_x, dp_y, dp_s] = dp;
        if (dp_x >= 0.0) && (dp_y >= 0.0) {
            // +XY area
            (point - Vec2::new(size.x, 0.0)).normalize()
        } else if (dp_x >= 0.0) && (dp_s >= 0.0) {
            // +XS area
            (point - Vec2::new(0.0, size.y)).normalize()
        } else if (dp_y >= 0.0) && (dp_s >= 0.0) {
            // +XY area
            point.normalize()
        } else {
            // X, Y, S area
            dp.iter().map(|v| v.abs()).zip(n).min_by(|(a, _), (b, _)| a.total_cmp(b)).unwrap().1
        }
    }
}
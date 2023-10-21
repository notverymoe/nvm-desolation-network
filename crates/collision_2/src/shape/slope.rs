// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::{Vec2, Gizmos, Color};

use crate::{RaycastTarget, RayCaster, Projection, ProjectOnAxis, GizmoRenderable};

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
    
    fn normal_at(&self, point: Vec2) -> Vec2 {
        let size = self.size();
        let size_abs = size.abs();

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

        if dp.iter().any(|&v| v > 0.0) {
            // Is outside

            // if outside n[0] and n[1] segment
            if (dp[1] > 0.0) && (dp[0] > 0.0) {
                // O Region
                return point.normalize();
            }

            let dp_s_perp = self.direction.dot(Vec2::new(point.x, point.y - size.y));

            if (dp[1] < -size_abs.y) && (dp_s_perp < 0.0) {
                // I region
                return (point - Vec2::new(0.0, size.y)).normalize();
            }

            if (dp[0] < -size_abs.x) && (dp_s_perp > self.length) {
                // U region
                return (point - Vec2::new(size.x, 0.0)).normalize();
            }

            return dp.iter().zip(n).filter(|(&v, _)| v >= 0.0).min_by(|(a, _), (b, _)| a.total_cmp(b)).unwrap().1;
        }

        dp.iter().zip(n).max_by(|(a, _), (b, _)| a.total_cmp(b)).unwrap().1
    }
}

impl GizmoRenderable for SlopeData {
    fn render(&self, gizmos: &mut Gizmos, offset: Vec2, color: Color) {
        let size = self.size();
        gizmos.linestrip_2d(
            [
                offset,
                offset + Vec2::new(0.0, size.y),
                offset + Vec2::new(size.x, 0.0),
                offset,
            ],
            color
        );
    }
}
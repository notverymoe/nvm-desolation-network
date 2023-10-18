// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::{Vec2, Gizmos, Color};

use crate::{RaycastTarget, RayCaster, Projection, ProjectOnAxis, NormalAtPoint, GizmoRenderable};

#[derive(Debug, Clone, Copy)]
pub struct SlopeRoundedData {
    pub radius:    f32,
    pub direction: Vec2,
    pub length:    f32,
}


impl SlopeRoundedData {

    pub fn new(size: Vec2, radius: f32) -> Self {
        let length    =  size.length();
        let direction = -size.perp()/length;
        Self{radius, direction, length}
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

impl ProjectOnAxis for SlopeRoundedData {
    fn project_aabb(&self) -> [Projection; 2] {
        let size = self.size();
        [
            Projection::new_unsorted(0.0, size.x).expanded_by(self.radius),
            Projection::new_unsorted(0.0, size.y).expanded_by(self.radius),
        ]
    }

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        Projection::from_points_iter(axis, self.points()).expanded_by(self.radius)
    }
}

impl RaycastTarget for SlopeRoundedData {
    fn raycast(&self, ray: &RayCaster) -> Option<Projection> {
        let size = self.size();
        let off_slope = self.normal() * self.radius;
        let x_off = size.x.signum() * self.radius;
        let y_off = size.y.signum() * self.radius;
        [
            ray.find_circle_intersection(Vec2::ZERO,             self.radius),
            ray.find_circle_intersection(Vec2::new(size.x, 0.0), self.radius),
            ray.find_circle_intersection(Vec2::new(0.0, size.y), self.radius),
            ray.find_bounded_ray_intersection(Vec2::new(0.0, -y_off), size.x.signum() * Vec2::X, size.x.abs()),
            ray.find_bounded_ray_intersection(Vec2::new(-x_off, 0.0), size.y.signum() * Vec2::Y, size.y.abs()),
            ray.find_bounded_ray_intersection(off_slope + Vec2::new(0.0, size.y), self.direction, self.length),
        ].iter().filter_map(|v| *v).reduce(|c, v| c.merged_with(v))
    }
}

impl NormalAtPoint for SlopeRoundedData {
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

impl GizmoRenderable for SlopeRoundedData {
    fn render(&self, gizmos: &mut Gizmos, offset: Vec2, color: Color) {
        let size = self.size();
        
        gizmos.circle_2d(offset, self.radius, color);
        gizmos.circle_2d(offset + Vec2::new(size.x, 0.0), self.radius, color);
        gizmos.circle_2d(offset + Vec2::new(0.0, size.y), self.radius, color);

        let off_slope = self.normal() * self.radius;
        let off_x = size.x.signum() * self.radius;
        let off_y = size.y.signum() * self.radius;

        gizmos.line_2d(
            offset + Vec2::new(   0.0, -off_y),
            offset + Vec2::new(size.x, -off_y),
            color
        );

        gizmos.line_2d(
            offset + Vec2::new(-off_x,    0.0),
            offset + Vec2::new(-off_x, size.y),
            color
        );

        gizmos.line_2d(
            offset + off_slope + Vec2::new(size.x, 0.0),
            offset + off_slope + Vec2::new(0.0, size.y),
            color
        );
    }
}
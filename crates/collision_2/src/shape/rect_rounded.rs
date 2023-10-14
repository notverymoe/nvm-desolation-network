// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{Projection, ProjectOnAxis, RaycastTarget, RayCaster, NormalAtPoint};

#[derive(Debug, Clone, Copy)]
pub struct RectRoundedData {
    pub size:   Vec2,
    pub radius: f32,
}

impl RectRoundedData {
    pub const fn new(size: Vec2, radius: f32) -> Self {
        Self{size, radius}
    }
}

impl NormalAtPoint for RectRoundedData {
    fn normal_at(&self, point: Vec2) -> Vec2 {
        let pnt_abs = point.abs();
        let dist_x = pnt_abs.x - self.size.x; 
        let dist_y = pnt_abs.y - self.size.y;

        // OPT can we make this branchless?

        if dist_x >= 0.0 && dist_y >= 0.0 {
            // In a circle's corner of influence
            point.signum() * (pnt_abs - self.size).normalize()
        } else if dist_x.signum() == dist_y.signum() {
            if dist_x == dist_y {
                Vec2::new(
                    point.x.signum() * std::f32::consts::FRAC_1_SQRT_2,
                    point.y.signum() * std::f32::consts::FRAC_1_SQRT_2,
                )
            } else if dist_x < dist_y {
                Vec2::new(point.x.signum(), 0.0)
            } else {
                Vec2::new(0.0, point.y.signum())
            }
        } else if dist_x.signum() < dist_y.signum() {
            Vec2::new(0.0, point.y.signum())
        } else {
            Vec2::new(point.x.signum(), 0.0)
        }
    }
}

impl ProjectOnAxis for RectRoundedData {
    fn project_aabb(&self) -> [Projection; 2] {
        [
            Projection([-self.size.x - self.radius, self.size.x + self.radius]),
            Projection([-self.size.y - self.radius, self.size.y + self.radius]),
        ]
    }

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        // Don't ask, this works, it's magic. See RectData for some info.
        let axis_dp = axis.abs().dot(self.size) + self.radius;
        Projection([-axis_dp, axis_dp])
    }
}

impl RaycastTarget for RectRoundedData {
    fn raycast(&self, ray: &RayCaster) -> Option<Projection> {
        let min_x = self.size.x;
        let max_x = self.size.x + self.radius;

        let min_y = self.size.y;
        let max_y = self.size.y + self.radius;

        // OPT we could use a modified rect intersecton to only operate on one axis
        [
            ray.find_circle_intersection(Vec2::new( self.size.x,  self.size.y), self.radius),
            ray.find_circle_intersection(Vec2::new(-self.size.x,  self.size.y), self.radius),
            ray.find_circle_intersection(Vec2::new(-self.size.x, -self.size.y), self.radius),
            ray.find_circle_intersection(Vec2::new( self.size.x, -self.size.y), self.radius),
            ray.find_rect_intersection(Vec2::new(-min_x, -max_y), Vec2::new(min_x,  max_y)), // vert test
            ray.find_rect_intersection(Vec2::new(-max_x, -min_y), Vec2::new(max_x,  min_y)), // horz test
        ].iter().filter_map(|v| *v).reduce(|p, c| p.merged_with(c))
    }
}

#[cfg(test)]
mod test {
    use bevy::prelude::Vec2;

    use crate::{RayCaster, RaycastTarget, Projection, RectRoundedData, NormalAtPoint, assert_vec_eq};

    #[test]
    fn raycast_rect_rounded() {
        let target = RectRoundedData::new(Vec2::ONE, 1.0);

        // miss x-axis
        let ray  = RayCaster::new(3.01*Vec2::Y + -3.0 * Vec2::X, Vec2::X);
        let result = target.raycast(&ray);
        assert_eq!(result, None);

        // hit x-axis
        let ray  = RayCaster::new(-3.0 * Vec2::X, Vec2::X);
        let result = target.raycast(&ray);
        assert_eq!(result, Some(Projection([1.0, 5.0])));

        // hit x-axis reverse
        let ray  = RayCaster::new(3.0 * Vec2::X, Vec2::X);
        let result = target.raycast(&ray);
        assert_eq!(result, Some(Projection([-5.0, -1.0])));

        // miss y-axis
        let ray  = RayCaster::new(3.01*Vec2::X + -3.0 * Vec2::Y, Vec2::Y);
        let result = target.raycast(&ray);
        assert_eq!(result, None);

        // hit y-axis
        let ray  = RayCaster::new(-3.0 * Vec2::Y, Vec2::Y);
        let result = target.raycast(&ray);
        assert_eq!(result, Some(Projection([1.0, 5.0])));

        // hit y-axis reverse
        let ray  = RayCaster::new(3.0 * Vec2::Y, Vec2::Y);
        let result = target.raycast(&ray);
        assert_eq!(result, Some(Projection([-5.0, -1.0])));

        // miss 45 deg
        let ray  = RayCaster::new(3.5*Vec2::X + -3.0*Vec2::ONE, Vec2::ONE.normalize());
        let result = target.raycast(&ray);
        assert_eq!(result, None);

        // hit 45 deg
        let ray  = RayCaster::new(-3.0*Vec2::ONE, Vec2::ONE.normalize());
        let result = target.raycast(&ray);
        assert_eq!(result, Some(Projection([
            2.0*std::f32::consts::SQRT_2 - 1.0,
            4.0*std::f32::consts::SQRT_2 + 1.0,
        ])));

        // hit 45 deg reverse
        let ray  = RayCaster::new(3.0*Vec2::ONE, Vec2::ONE.normalize());
        let result = target.raycast(&ray);
        assert_eq!(result, Some(Projection([
            -(4.0*std::f32::consts::SQRT_2 + 1.0),
            -(2.0*std::f32::consts::SQRT_2 - 1.0),
        ])));
    }

    #[test]
    fn normals_rect_rounded() {
        let target = RectRoundedData::new(Vec2::ONE, 1.0);

        assert_vec_eq!(target.normal_at(100.0 *   Vec2::X),  Vec2::X);
        assert_vec_eq!(target.normal_at(100.0 *  -Vec2::X), -Vec2::X);
        assert_vec_eq!(target.normal_at(100.0 *   Vec2::Y),  Vec2::Y);
        assert_vec_eq!(target.normal_at(100.0 *  -Vec2::Y), -Vec2::Y);
        assert_vec_eq!(target.normal_at(100.0 * Vec2::ONE), Vec2::ONE.normalize());

    }

}
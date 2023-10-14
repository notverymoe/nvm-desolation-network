// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{Projection, ProjectOnAxis, RaycastTarget, RayCaster, NormalAtPoint};

#[derive(Debug, Clone, Copy)]
pub struct RectData {
    pub size: Vec2,
}

impl RectData {
    pub const fn new(size: Vec2) -> Self {
        Self{size}
    }
}

impl NormalAtPoint for RectData {
    fn normal_at(&self, point: Vec2) -> Vec2 {
        let pnt_abs = point.abs();
        let dist_x = pnt_abs.x - self.size.x; 
        let dist_y = pnt_abs.y - self.size.y;

        // OPT can we make this branchless?

        if dist_x >= 0.0 && dist_y >= 0.0 {
            Vec2::new(
                point.x.signum() * std::f32::consts::FRAC_1_SQRT_2,
                point.y.signum() * std::f32::consts::FRAC_1_SQRT_2,
            )
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
    fn raycast(&self, ray: &RayCaster) -> Option<Projection> {
        ray.find_rect_intersection(-self.size, self.size)
    }
}

#[cfg(test)]
mod test {
    use bevy::prelude::Vec2;

    use crate::{RayCaster, RaycastTarget, Projection, RectData, NormalAtPoint, assert_vec_eq};

    #[test]
    fn raycast_rect() {
        let target = RectData::new(Vec2::ONE);

        // miss x-axis
        let ray  = RayCaster::new(2.01 * Vec2::Y + -2.0 * Vec2::X, Vec2::X);
        let result = target.raycast(&ray);
        assert_eq!(result, None);

        // hit x-axis
        let ray  = RayCaster::new(-2.0 * Vec2::X, Vec2::X);
        let result = target.raycast(&ray);
        assert_eq!(result, Some(Projection([1.0, 3.0])));

        // hit x-axis reverse
        let ray  = RayCaster::new(2.0 * Vec2::X, Vec2::X);
        let result = target.raycast(&ray);
        assert_eq!(result, Some(Projection([-3.0, -1.0])));

        // miss y-axis
        let ray  = RayCaster::new(2.01 * Vec2::X + -2.0 * Vec2::Y, Vec2::Y);
        let result = target.raycast(&ray);
        assert_eq!(result, None);

        // hit y-axis
        let ray  = RayCaster::new(-2.0 * Vec2::Y, Vec2::Y);
        let result = target.raycast(&ray);
        assert_eq!(result, Some(Projection([1.0, 3.0])));

        // hit y-axis reverse
        let ray  = RayCaster::new(2.0 * Vec2::Y, Vec2::Y);
        let result = target.raycast(&ray);
        assert_eq!(result, Some(Projection([-3.0, -1.0])));

        // miss 45 deg
        let ray  = RayCaster::new(2.01 * Vec2::X + -2.0*Vec2::ONE, Vec2::ONE.normalize());
        let result = target.raycast(&ray);
        assert_eq!(result, None);

        // hit 45 deg
        let ray  = RayCaster::new(-2.0*Vec2::ONE, Vec2::ONE.normalize());
        let result = target.raycast(&ray);
        assert_eq!(result, Some(Projection([
            1.0*std::f32::consts::SQRT_2,
            3.0*std::f32::consts::SQRT_2,
        ])));

        // hit 45 deg reverse
        let ray  = RayCaster::new(2.0*Vec2::ONE, Vec2::ONE.normalize());
        let result = target.raycast(&ray);
        assert_eq!(result, Some(Projection([
            -3.0*std::f32::consts::SQRT_2,
            -1.0*std::f32::consts::SQRT_2,
        ])));
    }

    #[test]
    fn normals_rect() {
        let target = RectData::new(Vec2::ONE);

        assert_vec_eq!(target.normal_at(100.0 *   Vec2::X),  Vec2::X);
        assert_vec_eq!(target.normal_at(100.0 *  -Vec2::X), -Vec2::X);
        assert_vec_eq!(target.normal_at(100.0 *   Vec2::Y),  Vec2::Y);
        assert_vec_eq!(target.normal_at(100.0 *  -Vec2::Y), -Vec2::Y);
        assert_vec_eq!(target.normal_at(100.0 * Vec2::ONE), Vec2::ONE.normalize());

    }

}
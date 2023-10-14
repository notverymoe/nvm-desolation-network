// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{projection::{Projection, ProjectOnAxis}, ray::{RaycastTarget, Ray}};

#[derive(Debug, Clone, Copy)]
pub struct RectData {
    pub size: Vec2,
}

impl RectData {

    pub const fn new(size: Vec2) -> Self {
        Self{size}
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

    fn raycast(&self, ray: &Ray) -> Option<Projection> {
        ray.find_rect_intersection(-self.size, self.size)
    }

}

#[cfg(test)]
mod test {
    use bevy::prelude::Vec2;

    use crate::{ray::{Ray, RaycastTarget}, projection::Projection};

    use super::RectData;

    #[test]
    fn raycast_rect() {
        let target = RectData::new(Vec2::ONE);

        // x-axis
        let ray  = Ray::new(-2.0 * Vec2::X, Vec2::X);
        let result = target.raycast(&ray);
        assert_eq!(result, Some(Projection([1.0, 3.0])));

        // y-axis
        let ray  = Ray::new(-2.0 * Vec2::Y, Vec2::Y);
        let result = target.raycast(&ray);
        assert_eq!(result, Some(Projection([1.0, 3.0])));

        // 45 deg
        let ray  = Ray::new(-2.0*Vec2::ONE, Vec2::ONE.normalize());
        let result = target.raycast(&ray);
        assert_eq!(result, Some(Projection([
            1.0*std::f32::consts::SQRT_2,
            3.0*std::f32::consts::SQRT_2,
        ])));
    }

}
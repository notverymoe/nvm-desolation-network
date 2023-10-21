// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::{Gizmos, Vec2, Color};

use crate::{Projection, ProjectOnAxis, RayCaster, RaycastTarget, GizmoRenderable};

#[derive(Debug, Clone, Copy)]
pub struct CircleData {
    pub radius: f32,
}

impl CircleData {
    pub const fn new(radius: f32) -> Self {
        Self{radius}
    }
}

impl ProjectOnAxis for CircleData {
    fn project_aabb(&self) -> [Projection; 2] {
        [
            Projection([-self.radius, self.radius]),
            Projection([-self.radius, self.radius]),
        ]
    }

    fn project_on_axis(&self, _axis: Vec2) -> Projection {
        Projection([-self.radius, self.radius])
    }
}

impl RaycastTarget for CircleData {
    fn raycast(&self, ray: &RayCaster) -> Option<Projection> {
        ray.find_circle_intersection_at_origin(self.radius)
    }
    
    fn normal_at(&self, point: Vec2) -> Vec2 {
        point.normalize()
    }
}

impl GizmoRenderable for CircleData {
    fn render(&self, gizmos: &mut Gizmos, offset: Vec2, color: Color) {
        gizmos.circle_2d(offset, self.radius, color);
    }
}

#[cfg(test)]
mod test {
    use bevy::prelude::Vec2;

    use crate::{RayCaster, RaycastTarget, Projection, CircleData, assert_vec_eq};

    #[test]
    fn raycast_circle() {
        let target = CircleData::new(1.0);

        // miss x-axis
        let ray  = RayCaster::new(2.01* Vec2::Y + -2.0 * Vec2::X, Vec2::X);
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
        let ray  = RayCaster::new(2.01* Vec2::X + -2.0 * Vec2::Y, Vec2::Y);
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
        let ray  = RayCaster::new(2.01* Vec2::X + -Vec2::ONE, Vec2::Y);
        let result = target.raycast(&ray);
        assert_eq!(result, None);

        // hit 45 deg
        let ray  = RayCaster::new(-Vec2::ONE, Vec2::ONE.normalize());
        let result = target.raycast(&ray);
        assert_eq!(result, Some(Projection([
            std::f32::consts::SQRT_2 - 1.0,
            std::f32::consts::SQRT_2 + 1.0,
        ])));

        // hit 45 deg reverse
        let ray  = RayCaster::new(Vec2::ONE, Vec2::ONE.normalize());
        let result = target.raycast(&ray);
        assert_eq!(result, Some(Projection([
            -(std::f32::consts::SQRT_2 + 1.0),
            -(std::f32::consts::SQRT_2 - 1.0),
        ])));
    }

    #[test]
    fn normals_circle() {
        let target = CircleData::new(1.0);

        assert_vec_eq!(target.normal_at(100.0 *   Vec2::X),  Vec2::X);
        assert_vec_eq!(target.normal_at(100.0 *  -Vec2::X), -Vec2::X);
        assert_vec_eq!(target.normal_at(100.0 *   Vec2::Y),  Vec2::Y);
        assert_vec_eq!(target.normal_at(100.0 *  -Vec2::Y), -Vec2::Y);
        assert_vec_eq!(target.normal_at(100.0 * Vec2::ONE), Vec2::ONE.normalize());

    }

}
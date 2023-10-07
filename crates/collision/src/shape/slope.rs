// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::Projection;

use super::Project;

#[derive(Debug, Clone, Copy)]
pub struct Slope {
    origin:     Vec2,
    run:        f32,
    rise:       f32,
    normal_scl: f32,
}

impl Slope {

    pub const fn from_raw(origin: Vec2, run: f32, rise: f32, normal_scl: f32) -> Self {
        Self { origin, run, rise, normal_scl }
    }

    pub fn new(origin: Vec2, run: f32, rise: f32) -> Self {
        let mut result = Self{origin, run, rise, normal_scl: 0.0};
        result.recalculate_cache();
        result
    }

    pub fn normal(&self) -> Vec2 {
        Vec2::new(self.run, self.rise) * self.normal_scl
    }

}

impl Slope {

    pub fn origin(&self) -> Vec2 {
        self.origin
    }

    pub fn set_origin(&mut self, origin: Vec2) {
        self.origin = origin;
    }

}

impl Slope {

    pub fn point_run(&self) -> Vec2 {
        Vec2::new(self.origin.x + self.run, self.origin.y)
    }

    pub fn point_rise(&self) -> Vec2 {
        Vec2::new(self.origin.x, self.origin.y + self.rise)
    }

    pub fn set_rise_run(&mut self, rise: f32, run: f32) {
        self.rise = rise;
        self.run  = run;
        self.recalculate_cache();
    }

    pub fn get_rise_run(&self) -> [f32; 2] {
        [self.rise, self.run]
    }

}

impl Slope {

    pub fn points(&self) -> [Vec2; 3] {
        // Ordering for CCW polygon 
        if (self.run >= 0.0) == (self.rise >= 0.0) {
            [self.origin,  self.point_run(), self.point_rise()]
        } else {
            [self.origin, self.point_rise(),  self.point_run()]
        }
    }

    pub fn points_unordered(&self) -> [Vec2; 3] {
        [self.origin, self.point_run(), self.point_rise()]
    }

}

impl Slope {

    fn recalculate_cache(&mut self) {
        self.normal_scl = Vec2::new(self.run, self.rise).length_recip();
    }
    
}

impl Project for Slope {
    fn project_aabb(&self) -> [Projection; 2] {
        [
            Projection::new_unsorted(self.origin.x, self.origin.x + self.run ),
            Projection::new_unsorted(self.origin.y, self.origin.y + self.rise),
        ]
    }

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        Projection::from_points_iter(axis, self.points_unordered())
    }


    fn with_offset(&self, o: Vec2) -> Self {
        Self { origin: self.origin + o, rise: self.rise, run: self.run, normal_scl: self.normal_scl }
    }
}

#[cfg(test)]
mod test {
    use bevy::prelude::Vec2;

    use crate::{shape::Project, Projection};

    use super::Slope;

    #[test]
    fn test_slope_projection() {
        let dp = (2.0_f32).sqrt();

        // Q1
        assert_eq!(Slope::new(Vec2::ZERO,  1.0,  1.0).project_on_axis(Vec2::X), Projection([0.0, 1.0]));
        assert_eq!(Slope::new(Vec2::ZERO,  1.0,  1.0).project_on_axis(Vec2::Y), Projection([0.0, 1.0]));
        assert_eq!(Slope::new(Vec2::ZERO,  1.0,  1.0).project_on_axis(Vec2::ONE.normalize()), Projection([0.0, dp*0.5]));
        assert_eq!(Slope::new( Vec2::ONE,  1.0,  1.0).project_on_axis(Vec2::ONE.normalize()), Projection([ dp, dp*1.5]));

        // Q2
        assert_eq!(Slope::new(Vec2::ZERO, -1.0,  1.0).project_on_axis(Vec2::X), Projection([-1.0, 0.0]));
        assert_eq!(Slope::new(Vec2::ZERO, -1.0,  1.0).project_on_axis(Vec2::Y), Projection([ 0.0, 1.0]));
        assert_eq!(Slope::new(Vec2::ZERO, -1.0,  1.0).project_on_axis(Vec2::ONE.normalize()), Projection([-dp*0.5, dp*0.5]));
        assert_eq!(Slope::new( Vec2::ONE, -1.0,  1.0).project_on_axis(Vec2::ONE.normalize()), Projection([ dp*0.5, dp*1.5]));

        // Q4
        assert_eq!(Slope::new(Vec2::ZERO, -1.0, -1.0).project_on_axis(Vec2::X), Projection([-1.0, 0.0]));
        assert_eq!(Slope::new(Vec2::ZERO, -1.0, -1.0).project_on_axis(Vec2::Y), Projection([-1.0, 0.0]));
        assert_eq!(Slope::new(Vec2::ZERO, -1.0, -1.0).project_on_axis(Vec2::ONE.normalize()), Projection([-dp*0.5, 0.0]));
        assert_eq!(Slope::new( Vec2::ONE, -1.0, -1.0).project_on_axis(Vec2::ONE.normalize()), Projection([ dp*0.5,  dp]));

        // Q4
        assert_eq!(Slope::new(Vec2::ZERO,  1.0, -1.0).project_on_axis(Vec2::X), Projection([ 0.0, 1.0]));
        assert_eq!(Slope::new(Vec2::ZERO,  1.0, -1.0).project_on_axis(Vec2::Y), Projection([-1.0, 0.0]));
        assert_eq!(Slope::new(Vec2::ZERO,  1.0, -1.0).project_on_axis(Vec2::ONE.normalize()), Projection([-dp*0.5, dp*0.5]));
        assert_eq!(Slope::new( Vec2::ONE,  1.0, -1.0).project_on_axis(Vec2::ONE.normalize()), Projection([ dp*0.5, dp*1.5]));
    }

}
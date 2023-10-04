// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Slope {
    pub origin: Vec2,
    rise:   f32,
    run:    f32,
    normal: Vec2,
}

impl Slope {

    pub fn point_run(&self) -> Vec2 {
        Vec2::new(self.origin.x + self.run, self.origin.y)
    }

    pub fn point_rise(&self) -> Vec2 {
        Vec2::new(self.origin.x, self.origin.y + self.rise)
    }

    pub fn normal(&self) -> Vec2 {
        self.normal
    }

    pub fn set_rise_run(&mut self, rise: f32, run: f32) {
        self.rise = rise;
        self.run  = run;
        self.normal = Vec2::new(rise, run).normalize();
    }

    pub fn get_rise_run(&self) -> [f32; 2] {
        [self.rise, self.run]
    }

    pub fn points(&self) -> [Vec2; 3] {
        let point_run  = self.origin + Vec2::new(self.run,       0.0);
        let point_rise = self.origin + Vec2::new(     0.0, self.rise);

        if (self.run >= 0.0) == (self.rise >= 0.0) {
            [self.origin, point_run, point_rise]
        } else {
            [self.origin, point_rise, point_run]
        }
    }

}
// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Slope {
    origin:     Vec2,
    rise:       f32,
    run:        f32,
    normal_scl: f32,
}

impl Slope {

    pub fn new(origin: Vec2, rise: f32, run: f32) -> Self {
        let mut result = Self{origin, rise, run, normal_scl: 0.0};
        result.recalculate_cache();
        result
    }

    pub fn normal(&self) -> Vec2 {
        Vec2::new(self.rise, self.run).perp() * self.normal_scl
    }

}

impl Slope {

    pub fn origin(&self) -> Vec2 {
        self.origin
    }

    pub fn set_origin(&mut self, origin: Vec2) {
        self.origin = origin;
        self.recalculate_cache();
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
        self.normal_scl = Vec2::new(self.rise, self.run).length_recip();
    }
    
}
// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::Vec2;

pub trait ProjectOnAxis {
    fn project_aabb(&self) -> [Projection; 2];
    fn project_on_axis(&self, axis: Vec2) -> Projection;
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Projection(pub [f32; 2]);

impl Projection {
    pub const fn new(v: f32) -> Self {
        Self([v,v])
    }

    pub fn new_unsorted(a: f32, b: f32) -> Self {
        Self(if a <= b { [a, b] } else { [b, a] })
    }

    pub fn from_points_iter(axis: Vec2, points: impl IntoIterator<Item = Vec2>) -> Self {
        // OPT we might be able to get this neater
        points.into_iter().fold(Self([f32::INFINITY, f32::NEG_INFINITY]), |p, c| p.expanded_by(axis.dot(c)))
    }

    pub fn get_points(&self, axis: Vec2) -> [Vec2; 2] {
        [axis*self.0[0], axis*self.0[1]]
    }

    pub fn swept_by(mut self, a: Self) -> Self {
        self.sweep(a);
        self
    }

    pub fn merged_with(mut self, a: Self) -> Self {
        self.merge(a);
        self
    }

    pub fn expanded_by(mut self, a: f32) -> Self {
        self.expand(a);
        self
    }

    pub fn inflated_by(mut self, a: f32) -> Self {
        self.inflate(a);
        self
    }

    pub fn smeared_by(mut self, a: f32) -> Self {
        self.smear(a);
        self
    }

    pub fn offset_by(mut self, a: f32) -> Self {
        self.offset(a);
        self
    }

    pub fn reversed(mut self) -> Self {
        self.reverse();
        self
    }

}

impl Projection {

    pub fn sweep(&mut self, a: Self) {
        self.0[0] -= a.min();
        self.0[1] += a.max();
    }

    pub fn merge(&mut self, a: Self) {
        self.0[0] = self.0[0].min(a.min());
        self.0[1] = self.0[1].max(a.max());
    }

    pub fn expand(&mut self, a: f32) {
        self.0[0] = self.0[0].min(a);
        self.0[1] = self.0[1].max(a);
    }

    pub fn inflate(&mut self, a: f32) {
        self.0[0] -= a;
        self.0[1] += a;
    }

    pub fn smear(&mut self, a: f32) {
        if a >= 0.0 {
            self.0[1] += a;
        } else {
            self.0[0] += a;
        }
    }

    pub fn offset(&mut self, a: f32) {
        self.0[0] += a;
        self.0[1] += a;
    }

    pub fn reverse(&mut self) {
        self.0 = [-self.0[1], -self.0[0]];
    }

    pub fn min(&self) -> f32 {
        self.0[0]
    }

    pub fn max(&self) -> f32 {
        self.0[1]
    }

}

impl PartialEq for Projection {

    fn eq(&self, other: &Self) -> bool {
        (self.0[0] - other.0[0]).abs() < 1e-6 && (self.0[1] - other.0[1]).abs() < 1e-6
    }

}
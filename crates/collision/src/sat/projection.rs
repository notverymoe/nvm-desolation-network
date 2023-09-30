// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::Vec2;

#[derive(Clone, Copy, Default)]
pub struct Projection(pub [f32; 2]);

impl Projection {
    pub fn new(v: f32) -> Self {
        Self([v,v])
    }

    pub fn new_unsorted(a: f32, b: f32) -> Self {
        Self(if a <= b { [a, b] } else { [a, b] })
    }

    pub fn from_points_iter(axis: Vec2, points: impl IntoIterator<Item = Vec2>) -> Self {
        // OPT we might be able to get this neater
        points.into_iter().fold(Self([f32::INFINITY, f32::NEG_INFINITY]), |p, c| p.expanded_by(axis.dot(c)))
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

}

impl Projection {

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

    pub fn min(&self) -> f32 {
        self.0[0]
    }

    pub fn max(&self) -> f32 {
        self.0[1]
    }

}
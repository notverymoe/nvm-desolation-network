// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;
use nvm_collide::prelude::*;
use tinyvec::SliceVec;

mod util;
pub use util::*;

#[derive(Debug, Clone, Copy)]
pub struct SensorHit {
    pub entity:   Option<Entity>,
    pub distance: f32,
    pub point:    Vec2,
    pub normal:   Vec2,
}

impl Default for SensorHit {
    fn default() -> Self {
        Self{
            entity: None,
            distance: 0.0,
            point:  Vec2::ZERO,
            normal: Vec2::ZERO,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CollisionCandidate {
    pub entity:   Option<Entity>,
    pub collider: ShapeStatic,
}

impl Default for CollisionCandidate {
    fn default() -> Self {
        Self{
            entity: None,
            collider: Ball::new(Vec2::ZERO, 0.0).into(),
        }
    }
}

pub trait CollisionBroadphase {
    fn find_candidates(&self, collider: &ShapeMoving, direction: Vec2, max_dist: f32, out: &mut SliceVec<CollisionCandidate>) -> usize;
}


pub struct Buffer<T: Default + Copy>(Box<[T]>);

impl<T: Default + Copy> Buffer<T> {

    pub fn new(size: usize) -> Self {
        let mut buffer = Vec::with_capacity(size);
        buffer.resize(size, Default::default());
        Self(buffer.into_boxed_slice())
    }

    pub fn get(&mut self) -> SliceVec<T> {
        SliceVec::from_slice_len(&mut self.0, 0)
    }

}
// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;
use nvm_collide::prelude::*;
use tinyvec::SliceVec;

use crate::{CollisionCandidate, CollisionBroadphase, SensorHit, Buffer};

pub struct SensorSimple {
    buffer_candidates: Buffer<CollisionCandidate>,
    collider: ShapeMoving,
}

impl SensorSimple {

    pub fn new(buffer_size: usize, collider: ShapeMoving, ) -> Self {
        Self{
            buffer_candidates: Buffer::new(buffer_size),
            collider,
        }
    }

    pub fn test_movement(&mut self, broadphase: &impl CollisionBroadphase, motion_dir: Vec2, motion_dist: f32, out: &mut SliceVec<SensorHit>) -> usize {
        Self::test_movement_with(broadphase, &self.collider, motion_dir, motion_dist, &mut self.buffer_candidates.get(), out)
    }

    pub fn test_movement_with(
        broadphase: &impl CollisionBroadphase, 
        collider: &ShapeMoving, 
        motion_dir: Vec2, 
        motion_dist: f32, 
        candidates: &mut SliceVec<CollisionCandidate>,
        out:        &mut SliceVec<SensorHit>
    ) -> usize {
        debug_assert_eq!(out.len(), 0);

        if broadphase.find_candidates(collider, motion_dir, motion_dist, candidates) == 0 {
            return 0;
        }

        let raycaster = RayCaster::new(collider.origin(), motion_dir);
        for candidate in candidates.iter() {
            let combined = ShapeCombined::between_moving_and_static(collider, &candidate.collider);
            if let Some([enter, exit]) = raycaster.test(&combined) {
                // Exclude collisions past max distance
                if enter.distance > motion_dist {
                    continue;
                }

                // Exclude collisions that happen behind us but don't intersect at all
                if exit.distance < 0.0 {
                    // TODO OPT fast path where we don't need the exit calculated
                    continue;
                }

                // Sorted insert
                out.insert(
                    out.partition_point(|v| v.distance < enter.distance),
                    SensorHit{ 
                        entity:   candidate.entity, 
                        distance: enter.distance, 
                        point:    enter.point, 
                        normal:   enter.normal 
                    }
                );
            }
        }

        out.len()
    }

}

// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;
use nvm_collide::prelude::*;
use tinyvec::SliceVec;

use crate::{test_movement_with, CollisionBroadphase, Map, Buffer, CollisionCandidate, SensorHit};

#[derive(Debug, Component, Clone, Copy)]
pub struct PlatformerMotorConfig {
    pub size:        Vec2,
    pub dist_step:   f32,
    pub dist_snap:   f32,
    pub dist_ground: f32,
}

#[derive(Debug, Component, Clone, Copy)]
pub struct PlatformerMotor {
    pub velocity:   Vec2,
    pub allow_step: bool,
    pub allow_snap: bool,
}

#[derive(Debug, Component, Clone, Copy)]
pub struct PlatformerState {
    pub last_motion: Vec2,
    pub on_ground:   bool,
    pub ground_norm: Vec2,
    pub hit_norm:    Option<Vec2>,
}

pub fn motor_apply(
    mut q_motors: Query<(&GlobalTransform, &mut Transform, &mut PlatformerState, &mut PlatformerMotor, &PlatformerMotorConfig)>, 
    r_map:  Res<Map>,
    r_time: Res<Time>,
) {
    // TODO move into component to avoid realloc
    // TODO determine size
    let mut candidates = Buffer::new(32);
    let mut candidates = candidates.get(); 
    let mut hits       = Buffer::new(32);
    let mut hits       = hits.get(); 

    for (global_transform, mut transform, mut state, mut motor, config) in q_motors.iter_mut() {
        candidates.clear();
        hits.clear();

        let new_state = do_motor_apply(&*r_map, config, &motor, r_time.delta_seconds(), global_transform.translation().truncate(), &mut candidates, &mut hits);
        if let Some(hit_norm) = new_state.hit_norm {
            let motor_velocity = motor.velocity;
            motor.velocity += motor_velocity.normalize()*motor_velocity.dot(hit_norm);
        }

        if new_state.on_ground {
            let motor_velocity = motor.velocity;
            motor.velocity += motor_velocity.normalize()*motor_velocity.dot(new_state.ground_norm);
        }
    
        *state = new_state;

        // TODO slide

        transform.translation += new_state.last_motion.extend(0.0);
    }
}

fn do_motor_apply(
    broadphase: &impl CollisionBroadphase,
    config: &PlatformerMotorConfig,
    motor: &PlatformerMotor,
    dt: f32,
    origin: Vec2,
    candidates: &mut SliceVec<CollisionCandidate>,
    hits:       &mut SliceVec<SensorHit>,
) -> PlatformerState {
    let mut collider = create_collider(config, motor, origin);
    let mut new_state = PlatformerState{
        on_ground:   false,
        ground_norm: Vec2::Y,
        last_motion: dt*motor.velocity,
        hit_norm:    None,
    };

    // Check motion
    let motion_dist = new_state.last_motion.length();
    if motion_dist > 0.0 {
        let motion_dir  = new_state.last_motion/motion_dist;
        // Check for collision
        if test_movement_with(broadphase, &collider, motion_dir, motion_dist, candidates, hits) != 0 {
            let hit = hits.first().unwrap();
            new_state.on_ground   = is_ground_norm(hit.normal);
            new_state.ground_norm = if new_state.on_ground { hit.normal } else { Vec2::Y };
            new_state.last_motion = motion_dir * hit.distance;
            new_state.hit_norm    = Some(hit.normal);
        }
    }

    // Apply motion, so we can perform ground checks
    let can_step = motor.allow_step && config.dist_step > 0.0;
    if !new_state.on_ground || can_step {
        // We only apply if we need to do followup checks.
        // There are two cases, when we can step or when we're not on the ground already.
        collider.set_origin(collider.origin() + new_state.last_motion);
    }

    // If we can step, then we might be floating too close to the ground
    // we need to adjus the height of the collider upwards if we find ground
    if can_step {
        candidates.clear();
        hits.clear();
        if let Some([origin_new, ground_norm]) = find_step(broadphase, &collider, config.dist_step, candidates, hits) {
            collider.set_origin(origin_new);
            new_state.on_ground   = true;
            new_state.ground_norm = ground_norm;
        }
    }

    // If we're not already on the ground, we need to either check for ground or try snapping to it
    if !new_state.on_ground {
        let can_snap = motor.allow_snap && config.dist_snap > 0.0;
        candidates.clear();
        hits.clear();

        // If we can snap, then we don't need to perform ground detection
        if can_snap {
            // TODO how should we be applying the ground detection skin here... if at all? does it matter?
            //      we don't handle it in the motion situation... ugh... hate this stuff... come onnnnn...
            let dist_step = if motor.allow_step { config.dist_step } else { 0.0 };
            if let Some([origin_new, ground_norm]) = find_snap(broadphase, &collider, config.dist_snap, dist_step, candidates, hits) {
                collider.set_origin(origin_new);
                new_state.on_ground   = true;
                new_state.ground_norm = ground_norm;
            }
        } else if let Some(ground_norm) = find_ground(broadphase, &collider, config.dist_ground, candidates, hits) {
            new_state.on_ground   = true;
            new_state.ground_norm = ground_norm;
        }
    }

    new_state
}

// TODO (?) make these find functions return a modified sensor hit

fn find_step(
    broadphase: &impl CollisionBroadphase,
    collider:   &ShapeMoving,
    dist_step:  f32,
    candidates: &mut SliceVec<CollisionCandidate>,
    hits:       &mut SliceVec<SensorHit>,
) -> Option<[Vec2; 2]> {
    if test_movement_with(broadphase, collider, -Vec2::Y, dist_step, candidates, hits) != 0 {
        if let Some(hit) = hits.iter().find(is_valid_floor) {
            return Some([
                collider.origin() + (hit.distance - dist_step).max(0.0),
                hit.normal
            ]);
        }
    }
    None
}

fn find_snap(
    broadphase: &impl CollisionBroadphase,
    collider:   &ShapeMoving,
    dist_snap:  f32,
    dist_float: f32,
    candidates: &mut SliceVec<CollisionCandidate>,
    hits:       &mut SliceVec<SensorHit>,
) -> Option<[Vec2; 2]> {
    if test_movement_with(broadphase, collider, -Vec2::Y, dist_snap + dist_float, candidates, hits) != 0 {
        if let Some(hit) = hits.iter().find(is_valid_floor) {
            return Some([
                collider.origin() + (hit.distance - dist_float).min(0.0),
                hit.normal
            ]);
        }
    }
    None
}

fn find_ground(
    broadphase:  &impl CollisionBroadphase,
    collider:    &ShapeMoving,
    dist_ground: f32,
    candidates:  &mut SliceVec<CollisionCandidate>,
    hits:        &mut SliceVec<SensorHit>,
) -> Option<Vec2> {
    if test_movement_with(broadphase, collider, -Vec2::Y, dist_ground, candidates, hits) != 0 {
        if let Some(hit) = hits.iter().find(is_valid_floor) {
            return Some(hit.normal);
        }
    }
    
    None
}

fn create_collider(config: &PlatformerMotorConfig, motor: &PlatformerMotor, origin: Vec2) -> ShapeMoving {
    let dist_step = if motor.allow_step { config.dist_step } else { 0.0 };
    BoxAligned::new(
        Vec2::new(     origin.x,      origin.y - dist_step*0.5),
        Vec2::new(config.size.x, config.size.y - dist_step*0.5)
    ).into()
}

fn is_valid_floor(hit: &&SensorHit) -> bool {
    hit.distance >= 0.0 && is_ground_norm(hit.normal)
}

fn is_ground_norm(normal: Vec2) -> bool {
    normal.dot(Vec2::Y) > 0.01
}
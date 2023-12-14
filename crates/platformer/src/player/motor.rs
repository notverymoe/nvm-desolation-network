// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

#[derive(Debug, Component, Clone, Copy)]
pub struct PlatformerMotorConfig {
    pub size:      Vec2,
    pub dist_step: f32,
    pub dist_snap: f32,
}

#[derive(Debug, Component, Clone, Copy)]
pub struct PlatformerMotor {
    pub velocity:   Vec2,
    pub allow_step: bool,
    pub allow_snap: bool,
}

#[derive(Debug, Component, Clone, Copy)]
pub struct PlatformerState {
    pub on_ground: bool,
}

pub fn motor_apply(
    mut q_motors: Query<(&mut Transform, &GlobalTransform, &mut PlatformerState, &PlatformerMotor, &PlatformerMotorConfig)>, 
    r_time: Res<Time>,
) {
    for (mut transform, globalTransform, state, motor, config) in q_motors.iter_mut() {
        let origin = globalTransform.translation();

        let motion = motor.velocity * r_time.delta_seconds();
        let motion_dist = motion.length();
        if motion_dist <= 0.0 {
            continue; // TODO
        }
        let motion_dir  = motion/motion_dist;

        let origin_delta = origin - globalTransform.translation();
        transform.translation += origin_delta;
    }
}
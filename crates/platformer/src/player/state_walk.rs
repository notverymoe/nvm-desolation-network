// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use nvm_behave::prelude::*;

use crate::{PlatformerMarker, PlatformerMotor, STATE_WALK, ACT_FALL, PlatformerState};

#[derive(Debug, Component, Clone, Copy)]
pub struct PlatformerWalkConfig {
    pub speed: f32,
    pub dir:   f32,
}

pub fn state_walk_enter(
    mut q_platfomer: Query<(&mut PlatformerMotor,)>,
    r_engine: Res<StateEngine<PlatformerMarker>>,
) {
    for entity in r_engine.get_current(STATE_WALK).unwrap_or(&[]).iter().copied() {
        let (mut motor,) = q_platfomer.get_mut(entity).unwrap();
        motor.allow_snap = true;
        motor.allow_step = true;
    }
}

pub fn state_walk_update(
    mut q_platfomer: Query<(&mut PlatformerMotor, &PlatformerWalkConfig)>,
    r_engine: Res<StateEngine<PlatformerMarker>>,
) {
    for entity in r_engine.get_current(STATE_WALK).unwrap_or(&[]).iter().copied() {
        let (mut motor, config) = q_platfomer.get_mut(entity).unwrap();
        motor.velocity.x = config.speed * config.dir;
    }
}

pub fn state_walk_check(
    mut q_platfomer: Query<(&mut StateMachine<PlatformerMarker>, &PlatformerState)>,
    r_engine: Res<StateEngine<PlatformerMarker>>,
) {
    for entity in r_engine.get_current(STATE_WALK).unwrap_or(&[]).iter().copied() {
        let (mut machine, state) = q_platfomer.get_mut(entity).unwrap();
        if !state.on_ground {
            machine.trigger(ACT_FALL);
        }
    }
}
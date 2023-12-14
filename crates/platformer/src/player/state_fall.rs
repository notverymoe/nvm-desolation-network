// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use nvm_behave::prelude::*;

use crate::{PlatformerMarker, PlatformerMotor, PlatformerState, ACT_LAND, STATE_FALL};

#[derive(Debug, Component, Clone, Copy)]
pub struct PlatformerFallConfig {
    pub speed: f32,
}

pub fn state_fall_enter(
    mut q_platfomer: Query<(&mut PlatformerMotor,)>,
    r_engine: Res<StateEngine<PlatformerMarker>>,
) {
    for entity in r_engine.get_current(STATE_FALL).unwrap_or(&[]).iter().copied() {
        let (mut motor,) = q_platfomer.get_mut(entity).unwrap();
        motor.allow_snap = false;
        motor.allow_step = false;
    }
}

pub fn state_fall_update(
    mut q_platfomer: Query<(&mut PlatformerMotor, &PlatformerFallConfig)>,
    r_engine: Res<StateEngine<PlatformerMarker>>,
) {
    for entity in r_engine.get_current(STATE_FALL).unwrap_or(&[]).iter().copied() {
        let (mut motor, config) = q_platfomer.get_mut(entity).unwrap();
        motor.velocity.y = -config.speed;
    }
}

pub fn state_fall_check(
    mut q_platfomer: Query<(&mut StateMachine<PlatformerMarker>, &PlatformerState)>,
    r_engine: Res<StateEngine<PlatformerMarker>>,
) {
    for entity in r_engine.get_current(STATE_FALL).unwrap_or(&[]).iter().copied() {
        let (mut machine, state) = q_platfomer.get_mut(entity).unwrap();
        if state.on_ground {
            machine.trigger(ACT_LAND);
        }
    }
}
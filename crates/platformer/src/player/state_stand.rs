// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use nvm_behave::prelude::*;

use crate::{PlatformerMarker, PlatformerMotor, STATE_STAND};

pub fn state_stand_enter(
    mut q_platfomer: Query<(&mut PlatformerMotor,)>,
    r_engine: Res<StateEngine<PlatformerMarker>>,
) {
    for entity in r_engine.get_entering(STATE_STAND).unwrap_or(&[]).iter().copied() {
        let (mut motor,) = q_platfomer.get_mut(entity).unwrap();
        motor.velocity.x = 0.0;
        motor.velocity.y = 0.0;
        motor.allow_snap = true;
        motor.allow_step = true;
    }
}

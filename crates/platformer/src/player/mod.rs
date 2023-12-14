// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;
use nvm_behave::prelude::*;

mod sensor;
pub use sensor::*;

mod motor;
pub use motor::*;

mod state_stand;
pub use state_stand::*;

mod state_walk;
pub use state_walk::*;

mod state_fall;
pub use state_fall::*;

mod util;
pub use util::*;

pub struct PlatformerMarker;

behave_define!(
    PlatformerMarker,
    STATE_STAND,
    STATE_WALK,
    STATE_JUMP,
    STATE_FALL,
    (ACT_JUMP, STATE_JUMP, [STATE_WALK, STATE_STAND]),
    (ACT_FALL, STATE_FALL, []),
    (ACT_LAND, STATE_WALK, [STATE_FALL]),
);


#[derive(SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PlatformerUpdate {
    ApplyMotor,
    CheckState,
}

pub struct PlatformerPlugin;

impl Plugin for PlatformerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_state_engine_system::<PlatformerMarker>(Update)
            .add_state_transitions::<PlatformerMarker>(&[
                &ACT_FALL,
                &ACT_LAND
            ])
            .configure_sets(Update, (
                PlatformerUpdate::ApplyMotor,
                PlatformerUpdate::CheckState,
            ).chain().after(StateMachineUpdate::OnUpdate))
            .add_systems(Update, (state_stand_enter, state_fall_enter).in_set(StateMachineUpdate::OnEnter))
            .add_systems(Update, (motor_apply).in_set(PlatformerUpdate::ApplyMotor))
            .add_systems(Update, (state_fall_update, state_walk_update).in_set(PlatformerUpdate::CheckState))
            .add_systems(Update, (state_fall_check,  state_walk_check).in_set(PlatformerUpdate::CheckState));
    }
}
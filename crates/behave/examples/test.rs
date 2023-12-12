// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;
use nvm_behave::prelude::*;

struct PlatformerMarker;

behave_define!(
    PlatformerMarker,
    STATE_STAND,
    STATE_JUMP,
    STATE_CROUCH,
    (TRANSITION_JUMP,   STATE_JUMP, [STATE_STAND, STATE_JUMP]),
    (TRANSITION_CROUCH, STATE_JUMP, [STATE_STAND, STATE_JUMP]),
    (TRANSITION_STAND,  STATE_JUMP, [STATE_STAND, STATE_JUMP])
);


fn main() {
    App::new()
        .add_state_engine::<PlatformerMarker>(FixedUpdate)
        .add_state_transitions(&[
            &TRANSITION_JUMP,
            &TRANSITION_CROUCH,
            &TRANSITION_STAND
        ]);
        
}
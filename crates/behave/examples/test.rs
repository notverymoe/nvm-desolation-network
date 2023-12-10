// Copyright 2023 Natalie Baker // AGPLv3 //

use nvm_behave::prelude::*;

struct PlatformerMarker;

behave_define!(
    PlatformerMarker,
    STATE_STAND,
    STATE_JUMP,
    STATE_CROUCH,
    (TRANSITION_JUMP,   STATE_JUMP,   [STATE_STAND, STATE_CROUCH]),
    (TRANSITION_CROUCH, STATE_CROUCH, [STATE_STAND              ]),
    (TRANSITION_STAND,  STATE_STAND,  [STATE_CROUCH             ])
);

fn main() {

    

}
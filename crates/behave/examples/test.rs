// Copyright 2023 Natalie Baker // AGPLv3 //

use nvm_behave::{StateId, Transition};

struct PlatformerStateMarker;

type PlatformerState  = StateId<PlatformerStateMarker>;
type PlatformerAction = Transition<PlatformerStateMarker>;

const STATE_IDLE: PlatformerState = PlatformerState::from_name("STATE_IDLE");
const STATE_WALK: PlatformerState = PlatformerState::from_name("STATE_WALK");
const STATE_JUMP: PlatformerState = PlatformerState::from_name("STATE_JUMP");

const ACTION_JUMP: PlatformerAction = PlatformerAction::from_name("DO_JUMP", STATE_JUMP, &[STATE_IDLE, STATE_WALK]);

fn main() {

    

}
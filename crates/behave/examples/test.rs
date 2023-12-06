// Copyright 2023 Natalie Baker // AGPLv3 //

use nvm_behave::{StateId, Transition};

struct PlatformerStateMarker;

type PlatformerState  = StateId<PlatformerStateMarker>;
type PlatformerAction = Transition<PlatformerStateMarker>;

const STATE_IDLE: PlatformerState = PlatformerState::new("STATE_IDLE");
const STATE_WALK: PlatformerState = PlatformerState::new("STATE_WALK");
const STATE_JUMP: PlatformerState = PlatformerState::new("STATE_JUMP");

// TODO: multiple froms?
const ACTION_JUMP: PlatformerAction = PlatformerAction::new("DO_JUMP", Some(STATE_IDLE), STATE_JUMP);

fn main() {

    

}
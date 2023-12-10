// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::{Vec2, Component};

#[derive(PartialEq, Eq)]
pub enum GroundState {
    Air,
    Ground,
}

#[derive(Component)]
pub struct PlayerController {
    pub state:          GroundState,
    pub count_jumps:    u32,
    pub since_grounded: f32,
    pub since_jump:     f32,
    pub velocity:       Vec2,
}

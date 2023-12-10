// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Component;

#[derive(Component)]
pub struct PlayerInput {
    pub dir_look: f32,
    pub dir_move: f32,
    pub jump_press: bool,
    pub jump_held:  bool,
}
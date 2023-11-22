// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::Vec2;

mod input;
pub use input::*;

mod settings;
pub use settings::*;

pub enum PlayerState {
    Ground,
    Air,
}

pub struct Player {
    state: PlayerState,
    ground_last: Option<f64>,

    velocity: Vec2,
    
    jump_count: u8,
    jump_buffer: Option<f64>,
}
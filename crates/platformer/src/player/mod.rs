// Copyright 2023 Natalie Baker // AGPLv3 //

mod sensor;
pub use sensor::*;

pub struct PlatformerState {
    pub on_ground: bool,
    pub on_wall:   bool,
}

// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerSettings {
    pub speed_vert_gravity: VertSpeedSettings,
    pub speed_vert_jump:    VertSpeedSettings,

    pub speed_horz_ground: FacingSpeedSettings,
    pub speed_horz_air:    FacingSpeedSettings,

    pub control_jump: JumpSettings,
}

pub struct FacingSpeedSettings {
    pub forward:  HorzSpeedSettings,
    pub backward: HorzSpeedSettings
}

pub struct HorzSpeedSettings {
    pub accel: f32,
    pub decel: f32,
    pub speed: f32,
}

impl HorzSpeedSettings {

    pub fn from_limits(
        speed: f32, 
        accel_distance: f32, 
        decel_distance: f32
    ) -> Self {
        Self{
            speed,
            accel: calculate_accel_from_distance_and_speed(accel_distance, speed),
            decel: calculate_accel_from_distance_and_speed(decel_distance, speed),
        }
    }

}

pub struct VertSpeedSettings {
    pub accel: f32,
    pub speed: f32,
}

impl VertSpeedSettings {
    
    pub fn from_limits(
        fall_time_min:   f32,
        jump_height_min: f32,
        jump_height_max: f32,
    ) -> [VertSpeedSettings; 2] {
        let gravity        = calculate_accel_from_distance_and_time(jump_height_min, fall_time_min);
        let fall_time_max  = calculate_time_from_distance_and_accel(jump_height_max, gravity);
        let fall_speed_max = calculate_speed_from_distance_and_accel(fall_time_max, gravity);
        let jump_speed     = calculate_speed_from_distance_and_accel(jump_height_min, gravity);
        let jump_gravity   = calculate_accel_from_distance_and_speed(jump_height_max, jump_speed);
        [
            VertSpeedSettings{
                accel: gravity,
                speed: f32::MAX, //fall_speed_max,
            },
            VertSpeedSettings{
                accel: jump_gravity,
                speed: jump_speed,
            }
        ]

    }
}

pub struct JumpSettings {
    pub max_times:   u32,
    pub time_coyote: f32,
    pub time_buffer: f32,
}

fn calculate_speed_from_distance_and_accel(distance: f32, accel: f32) -> f32 {
    (2.0*accel*distance).sqrt()
}

fn calculate_accel_from_distance_and_speed(distance: f32, speed: f32) -> f32 {
    (speed*speed)/(2.0*distance)
}

fn calculate_accel_from_distance_and_time(distance: f32, time: f32) -> f32 {
    (2.0*distance)/(time*time)
}

fn calculate_time_from_distance_and_accel(distance: f32, accel: f32) -> f32 {
    ((2.0*distance)/accel).sqrt()
}

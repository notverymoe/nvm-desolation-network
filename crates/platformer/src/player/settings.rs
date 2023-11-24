// Copyright 2023 Natalie Baker // AGPLv3 //

pub struct PlayerSettings {
    pub speed_gravity:         VertSpeedSettings,
    pub speed_jump:            VertSpeedSettings,
    pub speed_ground_forward:  HorzSpeedSettings,
    pub speed_ground_backward: HorzSpeedSettings,
    pub speed_air_forward:     HorzSpeedSettings,
    pub speed_air_backward:    HorzSpeedSettings,

    pub control_jump: JumpSettings,
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
    pub limit: f32,
}

impl VertSpeedSettings {
    
    pub fn from_limits(
        max_fall_time:   f32,
        max_jump_time:   f32,
        max_jump_height: f32,
        min_jump_height: f32,
    ) -> [VertSpeedSettings; 2] {
        let gravity = calculate_accel_from_distance_and_time(max_jump_height, max_fall_time);

        // TODO ????
        [
            VertSpeedSettings{
                accel: gravity,
                limit: gravity * max_fall_time,
            },
            VertSpeedSettings{
                accel: calculate_accel_from_distance_and_time(max_jump_height, max_jump_time),
                limit: calculate_velocity_from_distance_and_accel(min_jump_height, gravity),
            }

        ]

    }


}

pub struct JumpSettings {
    pub count:        u8,
    pub coyote_time: f32,
    pub buffer_time: f32,
}

fn calculate_velocity_from_distance_and_accel(distance: f32, accel: f32) -> f32 {
    (2.0*accel*distance).sqrt()
}

fn calculate_accel_from_distance_and_speed(distance: f32, speed: f32) -> f32 {
    return (speed*speed)/(2.0*distance);
}

fn calculate_accel_from_distance_and_time(distance: f32, time: f32) -> f32 {
    return (2.0*distance)/(time*time);
}
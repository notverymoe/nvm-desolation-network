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
            accel: (speed*speed)/(2.0*accel_distance),
            decel: (speed*speed)/(2.0*decel_distance),
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

    }


}

pub struct JumpSettings {
    pub count:        u8,
    pub coyote_time: f32,
    pub buffer_time: f32,
}
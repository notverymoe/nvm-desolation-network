// Copyright 2023 Natalie Baker // AGPLv3 //

pub fn calculate_speed_from_distance_and_accel(distance: f32, accel: f32) -> f32 {
    (2.0*accel*distance).sqrt()
}

pub fn calculate_accel_from_distance_and_speed(distance: f32, speed: f32) -> f32 {
    (speed*speed)/(2.0*distance)
}

pub fn calculate_accel_from_distance_and_time(distance: f32, time: f32) -> f32 {
    (2.0*distance)/(time*time)
}

pub fn calculate_time_from_distance_and_accel(distance: f32, accel: f32) -> f32 {
    ((2.0*distance)/accel).sqrt()
}

// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use nvm_collision::{BoxAligned, RayCaster, DebugShapeData, ShapeCombined, DebugShape, ShapeMoving};

mod input;
pub use input::*;

mod settings;
pub use settings::*;

mod controller;
pub use controller::*;

use crate::Map;

pub fn update_player(
    mut q_player: Query<(&mut Transform, &mut PlayerController, &PlayerInput, &PlayerSettings)>,
    time: Res<Time>,
    map:  Res<Map>,
    // mut gizmos: Gizmos,
) {

    for (mut transform, mut controller, input, settings) in q_player.iter_mut() {
        let mut collider = create_player_collider(transform.translation.truncate());

        // // Calculate Jump Velocity // //

        let can_jump = controller.count_jumps < settings.control_jump.max_times;
        if input.jump_press && can_jump {
            controller.count_jumps += 1;
            controller.velocity.y = settings.speed_vert_jump.speed;
        } else if input.jump_held && controller.velocity.y > 0.0 {
            controller.velocity.y -= settings.speed_vert_jump.accel * time.delta_seconds();
            controller.velocity.y = f32::max(controller.velocity.y, -settings.speed_vert_gravity.speed);
        } else if controller.state == GroundState::Air {
            controller.velocity.y -= settings.speed_vert_gravity.accel * time.delta_seconds();
            controller.velocity.y = f32::max(controller.velocity.y, -settings.speed_vert_gravity.speed);
        }

        if input.jump_held && !can_jump {
            controller.since_jump = 0.0;
        }

        // // Calculate Horz Velocity // //

        if input.dir_move != 0.0 {
            // No accel
            controller.velocity.x = input.dir_move * settings.speed_horz_ground.forward.speed;
        } else {
            controller.velocity.x = 0.0;
        }

        // // Integrate // //

        let motion = controller.velocity * time.delta_seconds();

        if let Some((max_dist, motion_dir)) = test_motion(&collider, motion, &map) {
            let final_motion = max_dist * motion_dir;
            collider.origin += final_motion;
    
            if motion.y < 0.0 && final_motion.y >= 0.0 {
                controller.state = GroundState::Ground;
                controller.count_jumps = 0;
            } else if motion.y > 0.0 || final_motion.y < 0.0 {
                controller.state = GroundState::Air;
            }

            controller.velocity = final_motion / time.delta_seconds();

        }

        transform.translation = collider.origin.extend(transform.translation.z);
    }

}

fn test_motion(
    collider: &BoxAligned, 
    motion: Vec2, 
    map: &Map,
) -> Option<(f32, Vec2)> {
    let mut max_dist = motion.length();
    if max_dist == 0.0 {
        return None;
    }

    let [bound_min, bound_max] = calculate_move_bounds(collider, motion, map);

    let motion_dir = motion/max_dist;
    let ray = RayCaster::new(collider.origin, motion_dir);

    let collider: ShapeMoving = (*collider).into();

    // TODO OPT DDA?
    for x in bound_min.x..bound_max.x {
        for y in bound_min.y..bound_max.y {
            let pos = IVec2::new(x, y);
            if let Some(tile) = map.get_shape(pos) {
                //let mut colour = Color::BLUE;
                let combined = ShapeCombined::between_moving_and_static(&collider, &tile);
                if let Some(hit) = ray.test_enter(&combined) {
                    if hit.distance >= 0.0 {
                        max_dist = max_dist.min(hit.distance);
                        //colour = Color::RED;
                    }
                }
                //if let DebugShapeData::Polygon{points, ..} = tile.get_debug_shape_data() {
                //    gizmos.linestrip_2d((0..points.len()).chain(std::iter::once(0)).map(|i| points[i]), colour);
                //}
            }
        }
    }

    Some((max_dist, motion_dir))
}

fn create_player_collider(origin: Vec2) -> BoxAligned {
    BoxAligned::new(origin, Vec2::ONE * 0.25)
}

fn calculate_move_bounds(collider: &BoxAligned, motion: Vec2, map: &Map) -> [IVec2; 2] {
    let [bound_min, bound_max] = collider.bounds();
    let bound_min = (bound_min.min(bound_min + motion) - Vec2::ONE*0.5).max(Vec2::ZERO).floor().as_ivec2();
    let bound_max = (bound_max.max(bound_max + motion) + Vec2::ONE*0.5).min(Vec2::new(map.width() as f32, map.height() as f32)).ceil().as_ivec2();
    [bound_min, bound_max]
}
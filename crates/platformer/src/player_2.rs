// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{prelude::Component, math::{Vec2, IVec2}, time::Time, ecs::system::{Query, Res}, transform::components::Transform, log::info, gizmos::gizmos::Gizmos, render::color::Color};
use nvm_collision::{BoxAligned, RayCaster, ShapeCombined, ShapeDebugData, ShapeDebug};

use crate::Map;

pub struct PlayerInputMap {
    pub move_right: bool,
    pub move_left:  bool,
    pub jump:       bool,
}

#[derive(Component)]
pub struct Player {
    pub input:     PlayerInputMap,
    pub on_ground: bool,
    pub velocity:  Vec2,
}

pub fn update_player(
    mut q_player: Query<(&mut Player, &mut Transform)>, 
    time: Res<Time>,
    map:  Res<Map>,
    mut gizmos: Gizmos,
) {
    for (mut player, mut transform) in q_player.iter_mut() {
        player.velocity += Vec2::Y * -9.81 * time.delta_seconds();

        if player.input.jump {
            player.velocity = Vec2::Y * 299792458.0;
            player.input.jump = false;
            player.on_ground = false;
        }

        let collider = create_player_collider(transform.translation.truncate());
        let motion = player.velocity * time.delta_seconds();
        gizmos.line_2d(collider.origin, collider.origin + motion, Color::AQUAMARINE);


        let mut max_dist = motion.length();
        if max_dist == 0.0 {
            continue;
        }

        let [bound_min, bound_max] = calculate_move_bounds(&collider, motion, &map);

        let motion_dir = motion/max_dist;
        let ray = RayCaster::new(collider.origin, motion_dir);

        for x in bound_min.x..bound_max.x {
            for y in bound_min.y..bound_max.y {
                let pos = IVec2::new(x, y);
                if let Some(tile) = map.get_shape(pos) {

                    let mut colour = Color::BLUE;
                    
                    let combined = ShapeCombined::between_moving_and_static(&collider.into(), &tile);
                    if let Some(hit) = ray.test_enter(&combined) {
                        if hit.distance >= 0.0 {
                            max_dist = max_dist.min(hit.distance);
                            colour = Color::RED;
                        }
                    }


                    if let ShapeDebugData::Polygon{points, ..} = tile.get_debug_shape_data() {
                        gizmos.linestrip_2d((0..points.len()).chain(std::iter::once(0)).map(|i| points[i]), colour);
                    }
                }
            }
        }

        let final_motion = motion_dir * max_dist;
        info!("{:?}", final_motion);
        transform.translation += final_motion.extend(0.0);
        
        if motion.y < 0.0 && final_motion.y >= 0.0 {
           player.on_ground  = true;
        }

        if final_motion.y != motion.y {
           player.velocity.y = 0.0;
        }
    }
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
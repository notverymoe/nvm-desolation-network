// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;
use nvm_collision::{DebugShapeData, DebugShape};
use nvm_platformer::{Map, update_player, Player, PlayerInputMap};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, update_camera)
        .add_systems(Update, player_controls)
        .add_systems(PostUpdate, (update_player, render_player, render_tiles).chain())
        .run();
}

fn setup(mut commands: Commands) {
    let map = Map::new(&[
        "================================",
        "=-                            -=",
        "=                  =======     =",
        "=                        =     =",
        "=               =    --  =     =",
        "=               =    --  =     =",
        "=                        =     =",
        "============    ======== =     =",
        "=                  ==    =     =",
        "=                  --    =     =",
        "=      =       =       = =     =",
        "=      =       =       = =     =",
        "=   ====    =  ========= =     =",
        "=    --     =            =     =",
        "=           =  ========= =     =",
        "=    --  -- =  =       = =     =",
        "=   =========  =       = =     =",
        "=                        =     =",
        "=-                            -=",
        "================================",
    ]);


    commands.spawn(Camera2dBundle{
        transform: Transform::from_translation(Vec3::new(
            (map.width()  as f32 - 1.0)/2.0, 
            (map.height() as f32 - 1.0)/2.0, 
            0.0
        )),
        projection: OrthographicProjection{
            scale: 1.0/32.0,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn((
        Transform::from_translation(Vec3::new(2.25, 2.25, 0.0)),
        Player{
            input: PlayerInputMap{
                jump: false,
                move_left: false,
                move_right: false,
            },
            on_ground: false,
            velocity: Vec2::ZERO,
        }
    ));

    commands.insert_resource(map);
}

fn update_camera(mut q_camera: Query<&mut Transform, With<Camera2d>>, key: Res<Input<KeyCode>>, time: Res<Time>) {

    let mut dir = Vec2::ZERO;
    
    if key.pressed(KeyCode::Up) {
        dir += Vec2::Y;
    }
    
    if key.pressed(KeyCode::Down) {
        dir -= Vec2::Y;
    }
    
    if key.pressed(KeyCode::Left) {
        dir -= Vec2::X;
    }
    
    if key.pressed(KeyCode::Right) {
        dir += Vec2::X;
    }

    if dir != Vec2::ZERO {
        for mut camera in q_camera.iter_mut() {
            camera.translation += (dir * 10.0 * time.delta_seconds()).extend(0.0);
        }
    }

    
}

fn player_controls(mut q_player: Query<&mut Player>, keys: Res<Input<KeyCode>>) {
    for mut player in q_player.iter_mut() {
        if player.on_ground && keys.pressed(KeyCode::Space) {
            player.input.jump = true;
        }
    }
}

fn render_player(q_player: Query<&Transform, With<Player>>, mut gizmos: Gizmos) {
    for player in q_player.iter() {
        gizmos.rect_2d(
            player.translation.truncate(), 
            0.0, 
            Vec2::ONE * 0.5, 
            Color::RED
        );
    }
}

fn render_tiles(map: Res<Map>, mut gizmos: Gizmos) {

    for x in 0..map.width() {
        for y in 0..map.height() {
            let pos = IVec2::new(x, y);
            if let Some(shape) = map.get_shape(pos) {
                if let DebugShapeData::Polygon{points, ..} = shape.get_debug_shape_data() {
                    gizmos.linestrip_2d((0..points.len()).chain(std::iter::once(0)).map(|i| points[i]), Color::BLACK);
                }
            }
        }
    }

}
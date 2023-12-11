// Copyright 2023 Natalie Baker // AGPLv3 //

pub fn main() {
    
}

// use bevy::prelude::*;
// use nvm_collision::{ShapeDebugData, ShapeDebug};
// use nvm_platformer::{Map, update_player, PlayerController, PlayerInput, PlayerSettings, GroundState, JumpSettings, VertSpeedSettings, FacingSpeedSettings, HorzSpeedSettings};

// pub fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_systems(Startup, setup)
//         .add_systems(PreUpdate, update_camera)
//         .add_systems(Update, player_controls)
//         .add_systems(PostUpdate, (update_player, render_player, render_tiles).chain())
//         .run();
// }

// fn setup(mut commands: Commands) {
//     let map = Map::new(&[
//         "================================",
//         "=-                            -=",
//         "=                  =======     =",
//         "=                        =     =",
//         "=               =    --  =     =",
//         "=               =    --  =     =",
//         "=                        =     =",
//         "============    ======== =     =",
//         "=                  ==    =     =",
//         "=                  --    =     =",
//         "=      =       =       = =     =",
//         "=      =       =       = =     =",
//         "=   ====    =  ========= =     =",
//         "=    --     =            =     =",
//         "=           =  ========= =     =",
//         "=    --  -- =  =       = =     =",
//         "=   =========  =       = =     =",
//         "=                        =     =",
//         "=-                            -=",
//         "================================",
//     ]);


//     commands.spawn(Camera2dBundle{
//         transform: Transform::from_translation(Vec3::new(
//             (map.width()  as f32 - 1.0)/2.0, 
//             (map.height() as f32 - 1.0)/2.0, 
//             0.0
//         )),
//         projection: OrthographicProjection{
//             scale: 1.0/32.0,
//             ..Default::default()
//         },
//         ..Default::default()
//     });

//     let [speed_vert_gravity, speed_vert_jump] = VertSpeedSettings::from_limits(
//         0.25, 
//         1.1, 
//         4.1
//     );

//     commands.spawn((
//         Transform::from_translation(Vec3::new(2.25, 2.25, 0.0)),
//         PlayerController{
//             count_jumps: 0,
//             since_grounded: 0.0,
//             state: GroundState::Air,
//             since_jump: 0.0,
//             velocity: Vec2::ZERO,
//         },
//         PlayerInput{
//             dir_look: 0.0,
//             dir_move: 0.0,
//             jump_held: false,
//             jump_press: false,
//         },
//         PlayerSettings{
//             control_jump: JumpSettings{
//                 max_times: 2,
//                 time_buffer: 0.0,
//                 time_coyote: 0.0,
//             },
//             speed_vert_gravity,
//             speed_vert_jump,
//             speed_horz_air: FacingSpeedSettings{
//                 backward: HorzSpeedSettings::from_limits(5.0, 1.0, 1.0),
//                 forward: HorzSpeedSettings::from_limits(5.0, 1.0, 1.0),
//             },
//             speed_horz_ground: FacingSpeedSettings{
//                 backward: HorzSpeedSettings::from_limits(5.0, 1.0, 1.0),
//                 forward: HorzSpeedSettings::from_limits(5.0, 1.0, 1.0),
//             },
//         }
//     ));

//     commands.insert_resource(map);
// }

// fn update_camera(mut q_camera: Query<&mut Transform, With<Camera2d>>, key: Res<Input<KeyCode>>, time: Res<Time>) {

//     let mut dir = Vec2::ZERO;
    
//     if key.pressed(KeyCode::Up) {
//         dir += Vec2::Y;
//     }
    
//     if key.pressed(KeyCode::Down) {
//         dir -= Vec2::Y;
//     }
    
//     if key.pressed(KeyCode::Left) {
//         dir -= Vec2::X;
//     }
    
//     if key.pressed(KeyCode::Right) {
//         dir += Vec2::X;
//     }

//     if dir != Vec2::ZERO {
//         for mut camera in q_camera.iter_mut() {
//             camera.translation += (dir * 10.0 * time.delta_seconds()).extend(0.0);
//         }
//     }

    
// }

// fn player_controls(mut q_player: Query<&mut PlayerInput>, keys: Res<Input<KeyCode>>) {
//     for mut player in q_player.iter_mut() {
//         player.jump_press = keys.just_pressed(KeyCode::Space);
//         player.jump_held  = keys.pressed(KeyCode::Space);

//         player.dir_move = 0.0;

//         if keys.pressed(KeyCode::A) {
//             player.dir_move -= 1.0;
//         }
        
//         if keys.pressed(KeyCode::D) {
//             player.dir_move += 1.0;
//         }
//     }
// }

// fn render_player(q_player: Query<&Transform, With<PlayerController>>, mut gizmos: Gizmos) {
//     for player in q_player.iter() {
//         gizmos.rect_2d(
//             player.translation.truncate(), 
//             0.0, 
//             Vec2::ONE * 0.5, 
//             Color::RED
//         );
//     }
// }

// fn render_tiles(map: Res<Map>, mut gizmos: Gizmos) {

//     for x in 0..map.width() {
//         for y in 0..map.height() {
//             let pos = IVec2::new(x, y);
//             if let Some(shape) = map.get_shape(pos) {
//                 if let ShapeDebugData::Polygon{points, ..} = shape.get_debug_shape_data() {
//                     gizmos.linestrip_2d((0..points.len()).chain(std::iter::once(0)).map(|i| points[i]), Color::BLACK);
//                 }
//             }
//         }
//     }

// }
// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};

use collision_2::{RectRoundedData, RectData, CircleData, ShapeData, Shape, RayCaster, Projection, SlopeData, SlopeRoundedData, GizmoRenderable, NGonData, RaycastTarget, NGonDataTraced};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup,    setup )
        .add_systems(Update,     (update_raycaster, update_static, check_colliders).chain())
        .add_systems(PostUpdate, render)
        .run();
}

#[derive(Component)]
pub struct StaticCollider(Shape);

impl StaticCollider {

    const RECT:         RectData        = RectData::new(Vec2::new(50.0, 50.0));
    const CIRCLE:       CircleData      = CircleData::new(75.0);
    const RECT_ROUNDED: RectRoundedData = RectRoundedData::new(Vec2::new(50.0, 50.0), 25.0);

    pub fn new(origin: Vec2) -> Self {
        Self(Shape::new(origin, Self::CIRCLE))
    }

    pub fn next(&mut self) {
        self.0.data = match self.0.data {
            ShapeData::Circle(_)       => Self::RECT.into(),
            ShapeData::Rect(_)         => Self::RECT_ROUNDED.into(),
            ShapeData::RectRounded(_)  => SlopeData::new(Vec2::new(100.0, 100.0)).into(),
            ShapeData::Slope(_)        => SlopeRoundedData::new(Vec2::new(75.0, 75.0), 25.0).into(),
            ShapeData::SlopeRounded(s) => NGonData::<3>::new(s.points_sorted()).into(),
            ShapeData::NGon3(s)        => NGonDataTraced::<RectData, 3>::new(*s.points(), RectData::new(Vec2::new(20.0, 20.0))).into(),
            ShapeData::NGonTraced3(_)  => Self::CIRCLE.into(),
        }
    }

}

#[derive(Component)]
pub struct RayCasterCollider {
    origin: Vec2,
    target: Vec2,

    hits: Vec<(Entity, Projection)>,
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(StaticCollider::new(Vec2::new( 300.0,  300.0)));
    commands.spawn(StaticCollider::new(Vec2::new(-300.0,  300.0)));
    commands.spawn(StaticCollider::new(Vec2::new(-300.0, -300.0)));
    commands.spawn(StaticCollider::new(Vec2::new( 300.0, -300.0)));

    commands.spawn(StaticCollider::new(Vec2::new( 200.0,    0.0)));
    commands.spawn(StaticCollider::new(Vec2::new(   0.0,  200.0)));
    commands.spawn(StaticCollider::new(Vec2::new(-200.0,   0.0)));
    commands.spawn(StaticCollider::new(Vec2::new(   0.0, -200.0)));

    commands.spawn(RayCasterCollider{origin: -Vec2::X * 200.0, target: Vec2::ZERO, hits: Vec::default()});
}

fn update_static(
    mut q: Query<&mut StaticCollider>, 
    keys: Res<Input<KeyCode>>
) {
    if keys.just_pressed(KeyCode::Backslash) {
        for mut collider in q.iter_mut() {
            collider.next();
        }
    }
}

fn update_raycaster(
    mut q: Query<&mut RayCasterCollider>, 
    keys: Res<Input<KeyCode>>,
    time: Res<Time>
 ) {
    let mut caster = q.get_single_mut().unwrap();
    let mut offset_origin  = Vec2::ZERO;
    let mut offset_target = Vec2::ZERO;

    if keys.pressed(KeyCode::W) {
        offset_origin += Vec2::Y;
    }

    if keys.pressed(KeyCode::A) {
        offset_origin -= Vec2::X;
    }

    if keys.pressed(KeyCode::S) {
        offset_origin -= Vec2::Y;
    }

    if keys.pressed(KeyCode::D) {
        offset_origin += Vec2::X;
    }

    if offset_origin != Vec2::ZERO {
        offset_origin *= 200.0 * time.delta_seconds();
        caster.origin += offset_origin;
    }

    if keys.pressed(KeyCode::I) {
        offset_target += Vec2::Y;
    }

    if keys.pressed(KeyCode::J) {
        offset_target -= Vec2::X;
    }

    if keys.pressed(KeyCode::K) {
        offset_target -= Vec2::Y;
    }

    if keys.pressed(KeyCode::L) {
        offset_target += Vec2::X;
    }

    if offset_target != Vec2::ZERO {
        offset_target *= 200.0 * time.delta_seconds();
        caster.target += offset_target;
    }
}

fn check_colliders(
    mut q_caster:  Query<&mut RayCasterCollider>,
    q_static: Query<(Entity, &StaticCollider)>,
) {
    for mut caster in q_caster.iter_mut() {
        caster.hits.clear();
        let ray = RayCaster::new(caster.origin, (caster.target - caster.origin).normalize());
        for (shape_id, StaticCollider(shape)) in q_static.iter() {
            if let Some(projection) = ray.test_static(shape) {
                caster.hits.push((shape_id, projection));
            }
        }
    }

}

fn render(
    mut gizmos: Gizmos, 
    q_caster:  Query<&RayCasterCollider>,
    q_static: Query<(Entity, &StaticCollider)>,
) {
    for (shape_id, StaticCollider(shape)) in q_static.iter() {
        shape.render(&mut gizmos, Vec2::ZERO, if q_caster.iter().any(|v| v.hits.iter().any(|v| shape_id == v.0)) {
            Color::RED
        } else {
            Color::GREEN
        });
    }

    for caster in q_caster.iter() {
        let dir = (caster.target - caster.origin).normalize();

        gizmos.circle_2d(caster.origin, 10.0, Color::GREEN);
        gizmos.circle_2d(caster.target, 10.0, Color::GREEN);
        gizmos.line_2d(caster.origin - dir*6000.0, caster.target + dir*6000.0, if caster.hits.is_empty() { Color::LIME_GREEN } else { Color::PINK });

        for (hit_id, hit) in caster.hits.iter() {
            let [start, end] = hit.get_points(dir).map(|v| caster.origin + v);
            gizmos.line_2d(start, end, Color::PURPLE);

            let hit_shape = &q_static.get(*hit_id).unwrap().1.0;
            let start_norm = hit_shape.normal_at(start);
            gizmos.line_2d(start, start + start_norm*50.0, Color::BLUE);

            let end_norm = hit_shape.normal_at(end);
            gizmos.line_2d(end, end + end_norm*50.0, Color::BLUE);
        }
        
    }
}

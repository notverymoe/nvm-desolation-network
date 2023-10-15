// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};

use collision_2::{RectRoundedData, RectData, CircleData, ShapeData, Shape, Projection, ShapeCaster, NormalAtPoint, SlopeData};

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
    const CIRCLE:       CircleData      = CircleData::new(50.0);
    const RECT_ROUNDED: RectRoundedData = RectRoundedData::new(Vec2::new(25.0, 25.0), 25.0);

    pub fn new(origin: Vec2) -> Self {
        Self(Shape::new(origin, Self::CIRCLE))
    }

    pub fn next(&mut self) {
        self.0.data = match self.0.data {
            ShapeData::Circle(_)      => Self::RECT.into(),
            ShapeData::Rect(_)        => Self::RECT_ROUNDED.into(),
            ShapeData::RectRounded(_) => SlopeData::new(Vec2::new(25.0, 25.0)).into(),
            ShapeData::Slope(_)       => Self::CIRCLE.into(),
        }
    }

}

#[derive(Component)]
pub struct RayCasterCollider {
    collider: StaticCollider,
    target: Vec2,
    hits: Vec<(Entity, Projection, Shape)>,
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

    commands.spawn(RayCasterCollider{
        target: Vec2::X * 400.0, 
        collider: StaticCollider::new(-Vec2::X * 400.0),
        hits: Vec::default()
    });
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

    if keys.just_pressed(KeyCode::Tab) {
        caster.collider.next();
    }

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
        caster.collider.0.origin += offset_origin;
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
        let ray = ShapeCaster::new(&caster.collider.0, (caster.target - caster.collider.0.origin).normalize());
        for (shape_id, StaticCollider(shape)) in q_static.iter() {
            if let Some((projection, shape)) = ray.test_static(shape) {
                caster.hits.push((shape_id, projection, shape));
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
        render_shape(&mut gizmos, shape, if q_caster.iter().any(|v| v.hits.iter().any(|v| shape_id == v.0)) {
            Color::RED
        } else {
            Color::GREEN
        });
    }

    for caster in q_caster.iter() {
        let caster_origin = caster.collider.0.origin;
        let dir = (caster.target - caster_origin).normalize();

        render_shape(&mut gizmos, &caster.collider.0, Color::YELLOW);

        gizmos.circle_2d(caster.target, 10.0, Color::YELLOW);

        gizmos.line_2d(caster_origin - dir*6000.0, caster.target + dir*6000.0, if caster.hits.is_empty() { Color::LIME_GREEN } else { Color::PINK });

        for (_hit_id, hit, hit_shape) in caster.hits.iter() {
            let [start, _end] = hit.get_points(dir).map(|v| caster_origin + v);

            render_shape(&mut gizmos, hit_shape, Color::SALMON);
            render_shape(&mut gizmos, &Shape::new(start, caster.collider.0.data), Color::PURPLE);

            // TODO get normal at point
            //let hit_shape  = &q_static.get(*hit_id).unwrap().1.0;
            let start_norm = hit_shape.normal_at(start);
            //let [hit_point, _]  = hit_shape.raycast(&RayCaster::new(start, -start_norm)).unwrap().get_points(-start_norm).map(|v| start + v); // TODO better
            gizmos.line_2d(start, start + start_norm*50.0, Color::BLUE);
        }
        
    }
}

fn render_shape(gizmos: &mut Gizmos, shape: &Shape, color: Color) {
    match shape.data {
        ShapeData::Circle(s) => {
            gizmos.circle_2d(shape.origin, s.radius, color);
        },
        ShapeData::Rect(s) => {
            gizmos.rect_2d(shape.origin, 0.0, s.size*2.0, color);
        },
        ShapeData::RectRounded(s) => {
            gizmos.arc_2d(shape.origin + Vec2::new( s.size.x,  s.size.y), f32::to_radians( 45.0), f32::to_radians(90.0), s.radius, color);
            gizmos.arc_2d(shape.origin + Vec2::new(-s.size.x,  s.size.y), f32::to_radians(315.0), f32::to_radians(90.0), s.radius, color);
            gizmos.arc_2d(shape.origin + Vec2::new(-s.size.x, -s.size.y), f32::to_radians(225.0), f32::to_radians(90.0), s.radius, color);
            gizmos.arc_2d(shape.origin + Vec2::new( s.size.x, -s.size.y), f32::to_radians(135.0), f32::to_radians(90.0), s.radius, color);

            gizmos.line_2d(
                shape.origin + Vec2::new( s.size.x, s.size.y + s.radius ),
                shape.origin + Vec2::new(-s.size.x, s.size.y + s.radius ),
                color
            );

            gizmos.line_2d(
                shape.origin + Vec2::new(-(s.size.x + s.radius),  s.size.y),
                shape.origin + Vec2::new(-(s.size.x + s.radius), -s.size.y),
                color
            );

            gizmos.line_2d(
                shape.origin + Vec2::new(-s.size.x, -(s.size.y + s.radius)),
                shape.origin + Vec2::new( s.size.x, -(s.size.y + s.radius)),
                color
            );

            gizmos.line_2d(
                shape.origin + Vec2::new(s.size.x + s.radius, -s.size.y),
                shape.origin + Vec2::new(s.size.x + s.radius,  s.size.y),
                color
            );
        },
        ShapeData::Slope(s) => {
            let size = s.size();
            gizmos.linestrip_2d(
                [
                    shape.origin,
                    shape.origin + Vec2::new(0.0, size.y),
                    shape.origin + Vec2::new(size.x, 0.0),
                    shape.origin,
                ],
                color
            );
        },
    }
}


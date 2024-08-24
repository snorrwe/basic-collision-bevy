use basic_collision_bevy::prelude::*;
use bevy::prelude::*;

fn setup(mut cmd: Commands) {
    cmd.spawn(AABBBundle {
        aabb: AABB::from_size(Vec2::new(100.0, 200.0)),
        filter: CollisionFilter::all(),
        ..Default::default()
    })
    .insert(Visualize);

    cmd.spawn(AABBBundle {
        aabb: AABB::from_size(Vec2::new(100.0, 200.0)),
        filter: CollisionFilter::all(),
        transform: TransformBundle {
            local: Transform::from_translation(Vec3::new(50.0, 25.0, 0.0)),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Visualize);

    cmd.spawn(AABBBundle {
        aabb: AABB::from_size(Vec2::new(100.0, 200.0)),
        filter: CollisionFilter::all(),
        transform: TransformBundle {
            local: Transform::from_translation(Vec3::new(-200.0, -25.0, 0.0)),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Visualize);

    cmd.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CollisionPlugin))
        .add_systems(Startup, setup)
        .run();
}

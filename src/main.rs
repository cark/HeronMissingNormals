use bevy::prelude::*;
use heron::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(init)
        .add_system(movement)
        .add_system(collision_detection)
        .run();
}

#[derive(Component)]
struct Speed(Vec2);

fn spawn_entity(commands: &mut Commands, position: Vec2, speed: Vec2) {
    commands
        .spawn()
        .insert(Speed(speed))
        .insert(Transform::from_translation(position.extend(0.0)))
        .insert(GlobalTransform::default())
        .insert(RigidBody::Sensor)
        .insert(CollisionShape::Sphere { radius: 10.0 });
}

fn init(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    spawn_entity(&mut commands, Vec2::new(-100.0, 0.0), Vec2::new(1.0, 0.0));
    spawn_entity(&mut commands, Vec2::new(100.0, 0.0), Vec2::new(-1.0, 0.0));
}

fn movement(mut query: Query<(&Speed, &mut Transform)>) {
    for (speed, mut transform) in query.iter_mut() {
        transform.translation.x += speed.0.x;
        transform.translation.y += speed.0.y;
    }
}

fn collision_detection(mut events: EventReader<CollisionEvent>) {
    for event in events.iter() {
        if let CollisionEvent::Started(d1, d2) = event {
            info!(
                "normal count 1: {}, normal count 2: {}",
                d1.normals().len(),
                d2.normals().len()
            );
        }
    }
}

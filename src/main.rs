use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use rand::random;
use rand::Rng;

const CAMERA_SPEED: f32 = 500.;
const CAMERA_DECAY_RATE: f32 = 8.;
const CAMERA_ZOOM_SPEED: f32 = 3.;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .add_systems(Startup, (setup_camera, setup_physics, setup_world))
        .add_systems(Update, brownian_motion)
        .add_systems(Update, (move_camera, update_camera, zoom_camera).chain())
        .run();
}

fn setup_world(mut commands: Commands) {
    commands
        .spawn(Player)
        .insert(Transform::from_xyz(0.0, 0.0, 0.0));
}

fn setup_camera(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2d::default());
}

fn square(commands: &mut Commands, position: Vec2, velocity: Vec2, ang_velocity: f32) {
    let half_size = 25.0;

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(half_size, half_size))
        .insert(Transform::from_xyz(position.x, position.y, 0.0))
        .insert(GravityScale(0.0))
        .insert(Velocity { linvel: velocity, angvel: ang_velocity })
        .insert(Damping{ linear_damping: 0.5, angular_damping: 0.5 })
        .insert(Sleeping::disabled())
        .insert(Sprite {
            color: Color::srgb(random::<f32>(), random::<f32>(), random::<f32>()),
            custom_size: Some(Vec2::new(half_size*2.0, half_size*2.0)),
            ..Default::default()
        })
        .insert(Visibility::default())
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        });
}

fn setup_physics(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    for _ in 0..50 {
        let position = Vec2::new(rng.gen_range(-300.0..300.0), rng.gen_range(-300.0..300.0));
        let velocity = Vec2::new(rng.gen_range(-50.0..50.0), rng.gen_range(-50.0..50.0));
        let ang_velocity = rng.gen_range(-5.0..5.0);
        square(&mut commands, position, velocity, ang_velocity);
    }
}

fn brownian_motion(mut ext_impulses: Query<&mut ExternalImpulse>) {
    for mut ext_impulse in ext_impulses.iter_mut() {
        let mut rng = rand::thread_rng();
        let magnitude = 10000.0;

        ext_impulse.impulse = Vec2::new(rng.gen_range(-1.0*magnitude..1.0*magnitude), rng.gen_range(-1.0*magnitude..1.0*magnitude));
        ext_impulse.torque_impulse = rng.gen_range(-10.0*magnitude..10.0*magnitude);
    }

}

fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let Ok(player) = player.get_single() else {
        return;
    };

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}

fn move_camera(
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }


    let move_delta = direction.normalize_or_zero() * CAMERA_SPEED * time.delta_secs();
    player.translation += move_delta.extend(0.);
}

fn zoom_camera(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    for scroll_event in scroll_events.read() {
        let zoom_delta = scroll_event.y * CAMERA_ZOOM_SPEED * time.delta_secs();
        camera.scale = Vec3::splat(camera.scale.x - zoom_delta);
    }
}
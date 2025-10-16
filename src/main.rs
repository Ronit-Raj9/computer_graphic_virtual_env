use bevy::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_scene, setup_ui))
        .add_systems(Update, (orbit_system, rotate_system, bob_system, camera_controls, day_night_toggle))
        .insert_resource(ClearColor(Color::srgb(0.5, 0.7, 1.0))) // Light blue sky
        .run();
}

// Components for movement
#[derive(Component)]
struct Orbits {
    center: Vec3,
    radius: f32,
    speed: f32,
    angle: f32,
}

#[derive(Component)]
struct Rotates {
    axis: Vec3,
    speed: f32,
}

#[derive(Component)]
struct Bobs {
    amplitude: f32,
    frequency: f32,
    phase: f32,
}

#[derive(Component)]
struct ObservatoryDome;

#[derive(Component)]
struct FloatingRock;

#[derive(Component)]
struct Island;

// Resources
#[derive(Resource)]
struct DayNightState {
    is_day: bool,
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });

    // Directional light (sun)
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::srgb(1.0, 0.95, 0.8),
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(-PI / 4.0)),
        ..default()
    });

    // Island base (cone)
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cone {
                radius: 3.0,
                height: 2.0,
            }),
            material: materials.add(Color::srgb(0.4, 0.3, 0.2)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Island,
        Bobs {
            amplitude: 0.3,
            frequency: 0.5,
            phase: 0.0,
        },
    ));

    // Observatory base (cylinder)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cylinder {
            radius: 1.0,
            half_height: 1.0,
        }),
        material: materials.add(Color::srgb(0.8, 0.8, 0.9)),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    });

    // Observatory dome (sphere)
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere {
                radius: 1.2,
            }),
            material: materials.add(Color::srgb(0.9, 0.9, 0.95)),
            transform: Transform::from_xyz(0.0, 2.5, 0.0),
            ..default()
        },
        ObservatoryDome,
        Rotates {
            axis: Vec3::Y,
            speed: 0.3,
        },
    ));

    // Telescope (cylinder)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cylinder {
            radius: 0.1,
            half_height: 1.0,
        }),
        material: materials.add(Color::srgb(0.3, 0.3, 0.3)),
        transform: Transform::from_xyz(0.0, 3.0, 0.0)
            .with_rotation(Quat::from_rotation_x(PI / 2.0)),
        ..default()
    });

    // Telescope lens (small sphere)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere {
            radius: 0.15,
        }),
        material: materials.add(Color::srgb(0.1, 0.1, 0.2)),
        transform: Transform::from_xyz(0.0, 3.0, 1.0),
        ..default()
    });

    // Floating rocks
    let rock_positions = [
        (4.0, 0.0, 0.0),
        (-3.5, 0.0, 2.5),
        (2.0, 0.0, -4.0),
        (-2.0, 0.0, -3.0),
        (3.0, 0.0, 3.0),
        (-4.0, 0.0, -1.0),
    ];

    for (i, (x, y, z)) in rock_positions.iter().enumerate() {
        let radius = Vec3::new(*x, *y, *z).length();
        let angle = (*x).atan2(*z);
        
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid {
                    half_size: Vec3::new(0.4, 0.4, 0.4),
                }),
                material: materials.add(Color::srgb(0.3, 0.25, 0.2)),
                transform: Transform::from_xyz(*x, *y + 1.0, *z),
                ..default()
            },
            FloatingRock,
            Orbits {
                center: Vec3::new(0.0, 1.0, 0.0),
                radius,
                speed: 0.2 + (i as f32 * 0.1),
                angle,
            },
            Bobs {
                amplitude: 0.2,
                frequency: 0.8 + (i as f32 * 0.1),
                phase: i as f32 * 0.5,
            },
        ));
    }

    // Initialize day/night state
    commands.insert_resource(DayNightState { is_day: true });
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(
        TextBundle::from_section(
            "Floating Island Observatory\n\nControls:\nWASD - Camera movement\nSPACE - Toggle day/night",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

fn orbit_system(time: Res<Time>, mut query: Query<(&mut Transform, &mut Orbits)>) {
    for (mut transform, mut orbit) in query.iter_mut() {
        orbit.angle += orbit.speed * time.delta_seconds();
        
        let x = orbit.center.x + orbit.radius * orbit.angle.cos();
        let z = orbit.center.z + orbit.radius * orbit.angle.sin();
        
        transform.translation.x = x;
        transform.translation.z = z;
    }
}

fn rotate_system(time: Res<Time>, mut query: Query<(&mut Transform, &Rotates)>) {
    for (mut transform, rotation) in query.iter_mut() {
        let rotation_angle = rotation.speed * time.delta_seconds();
        transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(rotation.axis, rotation_angle));
    }
}

fn bob_system(time: Res<Time>, mut query: Query<(&mut Transform, &Bobs)>) {
    for (mut transform, bob) in query.iter_mut() {
        let bob_offset = bob.amplitude * (bob.frequency * time.elapsed_seconds() + bob.phase).sin();
        // Store the base Y position and add the bob offset
        let base_y = if bob.phase == 0.0 { 0.0 } else { 1.0 }; // Island at 0, rocks at 1
        transform.translation.y = base_y + bob_offset;
    }
}

fn camera_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera_transform = camera_query.single_mut();
    let mut rotation = 0.0;
    let mut zoom = 0.0;

    if keyboard_input.pressed(KeyCode::KeyA) {
        rotation += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        rotation -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        zoom -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        zoom += 1.0;
    }

    if rotation != 0.0 {
        camera_transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(rotation * time.delta_seconds()),
        );
    }

    if zoom != 0.0 {
        let forward = camera_transform.forward();
        camera_transform.translation += forward * zoom * time.delta_seconds() * 5.0;
    }
}

fn day_night_toggle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut clear_color: ResMut<ClearColor>,
    mut light_query: Query<&mut DirectionalLight>,
    mut day_night: ResMut<DayNightState>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        day_night.is_day = !day_night.is_day;
        
        if day_night.is_day {
            clear_color.0 = Color::srgb(0.5, 0.7, 1.0); // Light blue
            for mut light in light_query.iter_mut() {
                light.color = Color::srgb(1.0, 0.95, 0.8);
                light.illuminance = 10000.0;
            }
        } else {
            clear_color.0 = Color::srgb(0.05, 0.05, 0.2); // Dark navy
            for mut light in light_query.iter_mut() {
                light.color = Color::srgb(0.3, 0.3, 0.8);
                light.illuminance = 2000.0;
            }
        }
    }
}
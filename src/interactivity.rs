use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;

pub struct InteractivityPlugin;

impl Plugin for InteractivityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MushroomNoise>()
            .init_resource::<MushroomCount>()
            .add_event::<MushroomCollected>()
            .add_systems(Startup, spawn_initial_mushrooms)
            .add_systems(Update, (spawn_mushrooms_around_camera, check_mushroom_collection, handle_collection_effects));
    }
}

#[derive(Resource)]
pub struct MushroomNoise {
    pub noise: Perlin,
}

impl Default for MushroomNoise {
    fn default() -> Self {
        Self {
            noise: Perlin::new(99999), // Different seed
        }
    }
}

#[derive(Resource)]
pub struct MushroomCount {
    pub collected: u32,
}

impl Default for MushroomCount {
    fn default() -> Self {
        Self {
            collected: 0,
        }
    }
}

#[derive(Component)]
pub struct Mushroom {
    pub glow_intensity: f32,
}

#[derive(Component)]
pub struct Collectible;

#[derive(Event)]
pub struct MushroomCollected {
    pub position: Vec3,
}

fn spawn_initial_mushrooms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mushroom_noise: Res<MushroomNoise>,
) {
    spawn_mushrooms_in_area(&mut commands, &mut meshes, &mut materials, &mushroom_noise, -50.0, 50.0, -50.0, 50.0);
}

fn spawn_mushrooms_around_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mushroom_noise: Res<MushroomNoise>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Mushroom>)>,
    mushroom_query: Query<&Transform, With<Mushroom>>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        let camera_pos = camera_transform.translation;
        let spawn_distance = 40.0;
        
        let mut has_nearby_mushrooms = false;
        for mushroom_transform in mushroom_query.iter() {
            let distance = mushroom_transform.translation.distance(camera_pos);
            if distance < spawn_distance {
                has_nearby_mushrooms = true;
                break;
            }
        }

        if !has_nearby_mushrooms {
            spawn_mushrooms_in_area(
                &mut commands,
                &mut meshes,
                &mut materials,
                &mushroom_noise,
                camera_pos.x - spawn_distance,
                camera_pos.x + spawn_distance,
                camera_pos.z - spawn_distance,
                camera_pos.z + spawn_distance,
            );
        }
    }
}

fn spawn_mushrooms_in_area(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    mushroom_noise: &MushroomNoise,
    min_x: f32,
    max_x: f32,
    min_z: f32,
    max_z: f32,
) {
    let mut rng = rand::thread_rng();
    let rarity = 0.05; // 5% chance per grid cell

    for x in (min_x as i32)..(max_x as i32) {
        for z in (min_z as i32)..(max_z as i32) {
            let world_x = x as f32;
            let world_z = z as f32;

            let noise_value = mushroom_noise.noise.get([world_x as f64 * 0.15, world_z as f64 * 0.15]) as f32;
            let should_spawn = noise_value > (1.0 - rarity) && rng.gen_range(0.0..1.0) < rarity;

            if should_spawn {
                let terrain_height = mushroom_noise.noise.get([world_x as f64 * 0.1, world_z as f64 * 0.1]) as f32 * 5.0;
                let glow = 0.5 + rng.gen_range(0.0..1.0) * 0.5;

                // Mushroom cap
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Sphere {
                            radius: 0.3,
                        }),
                        material: materials.add(StandardMaterial {
                            base_color: Color::srgb(0.8, 0.2, 0.8),
                            emissive: Color::srgb(0.8 * glow, 0.2 * glow, 0.8 * glow).into(),
                            ..default()
                        }),
                        transform: Transform::from_xyz(world_x, terrain_height + 0.5, world_z),
                        ..default()
                    },
                    Mushroom {
                        glow_intensity: glow,
                    },
                    Collectible,
                ));

                // Mushroom stem
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Cylinder {
                        radius: 0.05,
                        half_height: 0.3,
                    }),
                    material: materials.add(Color::srgb(0.9, 0.9, 0.9)),
                    transform: Transform::from_xyz(world_x, terrain_height + 0.2, world_z),
                    ..default()
                });
            }
        }
    }
}

fn check_mushroom_collection(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    _player_query: Query<&Transform, (With<super::player::Player>, Without<Mushroom>)>,
    camera_query: Query<&Transform, (With<Camera3d>, With<super::player::Player>)>,
    mushroom_query: Query<(Entity, &Transform, &Mushroom), With<Collectible>>,
    mut commands: Commands,
    mut ev_mushroom_collected: EventWriter<MushroomCollected>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        if let Ok(camera_transform) = camera_query.get_single() {
            let camera_pos = camera_transform.translation;
            let camera_forward = Vec3::from(camera_transform.forward());
            let collection_range = 3.0;
            let max_angle = 0.5; // ~30 degrees cone

            // Raycast-style collection: check mushrooms in front of camera
            for (entity, mushroom_transform, _mushroom) in mushroom_query.iter() {
                let to_mushroom = mushroom_transform.translation - camera_pos;
                let distance = to_mushroom.length();
                
                if distance < collection_range {
                    let direction = to_mushroom.normalize();
                    let dot = camera_forward.dot(direction);
                    
                    // Check if mushroom is in front of camera (within cone)
                    if dot > max_angle {
                        ev_mushroom_collected.send(MushroomCollected {
                            position: mushroom_transform.translation,
                        });
                        commands.entity(entity).despawn_recursive();
                        
                        // Camera shake will be added in handle_collection_effects
                    }
                }
            }
        }
    }
}

fn handle_collection_effects(
    mut mushroom_count: ResMut<MushroomCount>,
    mut ev_mushroom_collected: EventReader<MushroomCollected>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut camera_query: Query<Entity, (With<Camera3d>, With<super::player::Player>)>,
) {
    for event in ev_mushroom_collected.read() {
        mushroom_count.collected += 1;
        
        // Add camera shake on collection
        if let Ok(camera_entity) = camera_query.get_single_mut() {
            commands.entity(camera_entity).insert(super::controls::CameraShake {
                intensity: 0.1,
                duration: 0.2,
                timer: 0.2,
            });
        }

        // Spawn particle effect (simplified - just spawn some glowing particles)
        for i in 0..10 {
            let angle = (i as f32 / 10.0) * 2.0 * std::f32::consts::PI;
            let offset = Vec3::new(angle.cos(), 0.0, angle.sin()) * 0.5;
            
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Sphere {
                        radius: 0.1,
                    }),
                        material: materials.add(StandardMaterial {
                            base_color: Color::srgb(1.0, 0.8, 0.2),
                            emissive: Color::srgb(1.0, 0.8, 0.2).into(),
                            ..default()
                        }),
                    transform: Transform::from_translation(event.position + offset),
                    ..default()
                },
                Particle {
                    lifetime: 1.0,
                    velocity: offset.normalize() * 2.0,
                },
            ));
        }
    }
}

#[derive(Component)]
pub struct Particle {
    pub lifetime: f32,
    pub velocity: Vec3,
}


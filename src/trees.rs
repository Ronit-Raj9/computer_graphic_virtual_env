use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;

pub struct TreesPlugin;

impl Plugin for TreesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TreeConfig>()
            .init_resource::<TreeNoise>()
            .add_systems(Startup, spawn_initial_trees)
            .add_systems(Update, (spawn_trees_around_camera, animate_wind));
    }
}

#[derive(Resource)]
pub struct TreeConfig {
    pub density: f32,
    pub spawn_distance: f32,
    pub min_height: f32,
    pub max_height: f32,
}

impl Default for TreeConfig {
    fn default() -> Self {
        Self {
            density: 0.3,
            spawn_distance: 50.0,
            min_height: 2.0,
            max_height: 4.0,
        }
    }
}

#[derive(Resource)]
pub struct TreeNoise {
    pub noise: Perlin,
}

impl Default for TreeNoise {
    fn default() -> Self {
        Self {
            noise: Perlin::new(54321), // Different seed from terrain
        }
    }
}

#[derive(Component)]
pub struct Tree;

#[derive(Component)]
pub struct TreeCanopy;

#[derive(Component)]
pub struct WindAffected {
    pub base_rotation: Quat,
}

fn spawn_initial_trees(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<TreeConfig>,
    tree_noise: Res<TreeNoise>,
) {
    spawn_trees_in_area(
        &mut commands,
        &mut meshes,
        &mut materials,
        &config,
        &tree_noise,
        -50.0,
        50.0,
        -50.0,
        50.0,
    );
}

fn spawn_trees_around_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<TreeConfig>,
    tree_noise: Res<TreeNoise>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Tree>)>,
    tree_query: Query<&Transform, With<Tree>>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        let camera_pos = camera_transform.translation;
        
        // Check if we need to spawn trees in new areas
        let mut has_nearby_trees = false;
        for tree_transform in tree_query.iter() {
            let distance = tree_transform.translation.distance(camera_pos);
            if distance < config.spawn_distance {
                has_nearby_trees = true;
                break;
            }
        }

        if !has_nearby_trees {
            let spawn_radius = config.spawn_distance;
            spawn_trees_in_area(
                &mut commands,
                &mut meshes,
                &mut materials,
                &config,
                &tree_noise,
                camera_pos.x - spawn_radius,
                camera_pos.x + spawn_radius,
                camera_pos.z - spawn_radius,
                camera_pos.z + spawn_radius,
            );
        }
    }
}

fn spawn_trees_in_area(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    config: &TreeConfig,
    tree_noise: &TreeNoise,
    min_x: f32,
    max_x: f32,
    min_z: f32,
    max_z: f32,
) {
    let mut rng = rand::thread_rng();

    for x in (min_x as i32)..(max_x as i32) {
        for z in (min_z as i32)..(max_z as i32) {
            let world_x = x as f32;
            let world_z = z as f32;

            // Use noise to determine if tree should spawn here
            let noise_value = tree_noise.noise.get([world_x as f64 * 0.1, world_z as f64 * 0.1]) as f32;
            let should_spawn = noise_value > (1.0 - config.density);

            if should_spawn && rng.gen_range(0.0..1.0) < config.density {
                // Sample terrain height using same noise as terrain generation (seed 12345, scale 0.1)
                use noise::{NoiseFn, Perlin};
                let terrain_noise = Perlin::new(12345); // Same seed as terrain
                let terrain_height = terrain_noise.get([world_x as f64 * 0.1, world_z as f64 * 0.1]) as f32 * 5.0;
                
                // Enhanced variety to tree sizes
                let size_variation = rng.gen_range(0.7..1.4);
                let tree_height = rng.gen_range(config.min_height..config.max_height) * size_variation;
                let trunk_radius = tree_height * 0.06;
                let canopy_radius = tree_height * 0.45 + rng.gen_range(-0.2..0.3);

                // More natural trunk color variation with richer browns
                let trunk_base = rng.gen_range(0.22..0.32);
                let trunk_color_r = (trunk_base + rng.gen_range(-0.06..0.06) as f32).clamp(0.15, 0.4);
                let trunk_color_g = (trunk_base * 0.65 + rng.gen_range(-0.04..0.04) as f32).clamp(0.1, 0.3);
                let trunk_color_b = (trunk_base * 0.35 + rng.gen_range(-0.03..0.03) as f32).clamp(0.05, 0.2);

                // More vibrant and natural canopy colors with better green variation
                let canopy_green = rng.gen_range(0.35..0.6);
                let canopy_color_r = rng.gen_range(0.04..0.1);
                let canopy_color_g = canopy_green;
                let canopy_color_b = rng.gen_range(0.04..0.1);

                // Spawn trunk with enhanced material and slight tapering
                let trunk_taper = 0.85; // Slight taper for more natural look
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Cylinder {
                            radius: trunk_radius,
                            half_height: tree_height / 2.0,
                        }),
                        material: materials.add(StandardMaterial {
                            base_color: Color::srgb(trunk_color_r, trunk_color_g, trunk_color_b),
                            metallic: 0.0,
                            perceptual_roughness: 0.88,
                            reflectance: 0.015,
                            ..default()
                        }),
                        transform: Transform::from_xyz(world_x, terrain_height + tree_height / 2.0, world_z)
                            .with_scale(Vec3::new(1.0, 1.0, trunk_taper)),
                        ..default()
                    },
                    Tree,
                ));

                // Spawn multiple canopy layers for more natural, lush look
                let canopy_y = terrain_height + tree_height;
                let canopy_offset_x = rng.gen_range(-0.2..0.2);
                let canopy_offset_z = rng.gen_range(-0.2..0.2);
                
                // Main canopy with better material
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Sphere {
                            radius: canopy_radius,
                        }),
                        material: materials.add(StandardMaterial {
                            base_color: Color::srgb(canopy_color_r, canopy_color_g, canopy_color_b),
                            metallic: 0.0,
                            perceptual_roughness: 0.72,
                            reflectance: 0.08,
                            ..default()
                        }),
                        transform: Transform::from_xyz(
                            world_x + canopy_offset_x, 
                            canopy_y, 
                            world_z + canopy_offset_z
                        )
                        .with_scale(Vec3::new(
                            1.0 + rng.gen_range(-0.1..0.1),
                            1.0 + rng.gen_range(-0.1..0.1),
                            1.0 + rng.gen_range(-0.1..0.1),
                        )),
                        ..default()
                    },
                    TreeCanopy,
                    WindAffected {
                        base_rotation: Quat::IDENTITY,
                    },
                ));
                
                // Secondary smaller canopy layer for depth (40% chance for more variety)
                if rng.gen_range(0.0..1.0) < 0.4 {
                    let secondary_radius = canopy_radius * 0.55;
                    let secondary_offset_x = rng.gen_range(-0.25..0.25);
                    let secondary_offset_z = rng.gen_range(-0.25..0.25);
                    let secondary_y = canopy_y + rng.gen_range(-0.4..0.6);
                    
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Sphere {
                                radius: secondary_radius,
                            }),
                            material: materials.add(StandardMaterial {
                                base_color: Color::srgb(
                                    (canopy_color_r * 0.85).clamp(0.0, 1.0),
                                    (canopy_color_g * 1.15).clamp(0.0, 1.0),
                                    (canopy_color_b * 0.85).clamp(0.0, 1.0),
                                ),
                                metallic: 0.0,
                                perceptual_roughness: 0.68,
                                reflectance: 0.08,
                                ..default()
                            }),
                            transform: Transform::from_xyz(
                                world_x + secondary_offset_x,
                                secondary_y,
                                world_z + secondary_offset_z
                            )
                            .with_scale(Vec3::new(
                                1.0 + rng.gen_range(-0.15..0.15),
                                1.0 + rng.gen_range(-0.15..0.15),
                                1.0 + rng.gen_range(-0.15..0.15),
                            )),
                            ..default()
                        },
                        TreeCanopy,
                        WindAffected {
                            base_rotation: Quat::IDENTITY,
                        },
                    ));
                }
                
                // Tertiary small canopy layer for extra depth (20% chance)
                if rng.gen_range(0.0..1.0) < 0.2 {
                    let tertiary_radius = canopy_radius * 0.35;
                    let tertiary_offset_x = rng.gen_range(-0.3..0.3);
                    let tertiary_offset_z = rng.gen_range(-0.3..0.3);
                    let tertiary_y = canopy_y + rng.gen_range(-0.5..0.7);
                    
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Sphere {
                                radius: tertiary_radius,
                            }),
                            material: materials.add(StandardMaterial {
                                base_color: Color::srgb(
                                    (canopy_color_r * 0.75).clamp(0.0, 1.0),
                                    (canopy_color_g * 1.2).clamp(0.0, 1.0),
                                    (canopy_color_b * 0.75).clamp(0.0, 1.0),
                                ),
                                metallic: 0.0,
                                perceptual_roughness: 0.65,
                                reflectance: 0.08,
                                ..default()
                            }),
                            transform: Transform::from_xyz(
                                world_x + tertiary_offset_x,
                                tertiary_y,
                                world_z + tertiary_offset_z
                            ),
                            ..default()
                        },
                        TreeCanopy,
                        WindAffected {
                            base_rotation: Quat::IDENTITY,
                        },
                    ));
                }
            }
        }
    }
}

fn animate_wind(time: Res<Time>, mut query: Query<&mut Transform, With<WindAffected>>) {
    let wind_strength = 0.1;
    let wind_speed = 2.0;
    let wind_angle = (time.elapsed_seconds() * wind_speed).sin() * wind_strength;

    for mut transform in query.iter_mut() {
        // Simple wind animation - rotate slightly
        transform.rotation = Quat::from_rotation_z(wind_angle * 0.1) * Quat::from_rotation_x(wind_angle * 0.05);
    }
}


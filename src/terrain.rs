use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerrainConfig>()
            .init_resource::<ChunkManager>()
            .add_systems(Startup, setup_terrain)
            .add_systems(Update, update_chunks);
    }
}

#[derive(Resource)]
pub struct TerrainConfig {
    pub chunk_size: f32,
    pub render_distance: i32,
    pub height_scale: f32,
    pub noise_scale: f64,
}

impl Default for TerrainConfig {
    fn default() -> Self {
        Self {
            chunk_size: 32.0,
            render_distance: 3,
            height_scale: 5.0,
            noise_scale: 0.1,
        }
    }
}

#[derive(Resource)]
pub struct ChunkManager {
    pub loaded_chunks: std::collections::HashSet<(i32, i32)>,
    pub noise: Perlin,
}

impl Default for ChunkManager {
    fn default() -> Self {
        Self {
            loaded_chunks: std::collections::HashSet::new(),
            noise: Perlin::new(12345), // Fixed seed for reproducibility
        }
    }
}

#[derive(Component)]
pub struct TerrainChunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
}

fn setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<TerrainConfig>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    // Generate initial chunks around origin
    for x in -config.render_distance..=config.render_distance {
        for z in -config.render_distance..=config.render_distance {
            spawn_chunk(
                &mut commands,
                &mut meshes,
                &mut materials,
                &config,
                &mut chunk_manager,
                x,
                z,
            );
        }
    }
}

fn update_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<TerrainConfig>,
    mut chunk_manager: ResMut<ChunkManager>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<TerrainChunk>)>,
    chunk_query: Query<(Entity, &TerrainChunk)>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        let camera_chunk_x = (camera_transform.translation.x / config.chunk_size).floor() as i32;
        let camera_chunk_z = (camera_transform.translation.z / config.chunk_size).floor() as i32;

        // Unload distant chunks
        let mut chunks_to_remove = Vec::new();
        for (entity, chunk) in chunk_query.iter() {
            let dx = (chunk.chunk_x - camera_chunk_x).abs();
            let dz = (chunk.chunk_z - camera_chunk_z).abs();
            if dx > config.render_distance || dz > config.render_distance {
                chunks_to_remove.push(entity);
                chunk_manager.loaded_chunks.remove(&(chunk.chunk_x, chunk.chunk_z));
            }
        }
        for entity in chunks_to_remove {
            commands.entity(entity).despawn_recursive();
        }

        // Load new chunks
        for x in (camera_chunk_x - config.render_distance)..=(camera_chunk_x + config.render_distance) {
            for z in (camera_chunk_z - config.render_distance)..=(camera_chunk_z + config.render_distance) {
                if !chunk_manager.loaded_chunks.contains(&(x, z)) {
                    spawn_chunk(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        &config,
                        &mut chunk_manager,
                        x,
                        z,
                    );
                }
            }
        }
    }
}

fn spawn_chunk(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    config: &TerrainConfig,
    chunk_manager: &mut ChunkManager,
    chunk_x: i32,
    chunk_z: i32,
) {
    let chunk_world_x = chunk_x as f32 * config.chunk_size;
    let chunk_world_z = chunk_z as f32 * config.chunk_size;

    // Create terrain mesh using Bevy's mesh builder
    let resolution = 32;
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    // Generate vertices
    for z in 0..=resolution {
        for x in 0..=resolution {
            let local_x = x as f32 / resolution as f32;
            let local_z = z as f32 / resolution as f32;
            
            let world_x = chunk_world_x + local_x * config.chunk_size;
            let world_z = chunk_world_z + local_z * config.chunk_size;
            
            // Sample noise for height
            let height = chunk_manager.noise.get([world_x as f64 * config.noise_scale, world_z as f64 * config.noise_scale]) as f32 * config.height_scale;
            
            positions.push([world_x, height, world_z]);
            normals.push([0.0, 1.0, 0.0]); // Simplified normals
            uvs.push([local_x, local_z]);
        }
    }

    // Generate indices
    for z in 0..resolution {
        for x in 0..resolution {
            let i = (z * (resolution + 1) + x) as u32;
            indices.extend_from_slice(&[
                i,
                i + resolution as u32 + 1,
                i + 1,
                i + 1,
                i + resolution as u32 + 1,
                i + resolution as u32 + 2,
            ]);
        }
    }

    // Calculate proper normals for better lighting
    let mut calculated_normals = vec![[0.0, 1.0, 0.0]; positions.len()];
    for i in 0..indices.len() / 3 {
        let i0 = indices[i * 3] as usize;
        let i1 = indices[i * 3 + 1] as usize;
        let i2 = indices[i * 3 + 2] as usize;
        
        let v0 = Vec3::from_array(positions[i0]);
        let v1 = Vec3::from_array(positions[i1]);
        let v2 = Vec3::from_array(positions[i2]);
        
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let normal = edge1.cross(edge2).normalize();
        
        calculated_normals[i0] = [calculated_normals[i0][0] + normal.x, calculated_normals[i0][1] + normal.y, calculated_normals[i0][2] + normal.z];
        calculated_normals[i1] = [calculated_normals[i1][0] + normal.x, calculated_normals[i1][1] + normal.y, calculated_normals[i1][2] + normal.z];
        calculated_normals[i2] = [calculated_normals[i2][0] + normal.x, calculated_normals[i2][1] + normal.y, calculated_normals[i2][2] + normal.z];
    }
    
    // Normalize normals
    for normal in &mut calculated_normals {
        let n = Vec3::from_array(*normal).normalize();
        *normal = [n.x, n.y, n.z];
    }

    // Determine material based on average height (before moving positions)
    let avg_height = positions.iter().map(|p| p[1]).sum::<f32>() / positions.len() as f32;
    let height_factor = (avg_height / config.height_scale).clamp(0.0, 1.0);
    
    // Gradient-based material color blending
    let grass_light = Color::srgb(0.3, 0.65, 0.25);
    
    let material_color = if height_factor > 0.4 {
        // Rock - blend based on height
        let blend = ((height_factor - 0.4) / 0.6).min(1.0);
        Color::srgb(
            0.5 + blend * 0.1,
            0.5 + blend * 0.05,
            0.5 + blend * 0.0,
        )
    } else if height_factor > 0.15 {
        // Dark grass to light grass transition
        let blend = ((height_factor - 0.15) / 0.25).min(1.0);
        Color::srgb(
            0.15 + blend * 0.15,
            0.4 + blend * 0.25,
            0.15 + blend * 0.1,
        )
    } else {
        // Light grass
        grass_light
    };

    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::MAIN_WORLD | bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, calculated_normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: material_color,
                metallic: 0.0,
                perceptual_roughness: 0.95,
                reflectance: 0.02,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        TerrainChunk {
            chunk_x,
            chunk_z,
        },
    ));

    chunk_manager.loaded_chunks.insert((chunk_x, chunk_z));
}


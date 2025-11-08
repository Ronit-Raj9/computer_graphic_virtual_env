use bevy::prelude::*;

pub struct ControlImprovementsPlugin;

impl Plugin for ControlImprovementsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            apply_movement_smoothing,
            handle_jump,
            handle_sprint,
            update_camera_fov,
            apply_head_bob,
            apply_camera_shake,
        ));
    }
}

#[derive(Component)]
pub struct Velocity {
    pub linear: Vec3,
    pub damping: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            linear: Vec3::ZERO,
            damping: 0.9,
        }
    }
}

#[derive(Component)]
pub struct CameraShake {
    pub intensity: f32,
    pub duration: f32,
    pub timer: f32,
}

// Apply velocity to transform (movement is calculated in player.rs)
fn apply_movement_smoothing(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut Velocity), With<super::player::Player>>,
) {
    for (mut transform, mut velocity) in player_query.iter_mut() {
        // Apply velocity to position
        transform.translation += velocity.linear * time.delta_seconds();
        
        // Reset horizontal velocity if very small
        if velocity.linear.x.abs() < 0.01 {
            velocity.linear.x = 0.0;
        }
        if velocity.linear.z.abs() < 0.01 {
            velocity.linear.z = 0.0;
        }
    }
}

// Handle jumping mechanics (gravity is handled in player.rs)
fn handle_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Velocity, With<super::player::Player>>,
    settings: Res<super::player::PlayerSettings>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for mut velocity in player_query.iter_mut() {
            // Simple jump - add upward velocity
            // In a full implementation, you'd check if on ground first
            // For now, allow jumping anytime
            velocity.linear.y = settings.jump_force;
        }
    }
}

// Handle sprinting (handled in player movement system, this is a placeholder)
fn handle_sprint(
    _keyboard_input: Res<ButtonInput<KeyCode>>,
    _player_query: Query<&super::player::Player>,
) {
    // Sprint is handled in player.rs player_movement system
}

// Update camera FOV based on sprint state
fn update_camera_fov(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Projection, With<Camera3d>>,
    settings: Res<super::settings::GameSettings>,
) {
    let is_sprinting = keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight);
    
    for mut projection in camera_query.iter_mut() {
        if let Projection::Perspective(ref mut perspective) = *projection {
            let target_fov = if is_sprinting {
                settings.fov * 1.2 // Widen FOV when sprinting
            } else {
                settings.fov
            };
            
            // Smoothly interpolate FOV
            perspective.fov = perspective.fov.lerp(target_fov.to_radians(), 0.1);
        }
    }
}

// Apply head bob based on movement
fn apply_head_bob(
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, With<super::player::Player>)>,
    velocity_query: Query<&Velocity, (With<super::player::Player>, Without<Camera3d>)>,
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        if let Ok(velocity) = velocity_query.get_single() {
            let speed = (velocity.linear.x * velocity.linear.x + velocity.linear.z * velocity.linear.z).sqrt();
            if speed > 0.1 {
                let bob_amount = 0.05;
                let bob_frequency = 10.0;
                let bob_offset = (time.elapsed_seconds() * bob_frequency).sin() * bob_amount;
                // Store base Y and apply bob
                camera_transform.translation.y += bob_offset * time.delta_seconds() * 5.0;
            }
        }
    }
}

// Apply camera shake (e.g., on mushroom collection)
fn apply_camera_shake(
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, With<super::player::Player>)>,
    mut shake_query: Query<(&mut CameraShake, Entity)>,
    mut commands: Commands,
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        for (mut shake, entity) in shake_query.iter_mut() {
            shake.timer -= time.delta_seconds();
            
            if shake.timer > 0.0 {
                let intensity = shake.intensity * (shake.timer / shake.duration);
                let offset = Vec3::new(
                    (time.elapsed_seconds() * 20.0).sin() * intensity * time.delta_seconds(),
                    (time.elapsed_seconds() * 15.0).cos() * intensity * time.delta_seconds(),
                    0.0,
                );
                camera_transform.translation += offset;
            } else {
                // Remove shake component when done
                commands.entity(entity).remove::<CameraShake>();
            }
        }
    }
}


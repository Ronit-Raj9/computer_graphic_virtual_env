use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::controls;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerSettings>()
            .add_systems(Startup, setup_player)
            .add_systems(Update, (player_movement, mouse_look))
            .add_systems(Update, handle_escape_key);
    }
}

#[derive(Resource)]
pub struct PlayerSettings {
    pub move_speed: f32,
    pub mouse_sensitivity: f32,
    pub jump_force: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            move_speed: 5.0,
            mouse_sensitivity: 0.001,
            jump_force: 5.0,
        }
    }
}

#[derive(Component)]
pub struct Player {
    pub velocity: Vec3,
    pub on_ground: bool,
}

fn setup_player(mut commands: Commands) {
    // Spawn camera as player - start higher above ground
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::new(0.0, 4.0, 5.0), Vec3::Y),
            ..default()
        },
        Player {
            velocity: Vec3::ZERO,
            on_ground: true,
        },
        controls::Velocity {
            linear: Vec3::ZERO,
            damping: 0.9,
        },
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut controls::Velocity), With<Player>>,
    settings: Res<PlayerSettings>,
    game_settings: Res<crate::settings::GameSettings>,
) {
    if let Ok((mut transform, mut velocity)) = player_query.get_single_mut() {
        let mut movement_direction = Vec3::ZERO;

        // Forward/backward
        if keyboard_input.pressed(KeyCode::KeyW) {
            movement_direction += Vec3::from(transform.forward());
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            movement_direction -= Vec3::from(transform.forward());
        }

        // Left/right
        if keyboard_input.pressed(KeyCode::KeyA) {
            movement_direction -= Vec3::from(transform.right());
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            movement_direction += Vec3::from(transform.right());
        }

        // Normalize direction
        if movement_direction.length() > 0.0 {
            movement_direction = movement_direction.normalize();
            
            // Check for sprint
            let is_sprinting = keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight);
            let speed_multiplier = if is_sprinting { game_settings.sprint_speed_multiplier } else { 1.0 };
            let target_speed = settings.move_speed * speed_multiplier;
            
            // Calculate target velocity
            let target_velocity = movement_direction * target_speed;
            
            // Smoothly accelerate towards target velocity
            let acceleration = 20.0;
            velocity.linear.x = velocity.linear.x.lerp(target_velocity.x, acceleration * time.delta_seconds());
            velocity.linear.z = velocity.linear.z.lerp(target_velocity.z, acceleration * time.delta_seconds());
        } else {
            // Decelerate when not moving
            velocity.linear.x *= velocity.damping;
            velocity.linear.z *= velocity.damping;
        }

        // Ground following - query terrain height using noise (same as terrain generation)
        use noise::{NoiseFn, Perlin};
        let noise = Perlin::new(12345); // Same seed as terrain
        let terrain_height = noise.get([transform.translation.x as f64 * 0.1, transform.translation.z as f64 * 0.1]) as f32 * 5.0;
        let min_height = terrain_height + 2.5; // Keep player 2.5 units above terrain
        
        // Apply gravity if above ground
        if transform.translation.y > min_height {
            velocity.linear.y -= 9.8 * time.delta_seconds();
        }
        
        if transform.translation.y < min_height {
            transform.translation.y = min_height;
            velocity.linear.y = 0.0; // Stop falling when on ground
        }
    }
}

fn mouse_look(
    mut player_query: Query<&mut Transform, With<Player>>,
    mut mouse_motion_events: EventReader<bevy::input::mouse::MouseMotion>,
    _settings: Res<PlayerSettings>,
    game_settings: Res<crate::settings::GameSettings>,
) {
    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        delta += event.delta;
    }

        if delta.length() > 0.0 {
        if let Ok(mut transform) = player_query.get_single_mut() {
            // Use game settings sensitivity if available, otherwise fall back to player settings
            let sensitivity = game_settings.mouse_sensitivity;
            
            // Horizontal rotation (yaw)
            let yaw = -delta.x * sensitivity;
            transform.rotate_y(yaw);

            // Vertical rotation (pitch) - limited to prevent flipping (-89 to 89 degrees)
            let pitch = -delta.y * sensitivity;
            let (yaw_angle, current_pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            let new_pitch = (current_pitch + pitch).clamp(-1.4, 1.4);
            
            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw_angle, new_pitch, 0.0);
        }
    }
}

fn handle_escape_key(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if let Ok(mut window) = windows.get_single_mut() {
            let is_locked = matches!(window.cursor.grab_mode, bevy::window::CursorGrabMode::Locked);
            window.cursor.visible = is_locked;
            window.cursor.grab_mode = if is_locked {
                bevy::window::CursorGrabMode::None
            } else {
                bevy::window::CursorGrabMode::Locked
            };
        }
    }
}


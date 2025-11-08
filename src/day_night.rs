use bevy::prelude::*;
use std::f32::consts::PI;

pub struct DayNightPlugin;

impl Plugin for DayNightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeOfDay>()
            .add_systems(Startup, setup_lighting)
            .add_systems(Update, (update_day_night_cycle, update_sky_color));
    }
}

#[derive(Resource)]
pub struct TimeOfDay {
    pub time: f32, // 0.0 to 24.0
    pub speed: f32, // Time multiplier
}

impl Default for TimeOfDay {
    fn default() -> Self {
        Self {
            time: 12.0, // Start at noon
            speed: 0.1, // Slow time progression
        }
    }
}

#[derive(Component)]
pub struct SunLight;

fn setup_lighting(mut commands: Commands) {
    // Spawn directional light (sun)
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::srgb(1.0, 0.95, 0.8),
                illuminance: 10000.0,
                shadows_enabled: true,
                shadow_depth_bias: 0.02,
                shadow_normal_bias: 0.5,
                ..default()
            },
            transform: Transform::from_rotation(Quat::from_rotation_x(-PI / 4.0)),
            ..default()
        },
        SunLight,
    ));
}

fn update_day_night_cycle(
    time: Res<Time>,
    mut time_of_day: ResMut<TimeOfDay>,
    mut light_query: Query<&mut Transform, With<SunLight>>,
) {
    // Update time
    time_of_day.time += time.delta_seconds() * time_of_day.speed;
    if time_of_day.time >= 24.0 {
        time_of_day.time -= 24.0;
    }

    // Calculate sun position (0 = midnight, 12 = noon)
    let sun_angle = (time_of_day.time / 24.0) * 2.0 * PI - PI / 2.0;
    
    // Update sun rotation
    if let Ok(mut sun_transform) = light_query.get_single_mut() {
        sun_transform.rotation = Quat::from_rotation_x(sun_angle);
    }
}

fn update_sky_color(
    time_of_day: Res<TimeOfDay>,
    mut clear_color: ResMut<ClearColor>,
    mut light_query: Query<&mut DirectionalLight, With<SunLight>>,
) {
    let t = time_of_day.time;
    
    // Calculate light intensity and color based on time
    let (intensity, color, sky_color) = if t >= 6.0 && t < 8.0 {
        // Dawn
        let factor = (t - 6.0) / 2.0;
        (
            5000.0 + factor * 5000.0,
            Color::srgb(1.0, 0.7 + factor * 0.25, 0.5 + factor * 0.3),
            Color::srgb(0.3 + factor * 0.2, 0.4 + factor * 0.3, 0.5 + factor * 0.2),
        )
    } else if t >= 8.0 && t < 18.0 {
        // Day
        (
            10000.0,
            Color::srgb(1.0, 0.95, 0.8),
            Color::srgb(0.5, 0.7, 1.0),
        )
    } else if t >= 18.0 && t < 20.0 {
        // Dusk
        let factor = (t - 18.0) / 2.0;
        (
            10000.0 - factor * 8000.0,
            Color::srgb(1.0, 0.95 - factor * 0.65, 0.8 - factor * 0.5),
            Color::srgb(0.5 - factor * 0.45, 0.7 - factor * 0.65, 1.0 - factor * 0.8),
        )
    } else {
        // Night
        (
            2000.0,
            Color::srgb(0.3, 0.3, 0.8),
            Color::srgb(0.05, 0.05, 0.2),
        )
    };

    clear_color.0 = sky_color;
    
    if let Ok(mut light) = light_query.get_single_mut() {
        light.color = color;
        light.illuminance = intensity;
    }
}


use bevy::prelude::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSettings>();
    }
}

#[derive(Resource)]
pub struct GameSettings {
    pub mouse_sensitivity: f32,
    pub fog_density: f32,
    pub render_distance: i32,
    pub enable_shadows: bool,
    pub enable_fog: bool,
    pub fov: f32,
    pub sprint_speed_multiplier: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 0.002,
            fog_density: 1.0,
            render_distance: 3,
            enable_shadows: true,
            enable_fog: true,
            fov: 75.0,
            sprint_speed_multiplier: 2.0,
        }
    }
}


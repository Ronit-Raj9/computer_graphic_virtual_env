use bevy::prelude::*;

pub struct VisualEnhancementsPlugin;

impl Plugin for VisualEnhancementsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            pulse_mushroom_glow,
            update_grass_animation,
        ));
    }
}

// Pulse mushroom glow based on time and day/night cycle
fn pulse_mushroom_glow(
    time: Res<Time>,
    time_of_day: Option<Res<super::day_night::TimeOfDay>>,
    mushroom_query: Query<&Handle<StandardMaterial>, With<super::interactivity::Mushroom>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let pulse_speed = 2.0;
    let base_pulse = (time.elapsed_seconds() * pulse_speed).sin() * 0.3 + 0.7;
    
    // Increase glow at night
    let night_multiplier = if let Some(time) = time_of_day {
        let t = time.time;
        if t >= 20.0 || t < 6.0 {
            1.5 // Night - brighter glow
        } else if t >= 18.0 || t < 8.0 {
            1.2 // Dusk/dawn - moderate glow
        } else {
            1.0 // Day - normal glow
        }
    } else {
        1.0
    };
    
    let glow_intensity = base_pulse * night_multiplier;
    
    for material_handle in mushroom_query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            // Get base emissive color and multiply intensity
            let base_emissive = material.emissive;
            // LinearRgba can be multiplied by f32 directly
            material.emissive = base_emissive * glow_intensity;
        }
    }
}

// Animate grass/foliage (placeholder for future grass system)
fn update_grass_animation(
    _time: Res<Time>,
    // This will be used when grass system is implemented
) {
    // Placeholder for grass animation
}


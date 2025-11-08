use bevy::prelude::*;

pub struct FogPlugin;

impl Plugin for FogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_fog)
            .add_systems(Update, update_fog);
    }
}

fn setup_fog(mut camera_query: Query<&mut FogSettings, Added<Camera3d>>) {
    for mut fog in camera_query.iter_mut() {
        *fog = FogSettings {
            color: Color::srgb(0.7, 0.8, 0.9),
            falloff: FogFalloff::Linear {
                start: 30.0,
                end: 100.0,
            },
            ..default()
        };
    }
}

fn update_fog(
    mut camera_query: Query<&mut FogSettings, With<Camera3d>>,
    time_of_day: Option<Res<super::day_night::TimeOfDay>>,
) {
    if let Some(ref time) = time_of_day {
        for mut fog in camera_query.iter_mut() {
            // Adjust fog based on time of day
            let t = time.time;
            
            if t >= 20.0 || t < 6.0 {
                // Night - more fog
                fog.color = Color::srgb(0.05, 0.05, 0.15);
                fog.falloff = FogFalloff::Linear {
                    start: 10.0,
                    end: 50.0,
                };
            } else if t >= 6.0 && t < 8.0 {
                // Dawn - moderate fog
                let factor = (t - 6.0) / 2.0;
                fog.color = Color::srgb(0.3 + factor * 0.2, 0.4 + factor * 0.3, 0.5 + factor * 0.2);
                fog.falloff = FogFalloff::Linear {
                    start: 15.0,
                    end: 60.0,
                };
            } else {
                // Day - light fog
                fog.color = Color::srgb(0.7, 0.8, 0.9);
                fog.falloff = FogFalloff::Linear {
                    start: 30.0,
                    end: 100.0,
                };
            }
        }
    }
}


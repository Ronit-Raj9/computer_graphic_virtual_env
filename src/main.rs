mod terrain;
mod trees;
mod player;
mod day_night;
mod interactivity;
mod fog;
mod ui;
mod visuals;
mod controls;
mod settings;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Procedural Forest Explorer".into(),
                resolution: (1280.0, 720.0).into(),
                cursor: bevy::window::Cursor {
                    visible: false,
                    grab_mode: bevy::window::CursorGrabMode::Locked,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            terrain::TerrainPlugin,
            trees::TreesPlugin,
            player::PlayerPlugin,
            day_night::DayNightPlugin,
            interactivity::InteractivityPlugin,
            fog::FogPlugin,
            ui::UIPlugin,
            visuals::VisualEnhancementsPlugin,
            controls::ControlImprovementsPlugin,
            settings::SettingsPlugin,
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.5, 0.7, 1.0)))
        .add_systems(Update, update_particles)
        .run();
}

fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particle_query: Query<(Entity, &mut Transform, &mut interactivity::Particle)>,
) {
    for (entity, mut transform, mut particle) in particle_query.iter_mut() {
        particle.lifetime -= time.delta_seconds();
        transform.translation += particle.velocity * time.delta_seconds();
        
        // Fade out
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_ui);
    }
}

#[derive(Component)]
struct UIText;

#[derive(Component)]
struct UIPanel;

fn setup_ui(mut commands: Commands) {
    // Create main container with modern design
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(15.0),
                left: Val::Px(15.0),
                padding: UiRect::all(Val::Px(25.0)),
                flex_direction: FlexDirection::Column,
                min_width: Val::Px(320.0),
                ..default()
            },
            // Modern dark blue-gray panel with better transparency
            background_color: BackgroundColor(Color::srgba(0.08, 0.12, 0.18, 0.92)),
            ..default()
        },
        UIPanel,
    ));

    // Create title text with better styling
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 20.0,
                color: Color::srgb(0.98, 0.98, 0.98),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(40.0),
            left: Val::Px(40.0),
            ..default()
        }),
        UIText,
    ));
}

fn update_ui(
    mut ui_query: Query<&mut Text, With<UIText>>,
    time_of_day: Option<Res<super::day_night::TimeOfDay>>,
    mushroom_count: Option<Res<super::interactivity::MushroomCount>>,
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
) {
    if let Ok(mut text) = ui_query.get_single_mut() {
        let mut info = String::from("🌲 FOREST EXPLORER 🌲\n");
        info.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n");
        
        // Stats section with modern formatting
        info.push_str("📊 GAME STATS\n");
        info.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        
        // Display FPS with better formatting
        if let Some(fps) = diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                let fps_status = if value >= 55.0 { 
                    "⚡ EXCELLENT" 
                } else if value >= 30.0 { 
                    "✓ GOOD" 
                } else { 
                    "⚠ LOW" 
                };
                info.push_str(&format!("  {} FPS: {:.0}\n\n", fps_status, value));
            }
        }

        // Time display with better formatting
        if let Some(time) = time_of_day {
            let hours = time.time.floor() as u32;
            let minutes = ((time.time - hours as f32) * 60.0) as u32;
            let time_icon = if time.time >= 6.0 && time.time < 18.0 { "☀️ DAY" } else { "🌙 NIGHT" };
            info.push_str(&format!("  {} {:02}:{:02}\n\n", time_icon, hours, minutes));
        }

        // Mushroom count with better formatting
        if let Some(count) = mushroom_count {
            info.push_str(&format!("  🍄 COLLECTED: {}\n\n", count.collected));
        }
        
        // Controls section with modern layout
        info.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        info.push_str("🎮 CONTROLS\n");
        info.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        info.push_str("  [WASD]     Movement\n");
        info.push_str("  [Shift]    Sprint Mode\n");
        info.push_str("  [Space]    Jump\n");
        info.push_str("  [Mouse]    Camera Look\n");
        info.push_str("  [E]        Interact\n");
        info.push_str("  [ESC]      Menu\n");

        text.sections[0].value = info;
    }
}


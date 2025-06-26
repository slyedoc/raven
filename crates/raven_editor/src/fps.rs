use bevy::{
    color::palettes::tailwind,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<FrameTimeDiagnosticsPlugin>() {
            app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        }

        // app.add_plugins(bevy::dev_tools::fps_overlay::FpsOverlayPlugin {
        //     config: bevy::dev_tools::fps_overlay::FpsOverlayConfig {
        //         text_config: TextFont {
        //             font_size: 42.0,
        //             ..default()
        //         },
        //         text_color: bevy::color::palettes::css::WHITE.into(),
        //         enabled: true,
        //     },
        // });

        app.add_systems(Startup, setup)
            .add_systems(Update, update_fps_text);
    }
}

#[derive(Component)]
struct FpsText;

fn setup(mut commands: Commands) {
    commands
        .spawn((
            FpsText,
            Name::new("FPS"),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(200.0),
                ..default()
            },
            Text::new("FPS: "),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(tailwind::GRAY_400.into()),
        ))
        .with_children(|b| {
            b.spawn((TextSpan::new("0"), TextColor(tailwind::GRAY_400.into())));
        });
}

fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    query: Query<Entity, With<FpsText>>,
    mut writer: Text2dWriter,
) {
    for e in &query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                *writer.text(e, 1) = format!("{value:4.0}");
                *writer.color(e, 1) = match value {
                    0.0..=30.0 => tailwind::RED_500.into(),
                    30.0..=60.0 => tailwind::YELLOW_500.into(),
                    _ => tailwind::GREEN_500.into(),
                };
            }
        }
    }
}

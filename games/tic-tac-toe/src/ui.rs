use bevy::prelude::*;

pub const MARGIN: Val = Val::Px(12.);

pub fn create_button() -> (Node, BorderColor, BorderRadius, BackgroundColor) {
    (
        Node {
            width: Val::Px(250.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            margin: UiRect::all(MARGIN),
            row_gap: MARGIN,
            ..default()
        },
        BorderColor(Color::BLACK),
        BorderRadius::MAX,
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
    )
}

pub fn create_label(text: &str, font: Handle<Font>) -> (Text, TextFont, TextColor) {
    (
        Text::new(text),
        TextFont {
            font,
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
    )
}

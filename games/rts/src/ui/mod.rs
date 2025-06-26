
use bevy::asset::weak_handle;

use crate::prelude::*;

pub const MARGIN: Val = Val::Px(12.);
pub const OUTLINE_WITHD: Val = Val::Px(2.);

pub const UI_BG: Color = Color::Srgba(tailwind::GRAY_900);
pub const UI_OUTLINE: Color = Color::Srgba(tailwind::GRAY_200);
pub const UI_BTN_BG: Color = Color::Srgba(tailwind::BLUE_900);
pub const LABEL_COLOR: Color = Color::Srgba(tailwind::BLUE_500);

pub const SELECT_MATERIAL_HOVER: Handle<StandardMaterial> = weak_handle!("0456681c-74fa-45eb-906c-6fbbaae462ed");
pub const SELECT_MATERIAL_PRESSED: Handle<StandardMaterial> = weak_handle!("3c91ab0e-e27c-4e1b-8e25-4ac83e68b6c3");

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<UiAssets>()
        .add_systems(Startup, setup_ui);
    //.add_observer(healthbar::on_add_health)
    //.add_systems(Update, healthbar::update_healthbars);
}

#[derive(Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
}

impl FromWorld for UiAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        Self { font }
    }
}

fn setup_ui(mut materials: ResMut<Assets<StandardMaterial>>) {
    materials.insert(
        &SELECT_MATERIAL_HOVER,
        StandardMaterial {
            base_color: tailwind::CYAN_300.into(),
            ..default()
        },
    );

    materials.insert(
        &SELECT_MATERIAL_PRESSED,
        StandardMaterial {
            base_color: tailwind::YELLOW_300.into(),
            ..default()
        },
    );
}

/// Returns an observer that updates the entity's material to the one specified.
pub fn color_on<E>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(Trigger<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    move |trigger, mut query| {
        if let Ok(mut material) = query.get_mut(trigger.target()) {
            material.0 = new_material.clone();
        }
    }
}

pub fn create_button_outer() -> (Button, Node, Outline, BackgroundColor) {
    (
        Button,
        Node {
            width: Val::Percent(80.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(MARGIN),
            ..default()
        },
        Outline {
            width: OUTLINE_WITHD,
            color: UI_OUTLINE,
            ..default()
        },
        BackgroundColor(UI_BTN_BG),
    )
}

pub fn create_button_square() -> (Button, Node, Outline, BackgroundColor) {
    (
        Button,
        Node {
            width: Val::Px(50.),
            height: Val::Px(50.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(MARGIN),
            ..default()
        },
        Outline {
            width: OUTLINE_WITHD,
            color: UI_OUTLINE,
            ..default()
        },
        BackgroundColor(UI_BTN_BG),
    )
}
pub fn create_button_text(text: String, font: Handle<Font>) -> (Text, TextFont, TextColor) {
    (
        Text::new(text),
        TextFont {
            font: font,
            font_size: 16.0,
            ..default()
        },
        TextColor(LABEL_COLOR),
    )
}

pub fn create_title(text: &str, font: Handle<Font>) -> (Text, TextFont, TextColor) {
    (
        Text::new(text),
        TextFont {
            font,
            font_size: 50.0,
            ..default()
        },
        TextColor(LABEL_COLOR),
    )
}

#[allow(dead_code)]
pub fn create_label(text: &str, font: Handle<Font>) -> (Text, TextFont, TextColor) {
    (
        Text::new(text),
        TextFont {
            font,
            font_size: 12.0,
            ..default()
        },
        TextColor(LABEL_COLOR),
    )
}

// An observer listener that changes the target entity's color.
pub fn recolor_on<E: Debug + Clone + Reflect>(
    color: Color,
) -> impl Fn(Trigger<E>, Query<&mut TextColor>) {
    move |ev, mut sprites| {
        let Ok(mut sprite) = sprites.get_mut(ev.target()) else {
            return;
        };
        sprite.0 = color;
    }
}

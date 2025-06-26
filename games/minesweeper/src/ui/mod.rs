mod fade;
pub use fade::*;
mod menu;
pub use menu::*;
mod cell;
pub use cell::*;

use bevy::{
    color::palettes::{css, tailwind},
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};

use crate::{AppState, game::GamePhase};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FadePlugin::<AppState>::default(),
            FadePlugin::<GamePhase>::default(),
        ))
        .init_resource::<UiAssets>();
    }
}


pub const BACKGROUND_COLOR: Color = Color::Srgba(tailwind::GRAY_900);
pub const PANEL_BACKGROUND: Color = Color::Srgba(tailwind::GRAY_800);
pub const PANEL_BORDER: Color = Color::Srgba(tailwind::GRAY_700);


pub const TEXT_COLOR: Color = Color::Srgba(tailwind::GRAY_200);
pub const SECONDARY_TEXT_COLOR: Color = Color::Srgba(tailwind::GRAY_400);


pub const FOCUS_BACKGROUND: Color = Color::Srgba(tailwind::GRAY_900);
pub const FOCUS_BORDER: Color = Color::Srgba(css::WHITE);

pub const NORMAL_BUTTON: Color = Color::Srgba(tailwind::SLATE_500);
pub const NORMAL_BUTTON_BORDER: Color = Color::Srgba(tailwind::SLATE_400);
//const NORMAL_BUTTON_TEXT: Color = Color::Srgba(tailwind::SLATE_100);
pub const HOVERED_BUTTON: Color = Color::Srgba(tailwind::SLATE_600);
pub const HOVERED_BUTTON_BORDER: Color = Color::Srgba(tailwind::SLATE_700);
pub const PRESSED_BUTTON: Color = Color::Srgba(tailwind::SLATE_700);
pub const PRESSED_BUTTON_BORDER: Color = Color::Srgba(tailwind::SLATE_800);

pub const SLIDER_TRACK_COLOR: Color = Color::Srgba(tailwind::GRAY_800);
pub const HOVERED_SLIDER_TRACK: Color = Color::Srgba(tailwind::GRAY_400);
pub const SLIDER_THUMB_COLOR: Color = Color::Srgba(tailwind::GRAY_200);
pub const HOVERED_SLIDER_THUMB: Color = Color::Srgba(tailwind::GRAY_400);

#[derive(Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub flag: Handle<Image>,
    pub question: Handle<Image>,
    pub bomb: Handle<Image>,
}

impl FromWorld for UiAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        UiAssets {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            flag: asset_server.load("icon/medal1.png"),
            question: asset_server.load("icon/question.png"),
            bomb: asset_server.load("icon/cross.png"),
        }
    }
}

#[derive(Component)]
#[component(on_add = on_add_focus)]
pub struct Focus;

fn on_add_focus(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
    world.commands()
        .entity(entity)
        .insert((
            BackgroundColor(FOCUS_BACKGROUND),
            Outline {
                width: Val::Px(2.),
                color: FOCUS_BORDER,
                ..default()
            },
        ));
}


fn update_colors_on<E>(
    background: Color,
    outline: Color,
) -> impl Fn(Trigger<E>, Query<(&mut BackgroundColor, &mut Outline, Has<Focus>)>) {
    // An observer closure that captures `new_material`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // material. Instead, the event type is a generic, and the material is passed in.
    move |trigger, mut query| {
        if let Ok((mut bg, mut out, focus)) = query.get_mut(trigger.target()) {
            if focus {
                bg.0 = FOCUS_BACKGROUND;
                out.color = FOCUS_BORDER;
            }
            else {
                bg.0 = background;
                out.color = outline;
            }
        }
    }
}



fn update_bg<E>(background: Color) -> impl Fn(Trigger<E>, Query<&mut BackgroundColor>) {
    // An observer closure that captures `new_material`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // material. Instead, the event type is a generic, and the material is passed in.
    move |trigger, mut query| {
        if let Ok(mut bg) = query.get_mut(trigger.target()) {
            bg.0 = background;
        }
    }
}

#[derive(Component)]
#[component(on_add = on_add_h1)]
pub struct H1;

fn on_add_h1(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
    let font = world.resource::<UiAssets>().font.clone();
    world.commands().entity(entity).insert(TextFont {
        font,
        font_size: 40.0,
        ..default()
    });
}


#[derive(Component)]
#[component(on_add = on_add_h4)]
pub struct H4;

fn on_add_h4(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
    let font = world.resource::<UiAssets>().font.clone();
    world.commands().entity(entity).insert((TextFont {
        font,
        font_size: 24.0,
        ..default()
    }, TextColor(SECONDARY_TEXT_COLOR)
    ));
}

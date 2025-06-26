use crate::ui::*;
use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};

#[derive(Component, Reflect, Debug)]
#[require(
    CellState,
    Node = Node {
        flex_grow: 1.,
        width: Val::Percent(100.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    BackgroundColor = BackgroundColor(PANEL_BACKGROUND)
)]
pub struct Cell {
    pub pos: UVec2,
    pub neighbor_bombs: u8,
    pub exposed: bool,
    pub mine: bool,
}

#[derive(Component, Default, Reflect, PartialEq, Eq, Debug)]
pub enum CellState {
    #[default]
    None,
    Flagged,
    Unknown,
}

#[derive(Component, Reflect, Debug)]
pub struct CellNeighbors(pub Vec<Entity>);

#[derive(Component)]
#[require(
    Button,
    Node = Node {
        flex_grow: 1.,
        height: Val::Percent(100.),
        padding: UiRect::all(Val::Px(4.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    BackgroundColor(NORMAL_BUTTON),
    //BorderColor(PANEL_BORDER),
    Outline = Outline {
        width: Val::Px(2.),
        color: NORMAL_BUTTON_BORDER,
        ..default()
    },
    //BorderRadius::all(Val::Px(5.)),
)]
#[component(on_add = on_add_button)]
pub struct CellButton;

fn on_add_button(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
    world
        .commands()
        .entity(entity)
        .observe(update_bg::<Pointer<Over>>(HOVERED_BUTTON))
        .observe(update_bg::<Pointer<Out>>(NORMAL_BUTTON))
        .observe(update_bg::<Pointer<Pressed>>(PRESSED_BUTTON))
        .observe(update_bg::<Pointer<Released>>(HOVERED_BUTTON));
    //  .insert(
    //      BackgroundColor(normal_button),
    //  )
}

#[derive(Component)]
#[require(Node)]
#[component(on_add = on_add_button_inner)]
pub struct CellButtonInner;

fn on_add_button_inner(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
    let font = world.resource::<UiAssets>().font.clone();
    world.commands().entity(entity).insert((TextFont {
        font,
        font_size: 10.0,
        ..default()
    },));
}

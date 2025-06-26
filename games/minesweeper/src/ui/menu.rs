use crate::ui::*;
use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};

#[derive(Component)]
#[require(
    Button,
    Node = Node {
        margin: UiRect::all(Val::Px(10.0)),
        padding: UiRect::all(Val::Px(4.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    BackgroundColor(NORMAL_BUTTON),
    Outline = Outline {
        width: Val::Px(2.),
        color: NORMAL_BUTTON_BORDER,
        ..default()
    },
    BorderRadius::all(Val::Px(5.)),
)]
#[component(on_add = on_add_button)]
pub struct MenuButton;

fn on_add_button(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
    world
        .commands()
        .entity(entity)
        .observe(update_colors_on::<Pointer<Over>>(
            HOVERED_BUTTON,
            HOVERED_BUTTON_BORDER,
        ))
        .observe(update_colors_on::<Pointer<Out>>(
            NORMAL_BUTTON,
            NORMAL_BUTTON_BORDER,
        ))
        .observe(update_colors_on::<Pointer<Pressed>>(
            PRESSED_BUTTON,
            PRESSED_BUTTON_BORDER,
        ))
        .observe(update_colors_on::<Pointer<Released>>(
            HOVERED_BUTTON,
            HOVERED_BUTTON_BORDER,
        ))
        .observe(|trigger: Trigger<OnRemove, Focus>, mut query: Query<(&mut BackgroundColor, &mut Outline)> | {
            if let Ok((mut bg, mut out)) = query.get_mut(trigger.target()) {
                bg.0 = NORMAL_BUTTON;
                out.color = NORMAL_BUTTON_BORDER;
            }
        });
    //  .insert(
    //      BackgroundColor(normal_button),
    //  )
}

#[derive(Component)]
#[require(
    Node = Node {
        width: Val::Percent(100.),
        // height: Val::Px(65.0),
        ..default()
    },
    TextColor = TextColor(TEXT_COLOR),
    //TextShadow = TextShadow::default(),
    TextLayout = TextLayout{
        justify: JustifyText::Center,
        ..default()
    },
)]
#[component(on_add = on_add_button_inner)]
pub struct MenuButtonInner;

fn on_add_button_inner(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
    let font = world.resource::<UiAssets>().font.clone();
    world.commands().entity(entity).insert((TextFont {
        font,
        font_size: 18.0,
        ..default()
    },));
    //  .insert(
    //      BackgroundColor(normal_button),
    //  )
}

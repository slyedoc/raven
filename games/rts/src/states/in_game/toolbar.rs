use crate::{prelude::*, states::in_paused::InPaused};

pub fn create_toolbar(mut commands: Commands, ui: Res<UiAssets>, assets: Res<AssetServer>) {
    // ui
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(0.),
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Row,

                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(6.0)),
                //row_gap: MARGIN,
                ..default()
            },
            BackgroundColor(UI_BG),
            StateScoped(AppState::InGame),
        ))
        .with_children(|b| {
            b.spawn((
                Button,
                Node {
                    width: Val::Px(100.),
                    ..default()
                },
                Text::new("System"),
                Outline {
                    width: Val::Px(2.),
                    color: tailwind::RED_400.into(),
                    ..default()
                },
                BackgroundColor(UI_BTN_BG),
            ))
            //.observe(goto_on::<Pointer<Down>>(&AppState::Menu));
            .observe(
                |_trigger: Trigger<Pointer<Click>>, mut next: ResMut<NextState<InPaused>>| {
                    next.set(InPaused::Active);
                },
            )
            .observe(recolor_on::<Pointer<Out>>(Color::srgb(1.0, 0.0, 0.0)))
            .observe(recolor_on::<Pointer<Pressed>>(Color::srgb(0.0, 0.0, 1.0)))
            .observe(recolor_on::<Pointer<Released>>(Color::srgb(0.0, 1.0, 0.0)));

            //.observe(recolor_on::<Pointer<Down>>(Color::srgb(0.0, 0.0, 1.0)));

            b.spawn((
                Node {
                    flex_grow: 1.0,
                    ..default()
                },
                children![(
                    GoldText,
                    create_resource("0", assets.load("icons/gold-coin.png"), ui.font.clone(),)
                ),
                (
                    WoodText,
                    create_resource("0", assets.load("icons/wood.png"), ui.font.clone(),)
                ),
                (
                    HousingText,
                    create_resource("0", assets.load("icons/house.png"), ui.font.clone(),)
                )
                ],
            ));
        });
}

#[derive(Component)]
pub struct GoldText;

#[derive(Component)]
pub struct WoodText;

#[derive(Component)]
pub struct HousingText;

pub fn create_resource(text: &str, icon: Handle<Image>, font: Handle<Font>) -> impl Bundle {
    (
        Node {
            margin: UiRect::axes(Val::Px(5.), Val::Px(2.)),
            padding: UiRect::axes(Val::Px(2.), Val::Px(0.)),
            width: Val::Px(100.),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::BLACK),
        BorderColor(tailwind::RED_400.into()),
        Outline {
            width: Val::Px(2.),
            //offset: Val::Px(6.),
            color: tailwind::GRAY_400.into(),
            ..default()
        },
        children![
            (
                Node {
                    width: Val::Px(24.),
                    height: Val::Px(24.),
                    ..default()
                },
                ImageNode {
                    image: icon,
                    ..default()
                },
            ),
            (
                Text::new(text),
                TextFont {
                    font,
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Center),
            )
        ],
    )
}

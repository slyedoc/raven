use crate::prelude::*;

pub fn create_minimap(
    mut commands: Commands,
    //ui: Res<UiAssets>,
    minimap: Res<Minimap>,
) {
    // ui
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Percent(0.),
                right: Val::Percent(0.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                // align_items: AlignItems::Stretch,
                padding: UiRect::all(MARGIN),
                row_gap: MARGIN,
                ..default()
            },
            StateScoped(AppState::InGame),
            BackgroundColor(UI_BG),
        ))
        .with_children(|builder| {
            builder.spawn((
                Node {
                    width: Val::Px(minimap.width),
                    height: Val::Px(minimap.height),
                    ..Default::default()
                },
                ImageNode {
                    image: minimap.image.clone(),
                    ..Default::default()
                },
            ));
        });
}

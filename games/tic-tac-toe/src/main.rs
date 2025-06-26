mod ui;
use ui::*;

use bevy::{color::palettes::css, prelude::*};
use rand::Rng;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        MeshPickingPlugin,
    ))
    //.insert_resource(WinitSettings::desktop_app())
    .init_state::<GameState>()
    .enable_state_scoped_entities::<GameState>()
    .init_resource::<Board>()
    .init_resource::<CurrentPlayer>()
    .add_systems(Startup, setup)
    .add_systems(OnEnter(GameState::Menu), mode_ui)
    .add_systems(OnEnter(GameState::Playing), setup_game)
    .add_systems(OnEnter(GameState::GameOver), game_over_ui)
    .add_systems(
        Update,
        ai_move.run_if(resource_changed::<CurrentPlayer>.and(in_state(GameState::Playing))),
    )
    .add_observer(on_move)
    .add_observer(on_ai_move)
    .add_observer(check_winner);

    app.run();
}

fn ai_move(mut commands: Commands, current_player: Res<CurrentPlayer>, game_mode: Res<GameMode>) {
    if current_player.0 == Player::O && *game_mode == GameMode::One {
        info!("AI move");
        commands.trigger(AIMove {
            strategy: Strategy::Random,
            player: Player::O,
        });
    }
}

struct BoardCell {
    entity: Entity,
    player: Option<Player>,
}

impl Default for BoardCell {
    fn default() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
            player: Default::default(),
        }
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct Board(pub [[BoardCell; 3]; 3]);

#[derive(Resource, Default, Deref, DerefMut)]
struct CurrentPlayer(pub Player);

#[derive(Resource, Default, Deref, DerefMut)]
struct Winner(pub Option<Player>);

#[derive(Resource, PartialEq, Eq)]
enum GameMode {
    One,
    Two,
}

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

#[derive(Component, Default, Debug, Copy, Clone, PartialEq, Eq)]
enum Player {
    #[default]
    X,
    O,
}

#[derive(Component)]
struct Tile {
    row: usize,
    col: usize,
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, IsDefaultUiCamera));
}

fn mode_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // root node
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(MARGIN),
                row_gap: MARGIN,
                ..default()
            },
            Pickable::IGNORE,
            StateScoped(GameState::Menu),
        ))
        .with_children(|builder| {
            builder
                .spawn((Node {
                    flex_direction: FlexDirection::Row,
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Tic Tac Toe"),
                        TextFont {
                            font: font.clone(),
                            font_size: 60.0,
                            ..default()
                        },
                        TextColor(css::WHITE.into()),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });

            builder
                .spawn(ui::create_button())
                .with_child(ui::create_label("1 Player", font.clone()))
                .observe(
                    |_click: Trigger<Pointer<Click>>,
                     mut state: ResMut<NextState<GameState>>,
                     mut cmd: Commands| {
                        cmd.insert_resource(GameMode::One);
                        state.set(GameState::Playing);
                    },
                );

            builder
                .spawn((ui::create_button(),))
                .with_child(ui::create_label("2 Players", font.clone()))
                .observe(
                    |_click: Trigger<Pointer<Click>>,
                     mut state: ResMut<NextState<GameState>>,
                     mut cmd: Commands| {
                        cmd.insert_resource(GameMode::Two);
                        state.set(GameState::Playing);
                    },
                );

            builder
                .spawn((ui::create_button(),))
                .with_child(ui::create_label("Quit", font.clone()))
                .observe(|_click: Trigger<Pointer<Click>>, mut cmd: Commands| {
                    cmd.send_event(AppExit::Success);
                });

            builder
                .spawn((Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(10.0),
                    right: Val::Px(10.0),
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::End,
                    ..default()
                },))
                .with_children(|builder| {
                    builder.spawn((
                        Text::new(format!("Version: {}", VERSION)),
                        TextFont {
                            font: font.clone(),
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(css::WHITE.into()),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
        });
}

fn game_over_ui(mut commands: Commands, asset_server: Res<AssetServer>, winner: Res<Winner>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // root node
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(MARGIN),
                ..default()
            },
            Pickable::IGNORE,
            StateScoped(GameState::GameOver),
        ))
        .with_children(|builder| {
            builder
                .spawn(Node {
                    padding: UiRect::all(Val::Px(5.)),
                    margin: UiRect::top(Val::VMin(5.)),
                    row_gap: Val::Px(5.),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(match winner.0 {
                            Some(p) => format!("Player {:?} wins!", p),
                            None => "It's a draw!".to_string(),
                        }),
                        TextFont {
                            font: font.clone(),
                            font_size: 60.0,
                            ..default()
                        },
                        TextColor(css::BLUE.into()),
                    ));
                });

            builder
                .spawn(ui::create_button())
                .with_child(ui::create_label("Play Again", font.clone()))
                .observe(
                    |_click: Trigger<Pointer<Click>>, mut state: ResMut<NextState<GameState>>| {
                        state.set(GameState::Playing);
                    },
                );

            builder
                .spawn((ui::create_button(),))
                .with_child(ui::create_label("Exit", font.clone()))
                .observe(
                    |_click: Trigger<Pointer<Click>>, mut state: ResMut<NextState<GameState>>| {
                        state.set(GameState::Menu);
                    },
                );
        });
}

fn setup_game(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut current_player: ResMut<CurrentPlayer>,
) {
    current_player.0 = Player::X;
    let tile_size = 200.0;
    for row in 0..3 {
        for col in 0..3 {
            let tile = commands
                .spawn((
                    Sprite {
                        color: css::GRAY.into(),
                        custom_size: Some(Vec2::splat(tile_size - 10.0)),
                        ..default()
                    },
                    Transform::from_xyz(
                        col as f32 * tile_size - tile_size,
                        row as f32 * tile_size - tile_size,
                        0.0,
                    ),
                    Tile { row, col },
                    StateScoped(GameState::Playing),
                ))
                .observe(on_pointer_down)
                .id();
            // update the board
            board[row][col].entity = tile;
            board[row][col].player = None;
        }
    }
    println!("Game started");
}

fn on_pointer_down(
    trigger: Trigger<Pointer<Click>>,
    mut tiles: Query<&Tile>,
    board: ResMut<Board>,
    mut commands: Commands,
) {
    let e = trigger.target();
    let tile = tiles.get_mut(e).unwrap();
    let row = tile.row;
    let col = tile.col;
    if board[row][col].player.is_none() {
        commands.trigger(Move { row, col });
    }
}

#[derive(Event, Debug)]
struct Move {
    col: usize,
    row: usize,
}

fn on_move(
    trigger: Trigger<Move>,
    mut current_player: ResMut<CurrentPlayer>,
    mut tiles: Query<&mut Sprite>,
    mut board: ResMut<Board>,
    mut commands: Commands,
) {
    let event = trigger.event();
    let row = event.row;
    let col = event.col;
    if board[row][col].player.is_none() {
        board[row][col].player = Some(current_player.0);
        current_player.0 = match current_player.0 {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        let mut sprite = tiles.get_mut(board[row][col].entity).unwrap();
        sprite.color = match board[row][col].player {
            Some(Player::X) => css::RED.into(),
            Some(Player::O) => css::BLUE.into(),
            None => css::GRAY.into(),
        };

        commands.trigger(CheckBoard);
    }
}

#[derive(Event, Debug)]
struct CheckBoard;

fn check_winner(
    _trigger: Trigger<CheckBoard>,
    mut game_state: ResMut<NextState<GameState>>,
    board: Res<Board>,
    mut commands: Commands,
) {
    let lines = [
        // Rows
        [(0, 0), (0, 1), (0, 2)],
        [(1, 0), (1, 1), (1, 2)],
        [(2, 0), (2, 1), (2, 2)],
        // Columns
        [(0, 0), (1, 0), (2, 0)],
        [(0, 1), (1, 1), (2, 1)],
        [(0, 2), (1, 2), (2, 2)],
        // Diagonals
        [(0, 0), (1, 1), (2, 2)],
        [(0, 2), (1, 1), (2, 0)],
    ];

    for line in lines.iter() {
        if let [Some(player), Some(player2), Some(player3)] = [
            board[line[0].0][line[0].1].player,
            board[line[1].0][line[1].1].player,
            board[line[2].0][line[2].1].player,
        ] {
            if player == player2 && player2 == player3 {
                game_state.set(GameState::GameOver);
                commands.insert_resource(Winner(Some(player)));
                return;
            }
        }
    }

    if board
        .iter()
        .all(|row| row.iter().all(|tile| tile.player.is_some()))
    {
        game_state.set(GameState::GameOver);
        commands.insert_resource(Winner(None));
    }
}

#[derive(Event, Debug)]
struct AIMove {
    strategy: Strategy,
    player: Player,
}

#[derive(Debug)]
enum Strategy {
    Random,
}

fn on_ai_move(trigger: Trigger<AIMove>, board: Res<Board>, mut commands: Commands) {
    let event = trigger.event();

    match event.strategy {
        Strategy::Random => {
            let mut rng = rand::thread_rng();

            let mut empty_tiles = Vec::new();
            for (row, row_tiles) in board.iter().enumerate() {
                for (col, tile) in row_tiles.iter().enumerate() {
                    if tile.player.is_none() {
                        empty_tiles.push((row, col));
                    }
                }
            }

            if let Some((row, col)) = empty_tiles.get(rng.gen_range(0..empty_tiles.len())) {
                info!("AI move: {:?} at ({}, {})", event.player, row, col);
                commands.trigger(Move {
                    row: *row,
                    col: *col,
                });
            }
        }
    }
}

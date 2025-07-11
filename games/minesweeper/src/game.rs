use std::{f32::consts::PI, ops::Sub, process::Child, time::Instant};

use crate::{AppState, GameConfig, InGame, ui::*};

use bevy::{
    color::palettes::tailwind,
    ecs::{component::HookContext, query, spawn::SpawnIter, world::DeferredWorld},
    input::common_conditions::input_just_pressed,
    platform::collections::{HashMap, HashSet},
    prelude::*,
};
use bevy_inspector_egui::{egui::TextStyle, prelude::*};
use bevy_rand::prelude::*;
use bevy_spawn_observer::SpawnObserver;
use rand::prelude::*;
use raven_util::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<GamePhase>()
            .add_systems(OnEnter(InGame), setup)
            .add_systems(OnEnter(GamePhase::Setup), setup_game)
            .add_systems(OnEnter(GamePhase::Win), setup_win)
            .add_systems(OnEnter(GamePhase::Lose), setup_lose)
            .add_systems(
                Update,
                on_escape.run_if(in_state(InGame).and(input_just_pressed(KeyCode::Escape))),
            )
            .add_systems(Update, check_winner.run_if(on_event::<CheckWinner>))
            .add_event::<CheckWinner>()
            .add_observer(on_add_cell)
            .add_observer(explode_mine)
            .add_observer(cycle)
            .register_type::<Board>()
            .register_type::<Cell>()
            .register_type::<CellState>();
    }
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(InGame = InGame)]
#[states(scoped_entities)]
pub enum GamePhase {
    #[default]
    Setup,
    Play,
    Win,
    Lose,
}

fn setup(mut commands: Commands) {
    commands.spawn((        
        StateScoped(InGame),
        MainUI,
        ui_root("Main UI"),
        children![(
            // first row
            Name::new("Options"),
            Node {
                width: Val::Percent(100.),
                align_items: AlignItems::Center,
                ..default()
            },
            MainHeader,
            children![(
                Node {
                    width: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![(
                    MenuButton,
                    Children::spawn((
                        Spawn((MenuButtonInner, Text("Exit".to_string()),)),
                        SpawnObserver::new(
                            |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                                commands.send_event(FadeTo(AppState::Menu));
                            },
                        ),
                    )),
                )]
            )],
        ),],
    ));
}

fn setup_game(
    mut commands: Commands,
    config: Res<GameConfig>,
    main_ui: Single<Entity, With<MainUI>>,
    board: Query<Entity, With<Board>>,
    mut rng: GlobalEntropy<WyRand>,
) {
    // delete old board
    for e in board.iter() {
        commands.entity(e).despawn();
    }

    let (size, mine_count) = config.mode.get();

    let board = commands
        .spawn((
            ChildOf(main_ui.into_inner()),
            // first row
            Name::new("Board 1"),
            Board {
                size,
                mines: mine_count,
                ..default()
            },
            Node {
                display: Display::Grid,
                aspect_ratio: Some(1.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(2.0)),
                row_gap: Val::Px(2.0),
                column_gap: Val::Px(2.0),

                //padding: UiRect::all(Val::Px(2.0)),
                grid_template_columns: vec![GridTrack::flex(1.0); size.x as usize],
                grid_template_rows: vec![GridTrack::flex(1.0); size.y as usize],
                ..default()
            },
            BackgroundColor(PANEL_BORDER),
        ))
        .id();

    // create cells for the board
    let mut entities = vec![Entity::PLACEHOLDER; size.x as usize * size.y as usize];
    for y in 0..size.y {
        for x in 0..size.x {
            let index = (y * size.x + x) as usize;
            entities[index] = commands
                .spawn((Name::new(format!("Cell {} {}", x, y)), ChildOf(board)))
                .id();
        }
    }

    // figure out where bombs are to be placed
    let mut mines = vec![false; size.x as usize * size.y as usize];
    let mut mines_placed = 0;
    while mines_placed < mine_count {
        let x = rng.random_range(0..size.x);
        let y = rng.random_range(0..size.y);
        let index = (y * size.x + x) as usize;
        if mines[index] {
            continue;
        }
        mines[index] = true;
        mines_placed += 1;
    }

    // figure out how many bombs are around each cell
    let mut neighboors_bombs = vec![0; size.x as usize * size.y as usize];
    for y in 0..size.y {
        for x in 0..size.x {
            let index = (y * size.x + x) as usize;
            if mines[index] {
                continue;
            }
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let tx: i32 = x as i32 + dx;
                    let ty: i32 = y as i32 + dy;
                    if tx < 0 || ty < 0 || tx >= size.x as i32 || ty >= size.y as i32 {
                        continue;
                    }
                    let tindex = (ty as u32 * size.x + tx as u32) as usize;
                    if mines[tindex] {
                        neighboors_bombs[index] += 1;
                    }
                }
            }
        }
    }

    // neighboors
    let mut neighboors = vec![Vec::<Entity>::new(); size.x as usize * size.y as usize];
    for y in 0..size.y {
        for x in 0..size.x {
            let index = (y * size.x + x) as usize;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let tx: i32 = x as i32 + dx;
                    let ty: i32 = y as i32 + dy;
                    if tx < 0 || ty < 0 || tx >= size.x as i32 || ty >= size.y as i32 {
                        continue;
                    }
                    let tindex = (ty as u32 * size.x + tx as u32) as usize;
                    neighboors[index].push(entities[tindex]);
                }
            }
        }
    }

    for y in 0..size.y {
        for x in 0..size.x {
            let index = (y * size.x + x) as usize;
            let e = entities[index];
            commands.entity(e).insert((
                Cell {
                    pos: UVec2::new(x, y),
                    neighbor_bombs: neighboors_bombs[index],
                    exposed: false,
                    mine: mines[index],
                },
                CellNeighbors(neighboors[index].clone()),
            ));
            commands.trigger_targets(OnAdd, e);
        }
    }

    commands.set_state(GamePhase::Play);
}

fn setup_win(
    mut commands: Commands,
    header: Single<Entity, With<MainHeader>>,   
    query: Query<(Entity, &Cell)>, 
) {

    // explode all mines
    for (e, cell) in query.iter() {
        if cell.mine {
            commands.trigger_targets(Explode(false), e);
        }
    }
    
    let header = header.into_inner();
    commands.spawn((
        Name::new("Win Title"),
        ChildOf(header),
        StateScoped(GamePhase::Win),
        Node {
            width: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,

            ..default()
        },
        children![(
            Text::new("You win!"),
            TextFont {
                font_size: 24.,
                ..default()
            },
            TextLayout {
                justify: JustifyText::Center,
                ..default()
            },
            TextColor(tailwind::GREEN_300.into()),
        ),],
    ));

    commands.spawn((
        Name::new("Right Options"),
        ChildOf(header),
        StateScoped(GamePhase::Win),
        Node {
            width: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            MenuButton,
            Name::new("New Game Button"),
            Children::spawn((
                Spawn((MenuButtonInner, Text("New Game".to_string()),)),
                SpawnObserver::new(
                    |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                        commands.send_event(FadeTo(GamePhase::Setup));
                    },
                ),
            )),
        )],
    ));
}

fn setup_lose(
    mut commands: Commands,
    main_ui: Single<Entity, With<MainHeader>>,
    _board: Single<&Board>,

    query: Query<(Entity, &Cell)>,
) {
    info!("You lose!");

    // explode all mines
    for (e, cell) in query.iter() {
        if cell.mine {
            commands.trigger_targets(Explode(false), e);
        }
    }

    let ui_e = main_ui.into_inner();

    commands.spawn((
        Name::new("Lose Text"),
        ChildOf(ui_e),
        Node {
            width: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        StateScoped(GamePhase::Lose),
        children![(
            Text::new("Boom"),
            TextFont {
                font_size: 24.,
                ..default()
            },
            TextColor(tailwind::RED_300.into()),
        )],
    ));

    commands.spawn((
        Name::new("Lose UI"),
        ChildOf(ui_e),
        Node {
            width: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        StateScoped(GamePhase::Lose),
        children![(
            MenuButton,
            Children::spawn((
                Spawn((MenuButtonInner, Text("New Game".to_string()),)),
                SpawnObserver::new(
                    |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                        commands.send_event(FadeTo(GamePhase::Setup));
                    },
                ),
            )),
        )],
    ));
}

fn on_add_cell(trigger: Trigger<OnAdd, Cell>, mut commands: Commands) {
    let e = trigger.target();

    commands
        .spawn((
            ChildOf(e),
            Name::new("Cell Button"),
            CellButton,
            children![(Name::new("Cell Button Inner"), CellButtonInner,)],
        ))
        .observe(
            move |trigger: Trigger<Pointer<Click>>,
                  mut commands: Commands,
                  state: Res<State<GamePhase>>| {
                // dont process clicks if not in play phase
                let state = state.get();
                if state != &GamePhase::Play {
                    return;
                }

                match trigger.event().button {
                    PointerButton::Primary => {
                        commands.trigger_targets(Explode(true), e);
                    }
                    PointerButton::Secondary => {
                        commands.trigger_targets(Cycle, e);
                    }
                    _ => (),
                };
            },
        );
}

#[derive(Component, Clone, Default, Reflect, Debug)]
#[reflect(Component)]

pub struct Board {
    pub size: UVec2,
    pub mines: usize,
}

#[derive(Component, Reflect, Debug)]
pub struct MainUI;

#[derive(Component, Reflect, Debug)]
pub struct MainHeader;

#[derive(Component, Reflect, Debug)]
pub struct Submit;

fn on_escape(mut commands: Commands) {
    commands.set_state(AppState::Menu);
}

#[derive(Event)]
pub struct Explode(pub bool); // propergate or not

fn explode_mine(
    trigger: Trigger<Explode>,
    mut query: Query<(&mut Cell, &CellNeighbors, Option<&Children>)>,
    mut commands: Commands,
    ui: Res<UiAssets>,
) {
    let id = trigger.target();
    let propergate = trigger.event().0;

    let (mut cell, neighbors, children) = query.get_mut(id).unwrap();

    // Check if the cell is already exposed
    if cell.exposed {
        return;
    }

    // Remove the cell button
    cell.exposed = true;
    if let Some(children) = children {
        for c in children.iter() {
            commands.entity(c).despawn();
        }
    }

    // update text
    if cell.neighbor_bombs != 0 {
        commands.spawn((
            ChildOf(id),
            Node::default(),
            Text::new(format!("{}", cell.neighbor_bombs)),
            TextFont {
                font_size: 20.,
                ..default()
            },
            TextColor(SECONDARY_TEXT_COLOR),
        ));
    }

    // we hit a mine
    if cell.mine  {
        commands.entity(id).insert((ImageNode {
            image: ui.bomb.clone(),
            ..default()
        },));

        if propergate {
            // game over
            commands.set_state(GamePhase::Lose);
        }
        return;
    }

    

    if propergate {
        // clear surrounding cells only on empty cell    
        if cell.neighbor_bombs == 0 {
            for e in neighbors.0.iter() {
                commands.trigger_targets(Explode(true), *e);
            }
        }

        commands.send_event(CheckWinner);
    }
}

#[derive(Event)]
struct Cycle;

fn cycle(
    trigger: Trigger<Cycle>,
    mut query: Query<&mut CellState>,
    children_query: Query<&Children>,
    ui: Res<UiAssets>,
    mut cell_button_inner_query: Query<&mut Node, With<CellButtonInner>>,
    mut commands: Commands,
    //board: Single<Entity, With<Board>>,
) {
    let e = trigger.target();
    let mut state = query.get_mut(e).unwrap();

    let t = match *state {
        CellState::None => CellState::Flagged,
        CellState::Flagged => CellState::Unknown,
        CellState::Unknown => CellState::None,
    };
    for c in children_query.iter_descendants(e) {
        if let Ok(mut node) = cell_button_inner_query.get_mut(c) {
            match t {
                CellState::None => {
                    commands.entity(c).remove::<ImageNode>();
                }
                CellState::Flagged => {
                    node.width = Val::Percent(100.);
                    node.height = Val::Percent(100.);
                    commands.entity(c).insert(ImageNode {
                        image: ui.flag.clone(),

                        ..default()
                    });
                }
                CellState::Unknown => {
                    commands.entity(c).insert(ImageNode {
                        image: ui.question.clone(),
                        ..default()
                    });
                }
            }
        }
    }
    *state = t;

    commands.send_event(CheckWinner);
}

// Send when the board needs to be checked for a winner
// Cant do this as trigger because it would fire any time a cell is changed, not once a frame if needed
#[derive(Event)]
pub struct CheckWinner;

fn check_winner(
    mut commands: Commands,
    query: Query<(&Cell, &CellState)>,
    config: Res<GameConfig>,
) {
    let (_size, mines) = config.mode.get();
    // check if all cells are exposed except for the mines
    let exposed = query.iter().filter(|(c, _s)| !c.exposed).count();

    // all mines are marked and only mines are marked
    let marked = query
        .iter()
        .filter(|(c, s)| **s == CellState::Flagged && c.mine)
        .count();
    if marked == mines || exposed == mines {
        commands.set_state(GamePhase::Win);
    }
}

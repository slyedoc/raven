// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::{prelude::*, window::WindowResolution};

use rts::RtsPlugin;

#[cfg(feature = "dev")]
use clap::Parser;

#[cfg(feature = "dev")]
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    // #[clap(short, long, value_enum)]
    // level: Option<Level>,
    #[clap(short, long)]
    seed: Option<u8>,
}

fn main() {
    let mut app = App::new();
    // let mut level = None;
    let seed = if cfg!(feature = "dev") {
        let args = Args::parse();
        args.seed
    } else {
        None
    };    
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Raven: RTS".to_string(),
                    resolution: WindowResolution::new(1920.0, 1080.0),
                    ..default()
                }),
                ..default()
            }),
        RtsPlugin {  
            seed,
        },      
        
    )).run();
}
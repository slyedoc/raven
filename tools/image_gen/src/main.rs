use image::{ImageReader, ImageError};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "image_gen")]
#[command(about = "Image processing utilities for texture generation")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert displacement image (white at highest) to depth map (black at highest)
    Depth {
        /// Input displacement image
        input: PathBuf,
        /// Output depth map image
        output: PathBuf,
    },
}

fn main() -> Result<(), ImageError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Depth { input, output } => {
            convert_depth(input, output)?;
        }
    }

    Ok(())
}

fn convert_depth(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), ImageError> {
    let img = ImageReader::open(input_path)?
        .decode()?;
    
    let mut rgba_img = img.to_rgba8();
    
    for pixel in rgba_img.pixels_mut() {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];
        let a = pixel[3];
        
        pixel[0] = 255 - r;
        pixel[1] = 255 - g;
        pixel[2] = 255 - b;
        pixel[3] = a;
    }
    
    rgba_img.save(output_path)?;
    
    println!("Inverted heightmap saved to: {}", output_path.display());
    Ok(())
}


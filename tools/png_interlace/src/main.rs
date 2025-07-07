use clap::Parser;
use image::{ImageReader, ImageFormat};

#[derive(Parser)]
#[command(name = "png_interlace")]
#[command(about = "Convert interlaced PNG files to non-interlaced format")]
struct Args {
    /// Input PNG file
    input: String,
    
    /// Output PNG file
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Load the image
    let img = ImageReader::open(&args.input)?;
    
    // Verify it's a PNG
    if img.format() != Some(ImageFormat::Png) {
        return Err("Input file must be a PNG".into());
    }
    
    let decoded_img = img.decode()?;
    
    // Save as PNG (image crate saves as non-interlaced by default)
    decoded_img.save(&args.output)?;
    
    println!("Converted {} to {} (non-interlaced)", args.input, args.output);
    Ok(())
}
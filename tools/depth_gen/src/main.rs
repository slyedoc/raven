use image::{ImageReader, ImageError};
use std::env;

fn main() -> Result<(), ImageError> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <input_image> <output_image>", args[0]);
        std::process::exit(1);
    }
    
    let input_path = &args[1];
    let output_path = &args[2];
    
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
    
    println!("Inverted heightmap saved to: {}", output_path);
    Ok(())
}
use std::env;
use std::fs::File;
use std::path::Path;

use petpet::FilterType;
use petpet::{generate, encode_gif};


pub fn file_to_gif(input: impl AsRef<Path>, output: impl AsRef<Path>, speed: i32, filter: FilterType) -> Result<(), Box<dyn std::error::Error>> {
    let input_image = image::open(&input)?.to_rgba8();
    let frames = generate(input_image, filter)?;
    let output = File::open(output)?;
    encode_gif(frames, output, speed)?;
    Ok(())
}

fn main() {
    let mut args = env::args();
    args.next();
    
    let input = args.next().expect("input file is required!");
    let output = args.next().expect("output file is required!");
    let speed = args.next().expect("speed is required!").parse().unwrap();

    file_to_gif(&input, &output, speed, FilterType::Lanczos3).unwrap();
}

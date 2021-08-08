use std::env;

use petpet::generate;
use petpet::encode_gif;

use::petpet::FilterType;

fn main() {
    let input = env::args().nth(1).expect("input file is required!");
    let output = env::args().nth(2).expect("output file is required!");
    let speed = env::args().nth(3).expect("speed is required!").parse().unwrap();
    for _ in 0..100 {
        let input_image = image::open(&input).expect("cannot read imput image").to_rgba8();
        let frames = generate(input_image, FilterType::Triangle).expect("cannot generate frames!");
        encode_gif(frames, &output, speed).expect("cannot export GIF!");
    }
}

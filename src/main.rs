use std::env;

use petpet::generate;
use petpet::encode_gif;

fn main() {
    let input = env::args().nth(1).expect("input file is required!");
    let output = env::args().nth(2).expect("output file is required!");

    let input_image = image::open(&input).expect("cannot read imput image").to_rgba8();

    let frames = generate(input_image).expect("cannot generate frames!");

    encode_gif(frames, output).expect("cannot export GIF!");
}
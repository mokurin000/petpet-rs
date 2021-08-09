use std::env;

use petpet::file_to_gif;
use petpet::FilterType;

fn main() {
    let input = env::args().nth(1).expect("input file is required!");
    let output = env::args().nth(2).expect("output file is required!");
    let speed = env::args().nth(3).expect("speed is required!").parse().unwrap();

    file_to_gif(&input, &output, speed, FilterType::Lanczos3).unwrap();
}

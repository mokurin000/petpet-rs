use std::env;

use petpet::file_to_gif;
use petpet::FilterType;

fn main() {
    let mut args = env::args();
    let input = args.next().expect("input file is required!");
    let output = args.next().expect("output file is required!");
    let speed = args.next().expect("speed is required!").parse().unwrap();

    file_to_gif(&input, &output, speed, FilterType::Lanczos3).unwrap();
}

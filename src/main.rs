use std::env;

#[cfg(feature = "encode_to_gif")]
pub fn encode_petpet_gif(
    input: impl AsRef<std::path::Path>,
    output: impl AsRef<std::path::Path>,
    speed: i32,
    filter: petpet::FilterType,
) -> Result<(), Box<dyn std::error::Error>> {
    use petpet::{encode_gif, generate};
    use std::fs::File;

    let input_image = image::open(&input)?.to_rgba8();
    let frames = generate(input_image, filter)?;
    let output = File::create(output)?;
    encode_gif(frames, output, speed)?;
    Ok(())
}

fn main() {
    #[cfg(not(feature = "encode_to_gif"))]
    compile_error!("petpet-cli requires encode_to_gif");

    let mut args = env::args();
    args.next();

    let input = args.next().expect("input file is required!");
    let output = args.next().expect("output file is required!");
    let speed = args.next().expect("speed is required!").parse().unwrap();

    #[cfg(feature = "encode_to_gif")]
    encode_petpet_gif(&input, &output, speed, petpet::FilterType::Lanczos3).unwrap();
}

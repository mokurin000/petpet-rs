use std::env;

fn main() {
    #[cfg(not(any(feature = "encode_to_gif", feature = "encode_to_apng")))]
    compile_error!("petpet-cli requires encode_to_gif/encode_to_apng");

    let mut args = env::args();
    args.next();

    let input = args.next().expect("input file is required!");
    let output = args.next().expect("output file is required!");
    let speed = args.next().expect("speed is required!").parse().unwrap();

    #[cfg(feature = "encode_to_gif")]
    encode_petpet_gif(&input, &output, speed, petpet::FilterType::Lanczos3).unwrap();
    #[cfg(feature = "encode_to_apng")]
    encode_petpet_apng(
        &input,
        &output.replace(".gif", ".apng"),
        petpet::FilterType::Lanczos3,
        petpet::png::FilterType::NoFilter,
    )
    .unwrap();
}

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
    let frames = generate(input_image, filter, None)?;
    let output = File::create(output)?;
    encode_gif(frames, output, speed)?;
    Ok(())
}

#[cfg(feature = "encode_to_apng")]
pub fn encode_petpet_apng(
    input: impl AsRef<std::path::Path>,
    output: impl AsRef<std::path::Path>,
    filter: petpet::FilterType,
    png_filter: petpet::png::FilterType,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use petpet::{encode_apng, generate};
    use std::fs::File;

    let input_image = image::open(&input)?.to_rgba8();
    let frames = generate(input_image, filter, None)?;
    let output = File::create(output)?;

    encode_apng(frames, output, png_filter, 1, 50)?;
    Ok(())
}

use std::io::Write;

use image::{Frame, ImageResult};

/// Encode Frame to GIF.
///
/// `frames` will encode as  GIF,
///
/// `output` should be the path of the GIF file.
///
/// [speed]: https://doc.servo.org/color_quant/struct.NeuQuant.html#method.new
///
/// for details of speed, please see Servo's [documents][speed].
pub fn encode_gif(
    frames: impl IntoIterator<Item = Frame>,
    output: impl Write,
    speed: i32,
) -> ImageResult<()> {
    use image::codecs::gif::GifEncoder;
    use image::codecs::gif::Repeat;

    let mut encoder = GifEncoder::new_with_speed(output, speed);
    encoder.set_repeat(Repeat::Infinite)?;
    encoder.encode_frames(frames)?;
    Ok(())
}

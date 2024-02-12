use std::io::Write;
use std::sync::OnceLock;

use image::error::ImageResult;
use image::RgbaImage;
use image::{Frame, ImageError, ImageFormat};

use image::codecs::gif::GifEncoder;
use image::codecs::gif::Repeat;
use image::Delay;

use image::imageops::overlay;
use image::imageops::resize;

pub use image::imageops::FilterType;

const FRAMES: u32 = 10;
const FRAME_DELAY: u32 = 20;
const HAND_HEIGHT_WIDTH: u32 = 112;

mod hand_raw;
static HANDS: OnceLock<[RgbaImage; 5]> = OnceLock::new();

fn load_png(buf: &[u8]) -> Result<RgbaImage, ImageError> {
    use image::load_from_memory_with_format;

    let dyn_image = load_from_memory_with_format(buf, ImageFormat::Png)?;
    Ok(dyn_image.to_rgba8())
}

/// Generate frames overlayed hands.
///
/// `image` is the Rgba8 Image to generate petpet.
pub fn generate(
    image: RgbaImage,
    filter: FilterType,
) -> ImageResult<impl IntoIterator<Item = Frame>> {
    let mut frames = Vec::<Frame>::new();

    let hands = HANDS.get_or_init(|| {
        [
            load_png(hand_raw::HAND_0).unwrap(),
            load_png(hand_raw::HAND_1).unwrap(),
            load_png(hand_raw::HAND_2).unwrap(),
            load_png(hand_raw::HAND_3).unwrap(),
            load_png(hand_raw::HAND_4).unwrap(),
        ]
    });

    for i in 0..FRAMES {
        let squeeze = if i < FRAMES / 2 { i } else { FRAMES - i } as f64;

        let width_scale = 0.8 + squeeze * 0.02;
        let height_scale = 0.8 - squeeze * 0.05;

        let width = (width_scale * HAND_HEIGHT_WIDTH as f64) as u32;
        let height = (height_scale * HAND_HEIGHT_WIDTH as f64) as u32;

        let offset_x = (((1.0 - width_scale) * 0.5 + 0.1) * HAND_HEIGHT_WIDTH as f64) as i64;
        let offset_y = (((1.0 - height_scale) - 0.08) * HAND_HEIGHT_WIDTH as f64) as i64;

        let resized_background = resize(&image, width, height, filter);

        let mut composited_image = RgbaImage::new(HAND_HEIGHT_WIDTH, HAND_HEIGHT_WIDTH);

        overlay(
            &mut composited_image,
            &resized_background,
            offset_x,
            offset_y,
        );

        let overlay_hand = &hands[i as usize / 2];
        overlay(&mut composited_image, overlay_hand, 0, 0);

        let frame_with_delay = Frame::from_parts(
            composited_image,
            0,
            0,
            Delay::from_numer_denom_ms(FRAME_DELAY, 1),
        );

        frames.push(frame_with_delay);
    }
    Ok(frames)
}

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
    let mut encoder = GifEncoder::new_with_speed(output, speed);
    encoder.set_repeat(Repeat::Infinite)?;
    encoder.encode_frames(frames)?;
    Ok(())
}

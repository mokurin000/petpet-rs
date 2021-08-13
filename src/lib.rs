#![feature(once_cell)]
use std::io::Write;
use std::lazy::SyncLazy;

use image::error::ImageResult;
use image::Frame;
use image::{Rgba, RgbaImage};

use image::codecs::gif::GifEncoder;
use image::codecs::gif::Repeat;
use image::Delay;

use image::imageops::overlay;
use image::imageops::resize;

pub use image::imageops::FilterType;

const FRAMES: u32 = 10;
const RESOLUTION: (u32, u32) = (112, 112);
static HANDS: SyncLazy<Vec<RgbaImage>> = SyncLazy::new(|| {
    (0..5)
        .map(|num| format!("{}.png", num))
        .map(|file| {
            use std::path::PathBuf;
            let mut path = PathBuf::from(".");
            path.push("res");
            path.push(file);

            image::open(&path)
                .unwrap_or_else(|_| panic!("Could not load image at {:?}", path))
                .to_rgba8()
        })
        .collect()
});

/// Generate frames overlayed hands.
///
/// `image` is the Rgba8 Image to generate petpet.
pub fn generate(
    image: RgbaImage,
    filter: FilterType,
) -> ImageResult<impl IntoIterator<Item = Frame>> {
    let mut frames = Vec::<Frame>::new();

    for i in 0..FRAMES {
        let squeeze = if i < FRAMES / 2 { i } else { FRAMES - i } as f64;

        let width_scale = 0.8 + squeeze * 0.02;
        let height_scale = 0.8 - squeeze * 0.05;

        let width = (width_scale * RESOLUTION.0 as f64) as u32;
        let height = (height_scale * RESOLUTION.1 as f64) as u32;

        let offset_x = (((1.0 - width_scale) * 0.5 + 0.1) * RESOLUTION.0 as f64) as u32;
        let offset_y = (((1.0 - height_scale) - 0.08) * RESOLUTION.1 as f64) as u32;

        let calucate_then_resize = resize(&image, width, height, filter);

        let mut resize_then_overlay = RgbaImage::new(RESOLUTION.0, RESOLUTION.1);

        overlay(
            &mut resize_then_overlay,
            &calucate_then_resize,
            offset_x,
            offset_y,
        );

        for (pixel_hand, pixel_canvas) in HANDS[i as usize / 2]
            .pixels()
            .zip(resize_then_overlay.pixels_mut())
        {
            if ! matches!(pixel_hand, Rgba([_, _, _, 0])) {
                *pixel_canvas = *pixel_hand;
            }
        }

        const DELAY: u32 = 20;
        let overlay_then_delay = Frame::from_parts(
            resize_then_overlay,
            0,
            0,
            Delay::from_numer_denom_ms(DELAY, 1),
        );

        frames.push(overlay_then_delay);
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
pub fn encode_gif (
    frames: impl IntoIterator<Item = Frame>,
    output: impl Write,
    speed: i32
) -> ImageResult<()> {
    let mut encoder = GifEncoder::new_with_speed(output, speed);
    encoder.set_repeat(Repeat::Infinite)?;
    encoder.encode_frames(frames)?;
    Ok(())
}

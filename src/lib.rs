use std::sync::OnceLock;

use image::error::ImageResult;
use image::{Frame, ImageError};
use image::{GenericImageView, Rgba, RgbaImage};

use image::Delay;

use image::imageops::overlay;
use image::imageops::resize;

pub use image::imageops::FilterType;

const FRAMES: usize = 10;
const FRAME_DELAY: u32 = 20;
pub const HAND_HEIGHT_WIDTH: u32 = 112;

static HANDS_RGBA: OnceLock<[RgbaImage; 5]> = OnceLock::new();

#[cfg(not(feature = "bundle_raw_hands"))]
fn load_hand_webp(buf: &[u8]) -> Result<RgbaImage, ImageError> {
    use image::load_from_memory_with_format;

    let dyn_image = load_from_memory_with_format(buf, image::ImageFormat::WebP)?;
    Ok(dyn_image.to_rgba8())
}

#[cfg(feature = "bundle_raw_hands")]
fn load_hand_raw(buf: &[u8]) -> Result<RgbaImage, ImageError> {
    use image::error::DecodingError;

    let image = RgbaImage::from_raw(HAND_HEIGHT_WIDTH, HAND_HEIGHT_WIDTH, buf.to_vec()).ok_or(
        ImageError::Decoding(DecodingError::from_format_hint(
            image::error::ImageFormatHint::Unknown,
        )),
    )?;
    Ok(image)
}

/// Generate frames overlayed hands.
///
/// `image` is the Rgba8 Image to generate petpet.
pub fn generate(
    image: RgbaImage,
    filter: FilterType,
) -> ImageResult<impl IntoIterator<Item = Frame>> {
    let hands = HANDS_RGBA.get_or_init(|| {
        #[cfg(not(feature = "bundle_raw_hands"))]
        {
            use hands::HANDS_WEBP;
            return HANDS_WEBP.map(|img| load_hand_webp(img).unwrap());
        }

        #[cfg(feature = "bundle_raw_hands")]
        {
            use hands::HANDS_RAW;
            return HANDS_RAW.map(|img| load_hand_raw(img).unwrap());
        }
    });
    Ok((0..FRAMES).map(move |num| encode_single_frame(num, &image, filter, hands)))
}

/// encode a petpet frame
///
/// accepts `hands` where they have same size with `HANDS_HEIGHT_WIDTH`
///
pub fn encode_single_frame(
    num: usize,
    image: &RgbaImage,
    filter: FilterType,
    hands: &[impl GenericImageView<Pixel = Rgba<u8>>; FRAMES / 2],
) -> image::Frame {
    let squeeze = if num < FRAMES / 2 { num } else { FRAMES - num } as f64;
    let width_scale = 0.8 + squeeze * 0.02;
    let height_scale = 0.8 - squeeze * 0.05;

    let width = (width_scale * HAND_HEIGHT_WIDTH as f64) as u32;
    let height = (height_scale * HAND_HEIGHT_WIDTH as f64) as u32;

    let offset_x = (((1.0 - width_scale) * 0.5 + 0.1) * HAND_HEIGHT_WIDTH as f64) as i64;
    let offset_y = (((1.0 - height_scale) - 0.08) * HAND_HEIGHT_WIDTH as f64) as i64;

    let resized_background = resize(image, width, height, filter);

    let mut composited_image = RgbaImage::new(HAND_HEIGHT_WIDTH, HAND_HEIGHT_WIDTH);

    overlay(
        &mut composited_image,
        &resized_background,
        offset_x,
        offset_y,
    );

    let hand_overlay = &hands[num as usize / 2];
    overlay(&mut composited_image, hand_overlay, 0, 0);

    let frame_with_delay = Frame::from_parts(
        composited_image,
        0,
        0,
        Delay::from_numer_denom_ms(FRAME_DELAY, 1),
    );

    frame_with_delay
}

#[cfg(feature = "encode_to_gif")]
mod encode_gif;
#[cfg(feature = "encode_to_gif")]
pub use encode_gif::encode_gif;

mod hands;

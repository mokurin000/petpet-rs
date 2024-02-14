use std::{error::Error, io::Write};

use apng::{BlendOp, DisposeOp, PNGImage};
use image::{DynamicImage, Frame};

use crate::HAND_HEIGHT_WIDTH;

/// Encode Frame to APNG.
///
/// Delay = (delay_num / delay_den) sec
pub fn encode_apng(
    frames: impl IntoIterator<Item = Frame>,
    mut output: impl Write,
    filter: png::FilterType,
    delay_num: u16,
    delay_den: u16,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let images = frames
        .into_iter()
        .map(Frame::into_buffer)
        .map(DynamicImage::ImageRgba8)
        .map(apng::load_dynamic_image)
        .collect::<Result<Vec<PNGImage>, _>>()?;

    let mut encoder = apng::Encoder::new(
        &mut output,
        apng::Config {
            width: HAND_HEIGHT_WIDTH,
            height: HAND_HEIGHT_WIDTH,
            num_plays: 0,
            num_frames: images.len() as _,
            color: png::ColorType::Rgba,
            depth: png::BitDepth::Eight,
            filter,
        },
    )?;

    encoder.encode_all(
        images,
        Some(&apng::Frame {
            dispose_op: Some(DisposeOp::ApngDisposeOpBackground),
            blend_op: Some(BlendOp::ApngBlendOpOver),
            delay_num: Some(delay_num),
            delay_den: Some(delay_den), // num / den
            ..Default::default()
        }),
    )?;

    Ok(())
}

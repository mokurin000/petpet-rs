#![feature(once_cell)]
use std::fs::File;
use std::lazy::SyncLazy;
use std::path::PathBuf;

use image::error::ImageResult;

use image::Frame;
use image::GenericImage;
use image::{Rgba, RgbaImage};

use image::codecs::gif::GifEncoder;
use image::codecs::gif::Repeat;

use image::imageops::crop;

use imageproc::geometric_transformations;

use geometric_transformations::translate;
use geometric_transformations::warp;
use geometric_transformations::Interpolation;
use geometric_transformations::Projection;

const FRAMES: u32 = 10;
const RESOLUTION: (u32, u32) = (112, 112);
const HANDS: SyncLazy<Vec<RgbaImage>> = SyncLazy::new(|| {
    (0..5)
        .map(|num| format!("{}.png", num))
        .map(|file| {
            let mut path = PathBuf::from(".");
            path.push("res");
            path.push(file);

            image::open(&path)
                .expect(&format!("Could not load image at {:?}", path))
                .to_rgba8()
        })
        .collect()
});

pub fn generate(image: RgbaImage) -> ImageResult<impl IntoIterator<Item = Frame>> {
    let image = translate(&image, (RESOLUTION.0 as i32, RESOLUTION.1 as i32));
    let mut frame = Vec::<Frame>::new();
    for i in 0..FRAMES {
        let squeeze = if i < FRAMES / 2 { i } else { FRAMES - i } as f32;

        let width = squeeze * 0.02 + 0.8;
        let height = squeeze * 0.05 + 0.8;

        let offset_x = (((1.0 - width) * 0.5 + 0.1) * RESOLUTION.0 as f32) as u32;
        let offset_y = (((1.0 - height) - 0.08) * RESOLUTION.1 as f32) as u32;

        let size_x = RESOLUTION.0 - offset_x;
        let size_y = RESOLUTION.1 - offset_y;

        let mut image = warp(
            &image,
            &Projection::scale(width, height),
            Interpolation::Bicubic,
            Rgba([0, 0, 0, 0]),
        ); // rescale image

        let image = crop(&mut image, 0, 0, size_x, size_y); // crop

        let mut canvas = RgbaImage::new(RESOLUTION.0, RESOLUTION.1);

        canvas.copy_from(&image, offset_x, offset_y)?;
        canvas.copy_from(&HANDS[i as usize], 0, 0)?;
        frame.push(Frame::new(canvas));
    }
    Ok(frame)
}

pub fn encode_gif<'a>(frames: impl IntoIterator<Item = Frame>, output: impl Into<PathBuf>) -> ImageResult<()> {
    let buf = File::create(output.into())?;
    let mut encoder = GifEncoder::new_with_speed(buf, 20);
    encoder.set_repeat(Repeat::Infinite)?;
    encoder.encode_frames(frames)?;
    Ok(())
}

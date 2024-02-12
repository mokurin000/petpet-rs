use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "bundle_raw_hands")]
    decode_webp_hands()?;

    Ok(())
}

#[cfg(feature = "bundle_raw_hands")]
fn decode_webp_hands() -> Result<(), Box<dyn Error>> {
    use std::{
        fs::{self, read_dir},
        path::PathBuf,
    };

    let hand_paths = read_dir("src/res")?
        .into_iter()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|image_path| image_path.is_file())
        .filter(|image_path| {
            image_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .ends_with(".webp")
        })
        .collect::<Vec<_>>();

    for hand_path in &hand_paths {
        println!("cargo:rerun-if-changed={}", hand_path.to_string_lossy());
    }

    for hand_path in hand_paths {
        let raw_hand_path = PathBuf::from(std::env::var("OUT_DIR")?).join(
            hand_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .replace(".webp", ".raw"),
        );
        let hand = image::open(hand_path)?;
        assert_eq!(hand.height(), 112);
        assert_eq!(hand.width(), 112);
        let raw_buffer = hand.to_rgba8().into_raw();
        fs::write(raw_hand_path, raw_buffer)?;
    }

    Ok(())
}

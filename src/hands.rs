#[cfg(not(feature = "bundle_raw_hands"))]
pub static HANDS_WEBP: [&[u8]; 5] = [
    include_bytes!("res/0.webp"),
    include_bytes!("res/1.webp"),
    include_bytes!("res/2.webp"),
    include_bytes!("res/3.webp"),
    include_bytes!("res/4.webp"),
];

#[cfg(feature = "bundle_raw_hands")]
pub static HANDS_RAW: [&[u8]; 5] = [
    include_bytes!("res/0.raw"),
    include_bytes!("res/1.raw"),
    include_bytes!("res/2.raw"),
    include_bytes!("res/3.raw"),
    include_bytes!("res/4.raw"),
];

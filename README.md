# petpet-rs

[![crates.io](https://img.shields.io/crates/v/petpet.svg)](https://crates.io/crates/petpet)

Also [petpet](https://github.com/camprevail/pet-pet-gif/), but in Rust.

## Usage

```bash
cargo +nightly run --release -- <input_image> <output_gif> <encode_speed>
```

**[details about encode_speed](https://doc.servo.org/color_quant/struct.NeuQuant.html#method.new)**

Note that **format of the input image must be the same as suffix name explains**,

otherwise you will get an error like `Bad Signature`!

## Todo

- [x] transparent background: [#1](https://github.com/poly000/petpet-rs/issues/1)

- [ ] Upload v1.0.0 to crates.io: [#2](https://github.com/poly000/petpet-rs/issues/2)

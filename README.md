# petpet-rs

[![crates.io](https://img.shields.io/crates/v/petpet.svg)](https://crates.io/crates/petpet)

[![Example Image](https://user-images.githubusercontent.com/34085039/129224045-41649633-7fb1-4bdf-85ce-eadfac183c3d.gif)](https://yande.re/post/show/304166)

Also [petpet](https://github.com/camprevail/pet-pet-gif/), but in Rust.

The default hands' images were credited by [PetPet Generator](https://benisland.neocities.org/petpet/).

[Here](https://github.com/poly000/awesome-petpet-hands) you can download more custom hands.

You need to replace petpet images in src/res then **recompile** petpet.

**Note:** crates.io's version is outdated.

## Download

You can download the latest release from [releases](https://github.com/poly000/petpet-rs/releases) page.

## Build

```bash
cargo +nightly build --release
```

## Usage

```bash
cargo +nightly run --release -- <input_image> <output_gif> <encode_speed>
```

**[details about encode_speed](https://doc.servo.org/color_quant/struct.NeuQuant.html#method.new)**

![more visual performance line chart](img/speed_to_cpu-time.svg)

Note that **format of the input image must be the same as suffix name explains**,

otherwise you will get an error like `Bad Signature`!

## Todo

- [x] transparent background: [#1](https://github.com/poly000/petpet-rs/issues/1)

- [ ] Upload new version to crates.io: [#2](https://github.com/poly000/petpet-rs/issues/2)

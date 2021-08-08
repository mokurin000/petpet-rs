# petpet-rs

Also [petpet](https://github.com/camprevail/pet-pet-gif/), but in Rust.

## Usage

```bash
cargo run --release <input_image> <output_gif>
```

Note that **format of the input image must be the same as suffix name explains**,

otherwise you will get an error like `Bad Signature`!

## Todo

- [x] **<del>[RIIR](https://github.com/ansuz/RIIR)</del>** I rewrite Petpet
in Rust just due to Kotlin Version runs slowly and I don't know how to extract
[the code](https://github.com/poly000/BotPlugin/blob/master/src/main/kotlin/io/github/rosemoe/miraiPlugin/modules/PetPet.kt) generating petpet.

- [ ] transparent background (Needing help: idk how to make `Frame` in `image` seperate each frames.)
Now I just [added a white background](https://github.com/poly000/petpet-rs/blob/f637cf3f147340692e1d0fdb9055739af7f1a3b2/src/lib.rs#L52).

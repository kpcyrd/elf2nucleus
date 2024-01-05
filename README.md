# elf2nucleus

Integrate micronucleus into the cargo buildsystem, flash an AVR firmware from an elf file.

```toml
# ./.cargo/config.toml
[target.'cfg(target_arch = "avr")']
# Choose a default "cargo run" tool
runner = "elf2nucleus"
```

Adding this to your cargo config should then allow you to use the cargo-native **run** command for flashing, without further need for a Makefile:

```sh
RUSTC_BOOTSTRAP=1 cargo run --release
```

Please note that elf2nucleus builds on top of [micronucleus](https://github.com/micronucleus/micronucleus) which needs to be installed too.

This project was inspired by [elf2uf2-rs](https://github.com/JoNil/elf2uf2-rs) for the rp2040.

Also shoutout to [@cyber-murmel](https://github.com/cyber-murmel) who motivated me to develop this and helped me with AVR development.

[![](https://repology.org/badge/vertical-allrepos/elf2nucleus.svg)](https://repology.org/project/elf2nucleus/versions)

## Building firmware with cargo

To build an AVR firmware repository it's often enough to run:

```sh
RUSTC_BOOTSTRAP=1 cargo build --release
```

cargo then compiles the firmware and puts it into an elf container file like this:

```
target/avr-attiny85/release/attiny85-hello-world.elf
```

For this `.cargo/config.toml` needs to be configured correctly, an example repository can be found at [github.com/kpcyrd/attiny85-hello-world](https://github.com/kpcyrd/attiny85-hello-world).

## Flashing with elf2nucleus

To invoke elf2nucleus without cargo, you can run it like this:

```sh
elf2nucleus ./attiny85-hello-world.elf
```

If you don't want to flash with elf2nucleus and instead read the firmware from an elf file into a raw binary file you can specify an additional output file name:

```sh
elf2nucleus ./attiny85-hello-world.elf ./firmware.bin
micronucleus --type raw --run --no-ansi ./firmware.bin
```

## Using Rust with micronucleus directly

At the time of writing, it's not possible to configure micronucleus as "cargo run" tool, because micronucleus can't read from elf files.

If you want to avoid using elf2nucleus you can use `avr-objcopy` to prepare the firmware file yourself:

```sh
avr-objcopy --output-target=ihex target/avr-attiny85/release/attiny85-hello-world.elf target/avr-attiny85/release/attiny85-hello-world.hex
micronucleus --timeout 60 --run --no-ansi target/avr-attiny85/release/attiny85-hello-world.hex
```

## License

`GPL-3.0-or-later`

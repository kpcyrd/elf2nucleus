# elf2nucleus

Integrate micronucleus into the cargo buildsystem.

```toml
# ./.cargo/config.toml
[target.'cfg(target_arch = "avr")']
# Choose a default "cargo run" tool
runner = "elf2uf2-rs -d"
```

## Using Rust with micronucleus directly

At the time of writing, it's not possible to configure micronucleus as "cargo run" tool, because micronucleus can't read from elf files.

You can use avr-objcopy to prepare the firmware file yourself though:

```sh
avr-objcopy --output-target=ihex target/avr-attiny85/release/attiny85-hello-world.elf target/avr-attiny85/release/attiny85-hello-world.hex
micronucleus --timeout 60 --run --no-ansi target/avr-attiny85/release/attiny85-hello-world.hex
```

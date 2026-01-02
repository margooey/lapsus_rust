# A WIP re-write of [Lapsus](https://github.com/margooey/Lapsus) in rust using various crates for Apple framework bindings.

## What is it
Lapsus is an application designed to emulate the feeling of using a trackball. It applies "momentum" to your cursor so that it glides (or slides) across the screen until slowly coming to a stop. Lapsus was born out of Magnes, which was an application designed to emulate the iPadOS cursor as a whole.

## Crates used:
- cidre
- core-graphics
- objc2-app-kit
- objc2-foundation
- objc2-core-foundation
- macos-multitouch
- log
- env_logger
- chrono

## Download
You can download Lapsus_Rust on the [Releases](https://github.com/margooey/lapsus_rust/releases) page. You can also download any built artifacts from the [workflow](https://github.com/margooey/lapsus_rust/actions).

## Build
```shell
cargo build --release
```

## Debug
```shell
cargo run RUST_LOG=DEBUG
```

## License
Lapsus_Rust is licensed under a custom non-commercial license.
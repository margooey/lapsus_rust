# A WIP re-write of [Lapsus](https://github.com/margooey/Lapsus) in rust using various crates for Apple framework bindings.

## Crates used:
- cidre
- core-graphics
- objc2-app-kit
- objc2-core-foundation
- objc2
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
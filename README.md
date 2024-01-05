# camera_free

A [Skyline](https://github.com/skyline-dev/skyline) plugin for Super Smash Bros. Ultimate that frees the camera controls of its restrictions. This plugin is meant to be used alongside this [mod](https://gamebanana.com/mods/38659).

The latest release can be found [here](https://github.com/ThatNintendoNerd/camera_free/releases).

## Features

- Unlimited camera panning
- Unlimited camera rotation
- Unlimited camera zooming
- Unlimited camera tilting
- Full 180° viewing angle range
- Unrestricted VR camera controls

## Building

NOTE: This project cannot be compiled without the smash_stage library. Said library is unreleased due to its incomplete state, but its release is planned.

With an up-to-date version of the Rust toolchain installed and [cargo-skyline](https://github.com/jam1garner/cargo-skyline) 3.0.0 or newer, run the following command to compile the project in release mode:

```
cargo skyline build --release
```

The resulting build is found at `./target/aarch64-skyline-switch/release/libcamera_free.nro`

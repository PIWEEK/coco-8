# COCO-8

COCO-8 (or 👻-8) is a fantasy VM for games and creative coding. COCO-8 runs on Wasm and it's written in Rust.

COCO-8 is [made of these parts](./docs/architecture.md):

- `coco-core`, the virtual CPU. It is basically [Uxn](https://wiki.xxiivv.com/site/uxn.html), as _documented_ (COCO-8 follows the documented spec, not the reference implementation of Uxn).

- Devices, which interface with the CPU and provide input / output with peripherals or other systems (e.g. video, audio, controller input, etc.).

- A web-based GUI, for users to load and run COCO-8 roms.

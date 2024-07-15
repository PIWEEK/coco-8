# Use cargo to build the wasm module made in Rust
cargo build --release --target wasm32-unknown-unknown

# Build ESM modules in coco-ui
wasm-bindgen target/wasm32-unknown-unknown/release/coco_vm.wasm --out-dir coco-ui/vendor --target web --no-typescript


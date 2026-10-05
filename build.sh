cd hello-world && cargo build --release --target wasm32-wasip3 && cd ..
cd wasi-impl && cargo build --release --target wasm32-unknown-unknown && cd ..
cd run && cargo run --release && cd ..

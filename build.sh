pushd logger
cargo component build --release
popd
pushd target/wasm32-wasip1/release
wac plug logger.wasm --plug adder.wasm -o composed.wasm
popd


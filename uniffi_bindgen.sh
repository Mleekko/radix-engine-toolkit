bindgen() {
    cargo run \
        --manifest-path="../uniffi-bindgen/Cargo.toml" -- \
        generate ./src/radix_engine_toolkit_uniffi.udl \
        --no-format \
        --language $1 \
        --out-dir ./output \
        --lib-file ./target/debug/libradix_engine_toolkit_uniffi.a
}

cd radix-engine-toolkit-uniffi;
cargo build

bindgen java

uniffi-bindgen-cs src/radix_engine_toolkit_uniffi.udl --lib-file ./target/debug/libradix_engine_toolkit_uniffi.a --out-dir output

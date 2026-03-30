fn main() {
    // Tell cargo to emit a `wasm-bindgen` compatible binary
    println!("cargo:rerun-if-env-changed=WASM_BINDGEN");
}
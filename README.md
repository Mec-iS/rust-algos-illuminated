# rust-algos-illuminated

Implementations in Rust based on "Algorithms Illuminated" book

## Give it a try

* `cargo build`
* `cargo test` or `cargo bench`

## Run Rust-compiled functions from Python (via WebAssembly)

* `cargo build --release --target=wasm32-unknown-unknown` (you need the right Rust toolchain)
* `python3.6 -m pip install wasmer`
* `python3.6 run_with_wasmer.py`

### References

* [Build Rust to WASM](https://www.reddit.com/r/rust/comments/9t95fd/howto_setting_up_webassembly_on_stable_rust/):

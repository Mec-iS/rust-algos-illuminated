from wasmer import Instance

path = 'target/wasm32-unknown-unknown/release/rust-algos-illuminated.wasm'

with open(path, 'rb') as bytecode:
    wasm_bytes = bytecode.read()
    instance = Instance(wasm_bytes)
    result = instance.exports.RecIntMul(12, 12)

    print("Modules exported from Rust: ")
    print(instance.exports)
    print("call RecIntMul(12, 12): ")
    print(result)

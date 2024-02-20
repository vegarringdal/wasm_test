# wasm_test

Sample for issue:
https://github.com/RReverser/serde-wasm-bindgen/issues/63

Run
* clone `git clone https://github.com/vegarringdal/wasm_test`
* `npm install`
* `npm start`
* open browser at http://localhost:2080/ and open console


# wasm part

-   https://www.rust-lang.org/tools/install
-   https://rustwasm.github.io/wasm-pack/installer/ (cargo install --version 0.12.1 wasm-pack)

# new wasm build (Ive added builds, no need to run unless you edit)

-   wasm-pack build ./wasm1 --target web --release
-   wasm-pack build ./wasm2 --target web --release
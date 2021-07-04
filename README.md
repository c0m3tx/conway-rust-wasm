# WebAssembly Conway's game of life

A simple implementation of Life using Rust and WebAssembly.

## How to run

In order to build this application you need to install the [Rust WebAssembly compiler](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm) first.

    cargo install wasm-pack

Then you can build the project using

    wasm-pack build --target web

Finally, copy the generated `pkg` folder, `index.html` and `index.js` to a local webserver, or just run (for instance)

    python3 -m http.server

in the root folder.

## How to contribute

Feel free to contribute to the project by opening a PR!

## License

This project is licensed under the MIT - see the [LICENSE](https://github.com/c0m3tx/c0m3tx/conway-rust-wasm/blob/master/LICENSE) file for details

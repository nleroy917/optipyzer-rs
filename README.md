# Optipyzer
Optipyzer is a multi-species codon optimization engine. The [original implementation](https://github.com/nleroy917/optipyzer) was written in Python and is available on GitHub. I'm looking to port it to Rust for speed and reducing bugs. This is a work in progress. Ideally the server will remain in Python and the engine will be written in Rust.

The idea is to have three main components:
1. The core rust implementation,
2. A Python wrapper for the Rust implementation, and
3. wasm bindings for the Rust implementation.
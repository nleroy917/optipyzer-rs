mod utils;

use multimizer::optimizations::optimize_seq_test;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, multimizer-web!");
}

#[wasm_bindgen]
pub fn optimize(query: &str) -> String {
    optimize_seq_test(query)
}

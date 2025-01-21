mod models;
mod utils;


use multimizer::optimizations::{optimize_for_single_organism, OptimizationOptions};
use multimizer::utils::parse_fasta_sequences_from_string;

use wasm_bindgen::prelude::*;

use crate::models::{
    JsCodonUsage,
    JsOptimizationResult,
    ParsedFastaSequences
};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, multimizer-web!");
}

#[wasm_bindgen(js_name = "parseFastaSequencesFromString")]
pub fn parse_fasta_sequences_from_string_js(input: &str) -> Result<JsValue, JsValue> {
    match parse_fasta_sequences_from_string(input) {
        Ok(seqs) => {
            let parsed_seqs = ParsedFastaSequences { result: seqs };
            Ok(serde_wasm_bindgen::to_value(&parsed_seqs)?)
        }
        Err(err) => Err(JsValue::from(err.to_string())),
    }
}

#[wasm_bindgen(js_name = "optimizeSequence")]
pub fn optimize(query: &str, codon_usage: JsValue) -> Result<JsValue, JsValue> {
    let codon_usage: JsCodonUsage = serde_wasm_bindgen::from_value(codon_usage)?;
    let opts = OptimizationOptions::default();
    let res = optimize_for_single_organism(query, &codon_usage, &opts)?;

    Ok(serde_wasm_bindgen::to_value(&res))
}

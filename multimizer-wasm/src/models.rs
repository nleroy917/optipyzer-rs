use std::collections::HashMap;

use tsify::Tsify;
use serde::{Deserialize, Serialize};

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ParsedFastaSequences {
    pub result: HashMap<String, String>,
}

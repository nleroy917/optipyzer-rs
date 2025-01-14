use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ParsedFastaSequences {
    pub result: HashMap<String, String>,
}

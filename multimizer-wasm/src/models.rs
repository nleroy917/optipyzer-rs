use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tsify::Tsify;

type CodonUsageByResidue = HashMap<char, HashMap<multimizer::models::Codon, f64>>;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ParsedFastaSequences {
    pub result: HashMap<String, String>,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsOptimizationResult {
    pub seq: String,
    pub iterations: i32,
    pub translated_seq: String,
    pub rca_value: f64,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsCodonUsage {
    pub org_id: u32,
    pub ttt: u32,
    pub ttc: u32,
    pub tta: u32,
    pub ttg: u32,
    pub ctt: u32,
    pub ctc: u32,
    pub cta: u32,
    pub ctg: u32,
    pub att: u32,
    pub atc: u32,
    pub ata: u32,
    pub atg: u32,
    pub gtt: u32,
    pub gtc: u32,
    pub gta: u32,
    pub gtg: u32,
    pub tat: u32,
    pub tac: u32,
    pub taa: u32,
    pub tag: u32,
    pub cat: u32,
    pub cac: u32,
    pub caa: u32,
    pub cag: u32,
    pub aat: u32,
    pub aac: u32,
    pub aaa: u32,
    pub aag: u32,
    pub gat: u32,
    pub gac: u32,
    pub gaa: u32,
    pub gag: u32,
    pub tct: u32,
    pub tcc: u32,
    pub tca: u32,
    pub tcg: u32,
    pub cct: u32,
    pub ccc: u32,
    pub cca: u32,
    pub ccg: u32,
    pub act: u32,
    pub acc: u32,
    pub aca: u32,
    pub acg: u32,
    pub gct: u32,
    pub gcc: u32,
    pub gca: u32,
    pub gcg: u32,
    pub tgt: u32,
    pub tgc: u32,
    pub tga: u32,
    pub tgg: u32,
    pub cgt: u32,
    pub cgc: u32,
    pub cga: u32,
    pub cgg: u32,
    pub agt: u32,
    pub agc: u32,
    pub aga: u32,
    pub agg: u32,
    pub ggt: u32,
    pub ggc: u32,
    pub gga: u32,
    pub ggg: u32,
}

impl Into<CodonUsageByResidue> for JsCodonUsage {
    fn into(self) -> CodonUsageByResidue {
        let 
    }
}
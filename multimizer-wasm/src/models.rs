use std::collections::HashMap;

use multimizer::models::{CodonUsage, Codon};
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

impl From<JsCodonUsage> for CodonUsage {
    fn from(value: JsCodonUsage) -> Self {
        let mut codon_usage = HashMap::new();
        
        codon_usage.insert(Codon::TTT, value.ttt as i32);
        codon_usage.insert(Codon::TTC, value.ttc as i32);
        codon_usage.insert(Codon::TTA, value.tta as i32);
        codon_usage.insert(Codon::TTG, value.ttg as i32);
        codon_usage.insert(Codon::CTT, value.ctt as i32);
        codon_usage.insert(Codon::CTC, value.ctc as i32);
        codon_usage.insert(Codon::CTA, value.cta as i32);
        codon_usage.insert(Codon::CTG, value.ctg as i32);
        codon_usage.insert(Codon::ATT, value.att as i32);
        codon_usage.insert(Codon::ATC, value.atc as i32);
        codon_usage.insert(Codon::ATA, value.ata as i32);
        codon_usage.insert(Codon::ATG, value.atg as i32);
        codon_usage.insert(Codon::GTT, value.gtt as i32);
        codon_usage.insert(Codon::GTC, value.gtc as i32);
        codon_usage.insert(Codon::GTA, value.gta as i32);
        codon_usage.insert(Codon::GTG, value.gtg as i32);
        codon_usage.insert(Codon::TAT, value.tat as i32);
        codon_usage.insert(Codon::TAC, value.tac as i32);
        codon_usage.insert(Codon::TAA, value.taa as i32);
        codon_usage.insert(Codon::TAG, value.tag as i32);
        codon_usage.insert(Codon::CAT, value.cat as i32);
        codon_usage.insert(Codon::CAC, value.cac as i32);
        codon_usage.insert(Codon::CAA, value.caa as i32);
        codon_usage.insert(Codon::CAG, value.cag as i32);
        codon_usage.insert(Codon::AAT, value.aat as i32);
        codon_usage.insert(Codon::AAC, value.aac as i32);
        codon_usage.insert(Codon::AAA, value.aaa as i32);
        codon_usage.insert(Codon::AAG, value.aag as i32);
        codon_usage.insert(Codon::GAT, value.gat as i32);
        codon_usage.insert(Codon::GAC, value.gac as i32);
        codon_usage.insert(Codon::GAA, value.gaa as i32);
        codon_usage.insert(Codon::GAG, value.gag as i32);
        codon_usage.insert(Codon::TCT, value.tct as i32);
        codon_usage.insert(Codon::TCC, value.tcc as i32);
        codon_usage.insert(Codon::TCA, value.tca as i32);
        codon_usage.insert(Codon::TCG, value.tcg as i32);
        codon_usage.insert(Codon::CCT, value.cct as i32);
        codon_usage.insert(Codon::CCC, value.ccc as i32);
        codon_usage.insert(Codon::CCA, value.cca as i32);
        codon_usage.insert(Codon::CCG, value.ccg as i32);
        codon_usage.insert(Codon::ACT, value.act as i32);
        codon_usage.insert(Codon::ACC, value.acc as i32);
        codon_usage.insert(Codon::ACA, value.aca as i32);
        codon_usage.insert(Codon::ACG, value.acg as i32);
        codon_usage.insert(Codon::GCT, value.gct as i32);
        codon_usage.insert(Codon::GCC, value.gcc as i32);
        codon_usage.insert(Codon::GCA, value.gca as i32);
        codon_usage.insert(Codon::GCG, value.gcg as i32);
        codon_usage.insert(Codon::TGT, value.tgt as i32);
        codon_usage.insert(Codon::TGC, value.tgc as i32);
        codon_usage.insert(Codon::TGA, value.tga as i32);
        codon_usage.insert(Codon::TGG, value.tgg as i32);
        codon_usage.insert(Codon::CGT, value.cgt as i32);
        codon_usage.insert(Codon::CGC, value.cgc as i32);
        codon_usage.insert(Codon::CGA, value.cga as i32);
        codon_usage.insert(Codon::CGG, value.cgg as i32);
        codon_usage.insert(Codon::AGT, value.agt as i32);
        codon_usage.insert(Codon::AGC, value.agc as i32);
        codon_usage.insert(Codon::AGA, value.aga as i32);
        codon_usage.insert(Codon::AGG, value.agg as i32);
        codon_usage.insert(Codon::GGT, value.ggt as i32);
        codon_usage.insert(Codon::GGC, value.ggc as i32);
        codon_usage.insert(Codon::GGA, value.gga as i32);
        codon_usage.insert(Codon::GGG, value.ggg as i32);

        CodonUsage {
           codon_usage 
        }
    }   
}
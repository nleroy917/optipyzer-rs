use std::collections::HashMap;

pub type ProhibitedCodons = HashMap<char, Vec<String>>;
pub type Query = HashMap<String, AminoAcidMap>;
type AminoAcidMap = HashMap<char, CodonPreference>;
type CodonPreference = HashMap<String, f32>;


#[derive(Debug)]
pub struct Organism {
    org_id: i32,
    division: String,
    assembly: String,
    taxid: i32,
    species: String,
    organelle: String,
    translation_table: i32,
    num_cds: i32,
    num_codons: i32,
    gc_perc: f32,
    gc1_perc: f32,
    gc2_perc: f32,
    gc3_perc: f32,
}

#[derive(Debug)]
pub struct CodonUsage {
    pub org_id: i32,
    pub ttt: i32,
    pub ttc: i32,
    pub tta: i32,
    pub ttg: i32,
    pub ctt: i32,
    pub ctc: i32,
    pub cta: i32,
    pub ctg: i32,
    pub att: i32,
    pub atc: i32,
    pub ata: i32,
    pub atg: i32,
    pub gtt: i32,
    pub gtc: i32,
    pub gta: i32,
    pub gtg: i32,
    pub tat: i32,
    pub tac: i32,
    pub taa: i32,
    pub tag: i32,
    pub cat: i32,
    pub cac: i32,
    pub caa: i32,
    pub cag: i32,
    pub aat: i32,
    pub aac: i32,
    pub aaa: i32,
    pub aag: i32,
    pub gat: i32,
    pub gac: i32,
    pub gaa: i32,
    pub gag: i32,
    pub tct: i32,
    pub tcc: i32,
    pub tca: i32,
    pub tcg: i32,
    pub cct: i32,
    pub ccc: i32,
    pub cca: i32,
    pub ccg: i32,
    pub act: i32,
    pub acc: i32,
    pub aca: i32,
    pub acg: i32,
    pub gct: i32,
    pub gcc: i32,
    pub gca: i32,
    pub gcg: i32,
    pub tgt: i32,
    pub tgc: i32,
    pub tga: i32,
    pub tgg: i32,
    pub cgt: i32,
    pub cgc: i32,
    pub cga: i32,
    pub cgg: i32,
    pub agt: i32,
    pub agc: i32,
    pub aga: i32,
    pub agg: i32,
    pub ggt: i32,
    pub ggc: i32,
    pub gga: i32,
    pub ggg: i32,
}

use std::convert::TryFrom;
use std::{collections::HashMap, fmt::Display};

pub type ProhibitedCodons = HashMap<char, Vec<Codon>>;

#[derive(Debug)]
pub struct Organism {
    pub org_id: i32,
    pub division: String,
    pub assembly: String,
    pub taxid: i32,
    pub species: String,
    pub organelle: String,
    pub translation_table: i32,
    pub num_cds: i32,
    pub num_codons: i32,
    pub gc_perc: f32,
    pub gc1_perc: f32,
    pub gc2_perc: f32,
    pub gc3_perc: f32,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Codon {
    AAA,
    AAC,
    AAG,
    AAT,
    ACA,
    ACC,
    ACG,
    ACT,
    AGA,
    AGC,
    AGG,
    AGT,
    ATA,
    ATC,
    ATG,
    ATT,
    CAA,
    CAC,
    CAG,
    CAT,
    CCA,
    CCC,
    CCG,
    CCT,
    CGA,
    CGC,
    CGG,
    CGT,
    CTA,
    CTC,
    CTG,
    CTT,
    GAA,
    GAC,
    GAG,
    GAT,
    GCA,
    GCC,
    GCG,
    GCT,
    GGA,
    GGC,
    GGG,
    GGT,
    GTA,
    GTC,
    GTG,
    GTT,
    TAA,
    TAC,
    TAG,
    TAT,
    TCA,
    TCC,
    TCG,
    TCT,
    TGA,
    TGC,
    TGG,
    TGT,
    TTA,
    TTC,
    TTG,
    TTT,
}

impl TryFrom<&str> for Codon {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = s.to_uppercase();
        match s.as_str() {
            "AAA" => Ok(Codon::AAA),
            "AAC" => Ok(Codon::AAC),
            "AAG" => Ok(Codon::AAG),
            "AAT" => Ok(Codon::AAT),
            "ACA" => Ok(Codon::ACA),
            "ACC" => Ok(Codon::ACC),
            "ACG" => Ok(Codon::ACG),
            "ACT" => Ok(Codon::ACT),
            "AGA" => Ok(Codon::AGA),
            "AGC" => Ok(Codon::AGC),
            "AGG" => Ok(Codon::AGG),
            "AGT" => Ok(Codon::AGT),
            "ATA" => Ok(Codon::ATA),
            "ATC" => Ok(Codon::ATC),
            "ATG" => Ok(Codon::ATG),
            "ATT" => Ok(Codon::ATT),
            "CAA" => Ok(Codon::CAA),
            "CAC" => Ok(Codon::CAC),
            "CAG" => Ok(Codon::CAG),
            "CAT" => Ok(Codon::CAT),
            "CCA" => Ok(Codon::CCA),
            "CCC" => Ok(Codon::CCC),
            "CCG" => Ok(Codon::CCG),
            "CCT" => Ok(Codon::CCT),
            "CGA" => Ok(Codon::CGA),
            "CGC" => Ok(Codon::CGC),
            "CGG" => Ok(Codon::CGG),
            "CGT" => Ok(Codon::CGT),
            "CTA" => Ok(Codon::CTA),
            "CTC" => Ok(Codon::CTC),
            "CTG" => Ok(Codon::CTG),
            "CTT" => Ok(Codon::CTT),
            "GAA" => Ok(Codon::GAA),
            "GAC" => Ok(Codon::GAC),
            "GAG" => Ok(Codon::GAG),
            "GAT" => Ok(Codon::GAT),
            "GCA" => Ok(Codon::GCA),
            "GCC" => Ok(Codon::GCC),
            "GCG" => Ok(Codon::GCG),
            "GCT" => Ok(Codon::GCT),
            "GGA" => Ok(Codon::GGA),
            "GGC" => Ok(Codon::GGC),
            "GGG" => Ok(Codon::GGG),
            "GGT" => Ok(Codon::GGT),
            "GTA" => Ok(Codon::GTA),
            "GTC" => Ok(Codon::GTC),
            "GTG" => Ok(Codon::GTG),
            "GTT" => Ok(Codon::GTT),
            "TAA" => Ok(Codon::TAA),
            "TAC" => Ok(Codon::TAC),
            "TAG" => Ok(Codon::TAG),
            "TAT" => Ok(Codon::TAT),
            "TCA" => Ok(Codon::TCA),
            "TCC" => Ok(Codon::TCC),
            "TCG" => Ok(Codon::TCG),
            "TCT" => Ok(Codon::TCT),
            "TGA" => Ok(Codon::TGA),
            "TGC" => Ok(Codon::TGC),
            "TGG" => Ok(Codon::TGG),
            "TGT" => Ok(Codon::TGT),
            "TTA" => Ok(Codon::TTA),
            "TTC" => Ok(Codon::TTC),
            "TTG" => Ok(Codon::TTG),
            "TTT" => Ok(Codon::TTT),
            _ => Err(format!("Invalid codon: {}", s)),
        }
    }
}

impl Display for Codon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Codon::AAA => write!(f, "AAA"),
            Codon::AAC => write!(f, "AAC"),
            Codon::AAG => write!(f, "AAG"),
            Codon::AAT => write!(f, "AAT"),
            Codon::ACA => write!(f, "ACA"),
            Codon::ACC => write!(f, "ACC"),
            Codon::ACG => write!(f, "ACG"),
            Codon::ACT => write!(f, "ACT"),
            Codon::AGA => write!(f, "AGA"),
            Codon::AGC => write!(f, "AGC"),
            Codon::AGG => write!(f, "AGG"),
            Codon::AGT => write!(f, "AGT"),
            Codon::ATA => write!(f, "ATA"),
            Codon::ATC => write!(f, "ATC"),
            Codon::ATG => write!(f, "ATG"),
            Codon::ATT => write!(f, "ATT"),
            Codon::CAA => write!(f, "CAA"),
            Codon::CAC => write!(f, "CAC"),
            Codon::CAG => write!(f, "CAG"),
            Codon::CAT => write!(f, "CAT"),
            Codon::CCA => write!(f, "CCA"),
            Codon::CCC => write!(f, "CCC"),
            Codon::CCG => write!(f, "CCG"),
            Codon::CCT => write!(f, "CCT"),
            Codon::CGA => write!(f, "CGA"),
            Codon::CGC => write!(f, "CGC"),
            Codon::CGG => write!(f, "CGG"),
            Codon::CGT => write!(f, "CGT"),
            Codon::CTA => write!(f, "CTA"),
            Codon::CTC => write!(f, "CTC"),
            Codon::CTG => write!(f, "CTG"),
            Codon::CTT => write!(f, "CTT"),
            Codon::GAA => write!(f, "GAA"),
            Codon::GAC => write!(f, "GAC"),
            Codon::GAG => write!(f, "GAG"),
            Codon::GAT => write!(f, "GAT"),
            Codon::GCA => write!(f, "GCA"),
            Codon::GCC => write!(f, "GCC"),
            Codon::GCG => write!(f, "GCG"),
            Codon::GCT => write!(f, "GCT"),
            Codon::GGA => write!(f, "GGA"),
            Codon::GGC => write!(f, "GGC"),
            Codon::GGG => write!(f, "GGG"),
            Codon::GGT => write!(f, "GGT"),
            Codon::GTA => write!(f, "GTA"),
            Codon::GTC => write!(f, "GTC"),
            Codon::GTG => write!(f, "GTG"),
            Codon::GTT => write!(f, "GTT"),
            Codon::TAA => write!(f, "TAA"),
            Codon::TAC => write!(f, "TAC"),
            Codon::TAG => write!(f, "TAG"),
            Codon::TAT => write!(f, "TAT"),
            Codon::TCA => write!(f, "TCA"),
            Codon::TCC => write!(f, "TCC"),
            Codon::TCG => write!(f, "TCG"),
            Codon::TCT => write!(f, "TCT"),
            Codon::TGA => write!(f, "TGA"),
            Codon::TGC => write!(f, "TGC"),
            Codon::TGG => write!(f, "TGG"),
            Codon::TGT => write!(f, "TGT"),
            Codon::TTA => write!(f, "TTA"),
            Codon::TTC => write!(f, "TTC"),
            Codon::TTG => write!(f, "TTG"),
            Codon::TTT => write!(f, "TTT"),
        }
    }
}

#[derive(Debug)]
pub struct CodonUsage {
    codon_usage: HashMap<Codon, i32>,
}

#[allow(clippy::too_many_arguments)] // better way than just enumerating all codons?
impl CodonUsage {
    pub fn new(
        ttt: i32,
        ttc: i32,
        tta: i32,
        ttg: i32,
        ctt: i32,
        ctc: i32,
        cta: i32,
        ctg: i32,
        att: i32,
        atc: i32,
        ata: i32,
        atg: i32,
        gtt: i32,
        gtc: i32,
        gta: i32,
        gtg: i32,
        tat: i32,
        tac: i32,
        taa: i32,
        tag: i32,
        cat: i32,
        cac: i32,
        caa: i32,
        cag: i32,
        aat: i32,
        aac: i32,
        aaa: i32,
        aag: i32,
        gat: i32,
        gac: i32,
        gaa: i32,
        gag: i32,
        tct: i32,
        tcc: i32,
        tca: i32,
        tcg: i32,
        cct: i32,
        ccc: i32,
        cca: i32,
        ccg: i32,
        act: i32,
        acc: i32,
        aca: i32,
        acg: i32,
        gct: i32,
        gcc: i32,
        gca: i32,
        gcg: i32,
        tgt: i32,
        tgc: i32,
        tga: i32,
        tgg: i32,
        cgt: i32,
        cgc: i32,
        cga: i32,
        cgg: i32,
        agt: i32,
        agc: i32,
        aga: i32,
        agg: i32,
        ggt: i32,
        ggc: i32,
        gga: i32,
        ggg: i32,
    ) -> CodonUsage {
        let mut codon_usage = HashMap::new();

        codon_usage.insert(Codon::AAA, aaa);
        codon_usage.insert(Codon::AAC, aac);
        codon_usage.insert(Codon::AAG, aag);
        codon_usage.insert(Codon::AAT, aat);
        codon_usage.insert(Codon::ACA, aca);
        codon_usage.insert(Codon::ACC, acc);
        codon_usage.insert(Codon::ACG, acg);
        codon_usage.insert(Codon::ACT, act);
        codon_usage.insert(Codon::AGA, aga);
        codon_usage.insert(Codon::AGC, agc);
        codon_usage.insert(Codon::AGG, agg);
        codon_usage.insert(Codon::AGT, agt);
        codon_usage.insert(Codon::ATA, ata);
        codon_usage.insert(Codon::ATC, atc);
        codon_usage.insert(Codon::ATG, atg);
        codon_usage.insert(Codon::ATT, att);
        codon_usage.insert(Codon::CAA, caa);
        codon_usage.insert(Codon::CAC, cac);
        codon_usage.insert(Codon::CAG, cag);
        codon_usage.insert(Codon::CAT, cat);
        codon_usage.insert(Codon::CCA, cca);
        codon_usage.insert(Codon::CCC, ccc);
        codon_usage.insert(Codon::CCG, ccg);
        codon_usage.insert(Codon::CCT, cct);
        codon_usage.insert(Codon::CGA, cga);
        codon_usage.insert(Codon::CGC, cgc);
        codon_usage.insert(Codon::CGG, cgg);
        codon_usage.insert(Codon::CGT, cgt);
        codon_usage.insert(Codon::CTA, cta);
        codon_usage.insert(Codon::CTC, ctc);
        codon_usage.insert(Codon::CTG, ctg);
        codon_usage.insert(Codon::CTT, ctt);
        codon_usage.insert(Codon::GAA, gaa);
        codon_usage.insert(Codon::GAC, gac);
        codon_usage.insert(Codon::GAG, gag);
        codon_usage.insert(Codon::GAT, gat);
        codon_usage.insert(Codon::GCA, gca);
        codon_usage.insert(Codon::GCC, gcc);
        codon_usage.insert(Codon::GCG, gcg);
        codon_usage.insert(Codon::GCT, gct);
        codon_usage.insert(Codon::GGA, gga);
        codon_usage.insert(Codon::GGC, ggc);
        codon_usage.insert(Codon::GGG, ggg);
        codon_usage.insert(Codon::GGT, ggt);
        codon_usage.insert(Codon::GTA, gta);
        codon_usage.insert(Codon::GTC, gtc);
        codon_usage.insert(Codon::GTG, gtg);
        codon_usage.insert(Codon::GTT, gtt);
        codon_usage.insert(Codon::TAA, taa);
        codon_usage.insert(Codon::TAC, tac);
        codon_usage.insert(Codon::TAG, tag);
        codon_usage.insert(Codon::TAT, tat);
        codon_usage.insert(Codon::TCA, tca);
        codon_usage.insert(Codon::TCC, tcc);
        codon_usage.insert(Codon::TCG, tcg);
        codon_usage.insert(Codon::TCT, tct);
        codon_usage.insert(Codon::TGA, tga);
        codon_usage.insert(Codon::TGC, tgc);
        codon_usage.insert(Codon::TGG, tgg);
        codon_usage.insert(Codon::TGT, tgt);
        codon_usage.insert(Codon::TTA, tta);
        codon_usage.insert(Codon::TTC, ttc);
        codon_usage.insert(Codon::TTG, ttg);
        codon_usage.insert(Codon::TTT, ttt);

        CodonUsage { codon_usage }
    }

    pub fn get(&self, codon: &Codon) -> Option<i32> {
        self.codon_usage.get(codon).copied()
    }
}

impl IntoIterator for CodonUsage {
    type Item = (Codon, i32);
    type IntoIter = std::collections::hash_map::IntoIter<Codon, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.codon_usage.into_iter()
    }
}

impl Display for CodonUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (codon, count) in &self.codon_usage {
            writeln!(f, "{}: {}", codon, count)?;
        }
        Ok(())
    }
}

impl CodonUsage {
    pub fn into_fracs(self) -> HashMap<Codon, f32> {
        let total: i32 = self.codon_usage.values().sum();
        self.codon_usage
            .into_iter()
            .map(|(codon, count)| (codon, count as f32 / total as f32))
            .collect()
    }
}

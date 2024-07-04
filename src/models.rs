use std::collections::HashMap;

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
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

impl From<&str> for Codon {
    fn from(s: &str) -> Self {
        let s = s.to_uppercase();
        match s.to_string().as_str() {
            "AAA" => Codon::AAA,
            "AAC" => Codon::AAC,
            "AAG" => Codon::AAG,
            "AAT" => Codon::AAT,
            "ACA" => Codon::ACA,
            "ACC" => Codon::ACC,
            "ACG" => Codon::ACG,
            "ACT" => Codon::ACT,
            "AGA" => Codon::AGA,
            "AGC" => Codon::AGC,
            "AGG" => Codon::AGG,
            "AGT" => Codon::AGT,
            "ATA" => Codon::ATA,
            "ATC" => Codon::ATC,
            "ATG" => Codon::ATG,
            "ATT" => Codon::ATT,
            "CAA" => Codon::CAA,
            "CAC" => Codon::CAC,
            "CAG" => Codon::CAG,
            "CAT" => Codon::CAT,
            "CCA" => Codon::CCA,
            "CCC" => Codon::CCC,
            "CCG" => Codon::CCG,
            "CCT" => Codon::CCT,
            "CGA" => Codon::CGA,
            "CGC" => Codon::CGC,
            "CGG" => Codon::CGG,
            "CGT" => Codon::CGT,
            "CTA" => Codon::CTA,
            "CTC" => Codon::CTC,
            "CTG" => Codon::CTG,
            "CTT" => Codon::CTT,
            "GAA" => Codon::GAA,
            "GAC" => Codon::GAC,
            "GAG" => Codon::GAG,
            "GAT" => Codon::GAT,
            "GCA" => Codon::GCA,
            "GCC" => Codon::GCC,
            "GCG" => Codon::GCG,
            "GCT" => Codon::GCT,
            "GGA" => Codon::GGA,
            "GGC" => Codon::GGC,
            "GGG" => Codon::GGG,
            "GGT" => Codon::GGT,
            "GTA" => Codon::GTA,
            "GTC" => Codon::GTC,
            "GTG" => Codon::GTG,
            "GTT" => Codon::GTT,
            "TAA" => Codon::TAA,
            "TAC" => Codon::TAC,
            "TAG" => Codon::TAG,
            "TAT" => Codon::TAT,
            "TCA" => Codon::TCA,
            "TCC" => Codon::TCC,
            "TCG" => Codon::TCG,
            "TCT" => Codon::TCT,
            "TGA" => Codon::TGA,
            "TGC" => Codon::TGC,
            "TGG" => Codon::TGG,
            "TGT" => Codon::TGT,
            "TTA" => Codon::TTA,
            "TTC" => Codon::TTC,
            "TTG" => Codon::TTG,
            "TTT" => Codon::TTT,
            _ => panic!("Invalid codon: {}", s),
        }
    }
}

impl Codon {
    pub fn to_string(&self) -> String {
        let v = match self {
            Codon::AAA => "AAA",
            Codon::AAC => "AAC",
            Codon::AAG => "AAG",
            Codon::AAT => "AAT",
            Codon::ACA => "ACA",
            Codon::ACC => "ACC",
            Codon::ACG => "ACG",
            Codon::ACT => "ACT",
            Codon::AGA => "AGA",
            Codon::AGC => "AGC",
            Codon::AGG => "AGG",
            Codon::AGT => "AGT",
            Codon::ATA => "ATA",
            Codon::ATC => "ATC",
            Codon::ATG => "ATG",
            Codon::ATT => "ATT",
            Codon::CAA => "CAA",
            Codon::CAC => "CAC",
            Codon::CAG => "CAG",
            Codon::CAT => "CAT",
            Codon::CCA => "CCA",
            Codon::CCC => "CCC",
            Codon::CCG => "CCG",
            Codon::CCT => "CCT",
            Codon::CGA => "CGA",
            Codon::CGC => "CGC",
            Codon::CGG => "CGG",
            Codon::CGT => "CGT",
            Codon::CTA => "CTA",
            Codon::CTC => "CTC",
            Codon::CTG => "CTG",
            Codon::CTT => "CTT",
            Codon::GAA => "GAA",
            Codon::GAC => "GAC",
            Codon::GAG => "GAG",
            Codon::GAT => "GAT",
            Codon::GCA => "GCA",
            Codon::GCC => "GCC",
            Codon::GCG => "GCG",
            Codon::GCT => "GCT",
            Codon::GGA => "GGA",
            Codon::GGC => "GGC",
            Codon::GGG => "GGG",
            Codon::GGT => "GGT",
            Codon::GTA => "GTA",
            Codon::GTC => "GTC",
            Codon::GTG => "GTG",
            Codon::GTT => "GTT",
            Codon::TAA => "TAA",
            Codon::TAC => "TAC",
            Codon::TAG => "TAG",
            Codon::TAT => "TAT",
            Codon::TCA => "TCA",
            Codon::TCC => "TCC",
            Codon::TCG => "TCG",
            Codon::TCT => "TCT",
            Codon::TGA => "TGA",
            Codon::TGC => "TGC",
            Codon::TGG => "TGG",
            Codon::TGT => "TGT",
            Codon::TTA => "TTA",
            Codon::TTC => "TTC",
            Codon::TTG => "TTG",
            Codon::TTT => "TTT",
        };

        v.to_string()
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

    pub fn get(&self, codon: &Codon) -> i32 {
        self.codon_usage.get(codon).unwrap().to_owned()
    }
}

impl IntoIterator for CodonUsage {
    type Item = (Codon, i32);
    type IntoIter = std::collections::hash_map::IntoIter<Codon, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.codon_usage.into_iter()
    }
}

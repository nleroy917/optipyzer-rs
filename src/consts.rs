use std::collections::HashMap;

use crate::models::Codon;

pub const DEFAULT_THRESHOLD: f32 = 0.1;
pub const DEFAULT_ITERATIONS: i32 = 1000;
pub const VALID_AMINO_ACIDS: &str = "ACDEFGHIKLMNPQRSTVWY*";
pub const VALID_NUCLEOTIDES: &str = "ACGT";

pub struct NumCodonsByAA {
    pub num_codons: HashMap<char, i32>,
}

impl NumCodonsByAA {
    pub fn new() -> NumCodonsByAA {
        let num_codons = HashMap::from([
            ('A', 4),
            ('R', 6),
            ('N', 2),
            ('D', 2),
            ('C', 2),
            ('Q', 2),
            ('E', 2),
            ('G', 4),
            ('H', 2),
            ('I', 3),
            ('L', 6),
            ('K', 2),
            ('F', 2),
            ('P', 4),
            ('S', 6),
            ('T', 4),
            ('Y', 2),
            ('V', 4),
            ('*', 3),
        ]);

        NumCodonsByAA { num_codons }
    }
}

impl Default for NumCodonsByAA {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AACodonLibrary {
    map: HashMap<char, Vec<Codon>>,
}

impl AACodonLibrary {
    pub fn new() -> AACodonLibrary {
        let mut map = HashMap::new();

        map.insert('A', vec![Codon::GCT, Codon::GCC, Codon::GCA, Codon::GCG]);
        map.insert(
            'R',
            vec![
                Codon::CGT,
                Codon::CGC,
                Codon::CGA,
                Codon::CGG,
                Codon::AGA,
                Codon::AGG,
            ],
        );
        map.insert('N', vec![Codon::AAT, Codon::AAC]);
        map.insert('D', vec![Codon::GAT, Codon::GAC]);
        map.insert('C', vec![Codon::TGT, Codon::TGC]);
        map.insert('Q', vec![Codon::CAA, Codon::CAG]);
        map.insert('E', vec![Codon::GAA, Codon::GAG]);
        map.insert('G', vec![Codon::GGT, Codon::GGC, Codon::GGA, Codon::GGG]);
        map.insert('H', vec![Codon::CAT, Codon::CAC]);
        map.insert('I', vec![Codon::ATT, Codon::ATC, Codon::ATA]);
        map.insert(
            'L',
            vec![
                Codon::TTA,
                Codon::TTG,
                Codon::CTT,
                Codon::CTC,
                Codon::CTA,
                Codon::CTG,
            ],
        );
        map.insert('K', vec![Codon::AAA, Codon::AAG]);
        map.insert('M', vec![Codon::ATG]);
        map.insert('F', vec![Codon::TTT, Codon::TTC]);
        map.insert('P', vec![Codon::CCT, Codon::CCC, Codon::CCA, Codon::CCG]);
        map.insert(
            'S',
            vec![
                Codon::TCT,
                Codon::TCC,
                Codon::TCA,
                Codon::TCG,
                Codon::AGT,
                Codon::AGC,
            ],
        );
        map.insert('T', vec![Codon::ACT, Codon::ACC, Codon::ACA, Codon::ACG]);
        map.insert('W', vec![Codon::TGG]);
        map.insert('Y', vec![Codon::TAT, Codon::TAC]);
        map.insert('V', vec![Codon::GTT, Codon::GTC, Codon::GTA, Codon::GTG]);
        map.insert('*', vec![Codon::TAA, Codon::TAG, Codon::TGA]);

        AACodonLibrary { map }
    }

    pub fn get(&self, aa: char) -> Vec<Codon> {
        self.map.get(&aa).unwrap().to_vec()
    }
}

impl IntoIterator for AACodonLibrary {
    type Item = (char, Vec<Codon>);
    type IntoIter = std::collections::hash_map::IntoIter<char, Vec<Codon>>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl Default for AACodonLibrary {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CodonToAA {
    map: HashMap<Codon, char>,
}

impl CodonToAA {
    pub fn new() -> CodonToAA {
        let map = HashMap::from([
            (Codon::ATA, 'I'),
            (Codon::ATC, 'I'),
            (Codon::ATT, 'I'),
            (Codon::ATG, 'M'),
            (Codon::ACA, 'T'),
            (Codon::ACC, 'T'),
            (Codon::ACG, 'T'),
            (Codon::ACT, 'T'),
            (Codon::AAC, 'N'),
            (Codon::AAT, 'N'),
            (Codon::AAA, 'K'),
            (Codon::AAG, 'K'),
            (Codon::AGC, 'S'),
            (Codon::AGT, 'S'),
            (Codon::AGA, 'R'),
            (Codon::AGG, 'R'),
            (Codon::CTA, 'L'),
            (Codon::CTC, 'L'),
            (Codon::CTG, 'L'),
            (Codon::CTT, 'L'),
            (Codon::CCA, 'P'),
            (Codon::CCC, 'P'),
            (Codon::CCG, 'P'),
            (Codon::CCT, 'P'),
            (Codon::CAC, 'H'),
            (Codon::CAT, 'H'),
            (Codon::CAA, 'Q'),
            (Codon::CAG, 'Q'),
            (Codon::CGA, 'R'),
            (Codon::CGC, 'R'),
            (Codon::CGG, 'R'),
            (Codon::CGT, 'R'),
            (Codon::GTA, 'V'),
            (Codon::GTC, 'V'),
            (Codon::GTG, 'V'),
            (Codon::GTT, 'V'),
            (Codon::GCA, 'A'),
            (Codon::GCC, 'A'),
            (Codon::GCG, 'A'),
            (Codon::GCT, 'A'),
            (Codon::GAC, 'D'),
            (Codon::GAT, 'D'),
            (Codon::GAA, 'E'),
            (Codon::GAG, 'E'),
            (Codon::GGA, 'G'),
            (Codon::GGC, 'G'),
            (Codon::GGG, 'G'),
            (Codon::GGT, 'G'),
            (Codon::TCA, 'S'),
            (Codon::TCC, 'S'),
            (Codon::TCG, 'S'),
            (Codon::TCT, 'S'),
            (Codon::TTC, 'F'),
            (Codon::TTT, 'F'),
            (Codon::TTA, 'L'),
            (Codon::TTG, 'L'),
            (Codon::TAC, 'Y'),
            (Codon::TAT, 'Y'),
            (Codon::TAA, '_'),
            (Codon::TAG, '_'),
            (Codon::TGC, 'C'),
            (Codon::TGT, 'C'),
            (Codon::TGA, '_'),
            (Codon::TGG, 'W'),
        ]);

        CodonToAA { map }
    }

    pub fn convert(&self, codon: &Codon) -> Option<char> {
        self.map.get(codon).copied()
    }
}

impl Default for CodonToAA {
    fn default() -> Self {
        Self::new()
    }
}
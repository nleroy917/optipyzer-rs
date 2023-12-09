use std::collections::HashMap;

pub const DEFAULT_THRESHOLD: f32 = 0.1;
pub const DEFAULT_ITERATIONS: i32 = 1000;

pub struct AACodonLibrary {
    map: HashMap<char, Vec<&'static str>>
}

impl AACodonLibrary {
    pub fn new() -> AACodonLibrary {
        let mut map = HashMap::new();

        map.insert('A', vec!["GCT", "GCC", "GCA", "GCG"]);
        map.insert('R', vec!["CGT", "CGC", "CGA", "CGG", "AGA", "AGG"]);
        map.insert('N', vec!["AAT", "AAC"]);
        map.insert('D', vec!["GAT", "GAC"]);
        map.insert('C', vec!["TGT", "TGC"]);
        map.insert('Q', vec!["CAA", "CAG"]);
        map.insert('E', vec!["GAA", "GAG"]);
        map.insert('G', vec!["GGT", "GGC", "GGA", "GGG"]);
        map.insert('H', vec!["CAT", "CAC"]);
        map.insert('I', vec!["ATT", "ATC", "ATA"]);
        map.insert('L', vec!["TTA", "TTG", "CTT", "CTC", "CTA", "CTG"]);
        map.insert('K', vec!["AAA", "AAG"]);
        map.insert('M', vec!["ATG"]);
        map.insert('F', vec!["TTT", "TTC"]);
        map.insert('P', vec!["CCT", "CCC", "CCA", "CCG"]);
        map.insert('S', vec!["TCT", "TCC", "TCA", "TCG", "AGT", "AGC"]);
        map.insert('T', vec!["ACT", "ACC", "ACA", "ACG"]);
        map.insert('W', vec!["TGG"]);
        map.insert('Y', vec!["TAT", "TAC"]);
        map.insert('V', vec!["GTT", "GTC", "GTA", "GTG"]);
        map.insert('*', vec!["TAA", "TAG", "TGA"]);

        AACodonLibrary { map }
    }

    pub fn get(&self, aa: char) -> Vec<&'static str> {
        self.map.get(&aa).unwrap().to_vec()
    }
}
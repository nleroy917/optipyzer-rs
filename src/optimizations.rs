use std::collections::HashMap;

use crate::models::{Query, ProhibitedCodons};

pub fn find_prohibited_codons(query: &Query, threshold: f32) -> ProhibitedCodons {

    let mut prohibited_codons: ProhibitedCodons = HashMap::new();

    for (_, amino_acid_map) in query {
        for (aa, codon_preference) in amino_acid_map {
            for (codon, preference) in codon_preference {
                if preference < &threshold {
                    prohibited_codons
                    .entry(*aa)
                    .or_insert(vec![])
                    .push(codon.to_string())
                }
            }
        }
    }

    prohibited_codons
}
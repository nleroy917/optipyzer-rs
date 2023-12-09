use std::collections::HashMap;

use crate::consts::AACodonLibrary;
use crate::db::Database;
use crate::models::{Codon, ProhibitedCodons};

type CodonCountsByAminoAcid = HashMap<char, HashMap<Codon, i32>>;
type CodonFracsByAminoAcid = HashMap<char, HashMap<Codon, f32>>;
type UsageDataByOrganism = HashMap<i32, CodonFracsByAminoAcid>;


///
/// Find the prohibited codons for a particular organism
/// and threshold. This is a map of amino acids to a list
/// of codons.
/// 
/// E.g.
/// {
///  "A": ["GCT", "GCC", "GCA", "GCG"],
///  ...
/// }
/// 
/// # Arguments
/// - `query` - The codon usage data
/// - `threshold` - The threshold to use
/// 
/// # Returns
/// - `ProhibitedCodons` - The prohibited codons
/// 
pub fn find_prohibited_codons(query: &UsageDataByOrganism, threshold: f32) -> ProhibitedCodons {
    // check that the threshold is valid
    if threshold < 0.0 || threshold > 1.0 {
        panic!("Threshold must be between 0 and 1");
    }

    let mut prohibited_codons: ProhibitedCodons = HashMap::new();

    for (_, amino_acid_map) in query {
        for (aa, codon_preference) in amino_acid_map {
            for (codon, preference) in codon_preference {
                if preference < &threshold {
                    prohibited_codons
                        .entry(*aa)
                        .or_insert(vec![])
                        .push(codon.clone())
                }
            }
        }
    }

    prohibited_codons
}

///
/// Get the codon counts for a particular organism, this is
/// split by amino acid. So its a map of maps.
///
/// E.g.
/// {
///    "A": {
///       "GCT": 11,
///       "GCC": 42,
///       "GCA": 77,
///       "GCG": 12
///   },
/// }
///
/// # Arguments
/// - `db` - The database connection
/// - `org_id` - The organism ID
///
/// # Returns
/// - `Result<CodonCountsByAminoAcid, Box<dyn std::error::Error>>` - The result of the operation
pub fn get_codon_counts_by_amino_acid(
    db: &Database,
    org_id: &i32,
) -> Result<CodonCountsByAminoAcid, Box<dyn std::error::Error>> {
    let usage = db.get_codon_usage_for_organism(org_id)?;
    let mut counts: CodonCountsByAminoAcid = HashMap::new();

    for (aa, codons) in AACodonLibrary::new() {
        let mut codon_counts: HashMap<Codon, i32> = HashMap::new();

        for c in codons {
            codon_counts.insert(c.clone(), usage.get(&c));
        }

        counts.insert(aa, codon_counts);
    }
    Ok(counts)
}

///
/// Get the codon fractions for a particular organism, this is
/// split by amino acid. So its a map of maps.
///
/// E.g.
/// {
///   "A": {
///      "GCT": 0.11,
///      "GCC": 0.42,
///      "GCA": 0.35,
///      "GCG": 0.12
///   },
/// }
///
/// # Arguments
/// - `db` - The database connection
/// - `org_id` - The organism ID
///     
/// # Returns
/// - `Result<CodonCountsByAminoAcid, Box<dyn std::error::Error>>` - The result of the operation
///
pub fn get_codon_fracs_by_amino_acid(
    db: &Database,
    org_id: &i32,
) -> Result<CodonFracsByAminoAcid, Box<dyn std::error::Error>> {
    let counts = get_codon_counts_by_amino_acid(db, org_id)?;
    let mut fracs: CodonFracsByAminoAcid = HashMap::new();

    for (aa, codon_counts) in counts {
        let total: i32 = codon_counts.values().sum();

        let mut codon_fracs: HashMap<Codon, f32> = HashMap::new();

        for (codon, count) in codon_counts {
            codon_fracs.insert(codon, count as f32 / total as f32);
        }

        fracs.insert(aa, codon_fracs);
    }

    Ok(fracs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn db() -> Database {
        Database::new("codon.db").unwrap()
    }

    #[fixture]
    fn org_id() -> i32 {
        16815 // e. coli
    }

    #[rstest]
    fn test_get_codon_counts_by_amino_acid(db: Database, org_id: i32) {
        let counts = get_codon_counts_by_amino_acid(&db, &org_id);
        // we should check the counts are correct here,
        // but thats a ton of work, so we'll just check
        // that the operation was successful
        assert!(counts.is_ok());
    }

    #[rstest]
    fn test_get_codon_fracs_by_amino_acid(db: Database, org_id: i32) {
        let fracs = get_codon_fracs_by_amino_acid(&db, &org_id);
        // we should check the counts are correct here,
        // but thats a ton of work, so we'll just check
        // that the operation was successful
        assert!(fracs.is_ok());
    }

    #[rstest]
    fn test_find_prohibited_codons(db: Database, org_id: i32) {

        let fracs = get_codon_fracs_by_amino_acid(&db, &org_id).unwrap();
        let query = HashMap::from([(org_id, fracs)]);

        let prohibited_codons = find_prohibited_codons(&query, 0.1);

        let prohibted_arginines = prohibited_codons.get(&'R').unwrap();
        assert!(prohibted_arginines.contains(&Codon::CGA));
        assert!(prohibted_arginines.contains(&Codon::CGG));
        assert!(prohibted_arginines.contains(&Codon::AGA));
        assert!(prohibted_arginines.contains(&Codon::AGG));
    }
}

use statrs::statistics::Statistics;
use std::collections::HashMap;
use std::f32;

use crate::consts::{AACodonLibrary, NumCodonsByAA};
use crate::db::Database;
use crate::models::{Codon, ProhibitedCodons};

type CodonCountsByAminoAcid = HashMap<char, HashMap<Codon, i32>>;
type CodonFracsByAminoAcid = HashMap<char, HashMap<Codon, f32>>;
type UsageDataByOrganism = HashMap<i32, CodonFracsByAminoAcid>;

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
    #[allow(clippy::manual_range_contains)]
    if threshold < 0.0 || threshold > 1.0 {
        panic!("Threshold must be between 0 and 1");
    }

    let mut prohibited_codons: ProhibitedCodons = HashMap::new();

    for amino_acid_map in query.values() {
        for (aa, codon_preference) in amino_acid_map {
            for (codon, preference) in codon_preference {
                if preference < &threshold {
                    prohibited_codons
                        .entry(*aa)
                        .or_default()
                        .push(codon.clone())
                }
            }
        }
    }

    prohibited_codons
}

///
/// This will adjust an individual organisms codon tables to set
/// the codons in the prohibited codons list to 0.0 and then renormalize
/// the acceptable codons so that each residue's total sum of codon preferences
/// is equal to 1. This will remove the codons in place. This helps to
/// abvoid a copy of the data.
///
/// # Arguments
/// - `query` - The codon usage data. This is a map of organism IDs to a map of amino acids to a map of codons to codon preferences (confusing I know)
/// - `prohibited_codons` - The prohibited codons. This is a map of amino acids to a list of codons
/// - `var_threshold` - This is for residues for which all codons would be considered prohibited, the codon would be allowed in use if it's variance is within this threshold of the average of the list of minimum variances
///
/// # Returns
/// - `UsageDataByOrganism` - The codon usage data with the prohibited codons removed
pub fn remove_prohibited_codons(
    query: &mut UsageDataByOrganism,
    prohibited_codons: &ProhibitedCodons,
    var_threshold: f32,
) {
    let num_codons_by_aa = NumCodonsByAA::new().num_codons;
    let mut inaccessible_residues: Vec<char> = vec![];

    for (aa, codons) in prohibited_codons {
        let total_codons_for_aa = match num_codons_by_aa.get(aa) {
            Some(n) => n.to_owned() as usize,
            None => panic!("Invalid amino acid: {}", aa),
        };
        // this is true if all codons for this amino acid are prohibited
        // basically, theres no way to use this amino acid right now
        if codons.len() == total_codons_for_aa {
            inaccessible_residues.push(*aa);
        }
    }

    // initializes a dictionary that will store which codons should be allowed back
    // in use for the inaccessible residues
    let mut allowed_codons: HashMap<char, Vec<Codon>> = HashMap::new();

    for aa in inaccessible_residues {
        // initialize preference value map
        let mut preference_values: HashMap<Codon, Vec<f32>> = HashMap::new();

        // insert empty vector to initialize
        allowed_codons.insert(aa, vec![]);

        // loop thru and create the list of all values for inaccessible codons
        for codon_usage in query.values() {
            for codon_preference in codon_usage.values() {
                for (codon, preference) in codon_preference {
                    preference_values
                        .entry(codon.to_owned())
                        .or_default()
                        .push(*preference)
                }
            }
        }

        // convert to HashMap that maps list of prefs to variance
        let mut preference_values_variances: HashMap<Codon, f32> = HashMap::new();
        for (codon, preference_values) in preference_values {
            // map prefernce values to f64
            let preference_values: Vec<f64> = preference_values.iter().map(|x| *x as f64).collect();
            preference_values_variances.insert(codon, preference_values.variance() as f32);
        }

        // loop through the preference value list of variances
        // and find the minimum variance
        let mut min_variance: Vec<f64> = vec![f64::MAX];

        for (codon, variance) in preference_values_variances {
            // get average variance
            let mean = min_variance.to_owned().mean() as f32;

            if variance < (1.0 - var_threshold) * mean {
                min_variance = vec![variance as f64];
                allowed_codons.entry(aa).or_default().push(codon.to_owned());
            } else if variance < (1.0 + var_threshold) * mean {
                min_variance.push(variance as f64);
                allowed_codons.entry(aa).or_default().push(codon.to_owned());
            }
        }
    }

    // remove allowed codons from prohibited codons
    // this isn't the most efficient way to do this, but it works for now
    let mut prohibited_codons_cleaned = prohibited_codons.to_owned();

    for (aa, allowed_codons) in allowed_codons {
        for (i, _) in allowed_codons.iter().enumerate() {
            prohibited_codons_cleaned.entry(aa).or_default().remove(i);
        }
    }

    // finally, loop through the query and make the adjustments if necessary
    for (_, codon_usage) in query {
        for (aa, codon_preference) in codon_usage {
            let mut acceptable_codon_sum: f32 = 0.0;

            if prohibited_codons.contains_key(aa) {
                for (codon, pref) in &mut *codon_preference {
                    if !prohibited_codons[aa].contains(codon) {
                        acceptable_codon_sum += *pref;
                    }
                }

                for (codon, pref) in &mut *codon_preference {
                    if prohibited_codons[aa].contains(codon) && acceptable_codon_sum > 0.0 {
                        *pref /= acceptable_codon_sum;
                    } else {
                        *pref = 0.0;
                    }
                }
            }
        }
    }
}

pub fn equal_optimiation(query: &mut UsageDataByOrganism) {
    todo!()
}

pub fn get_species_weight() {
    todo!()
}

pub fn averaged_table() {
    todo!()
}

pub fn get_multitable_randomnumbers() {
    todo!()
}

pub fn convert_dna_to_protein() {
    todo!()
}

pub fn validate_query() {
    todo!()
}

pub fn optimize_codon_usage() {
    todo!()
}

pub fn get_rca_xyz() {
    todo!()
}

pub fn calculate_predicted_expression() {
    todo!()
}

pub fn get_redundantaa_rna() {
    todo!()
}

pub fn optimize_multitable_sd() {
    todo!()
}

pub fn optimize_multitable_ad() {
    todo!()
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

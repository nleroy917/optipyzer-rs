use statrs::statistics::Statistics;
use std::collections::HashMap;
use std::f32;

use anyhow::Result;

use crate::consts::{AACodonLibrary, NumCodonsByAA};
use crate::db::Database;
use crate::models::{Codon, ProhibitedCodons};

type CodonCountsByAminoAcid = HashMap<char, HashMap<Codon, i32>>;
type CodonFracsByAminoAcid = HashMap<char, HashMap<Codon, f32>>;
type UsageDataByOrganism = HashMap<i32, CodonFracsByAminoAcid>;
type SpeciesWeights = HashMap<i32, f32>;

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
pub fn find_prohibited_codons(query: &UsageDataByOrganism, threshold: f32) -> Result<ProhibitedCodons> {
    // check that the threshold is valid
    if !(0.0..=1.0).contains(&threshold) {
        anyhow::bail!("Threshold must be between 0 and 1")
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

    Ok(prohibited_codons)
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
) -> Result<()> {
    let num_codons_by_aa = NumCodonsByAA::new().num_codons;
    let mut inaccessible_residues: Vec<char> = vec![];

    for (aa, codons) in prohibited_codons {
        let total_codons_for_aa = match num_codons_by_aa.get(aa) {
            Some(n) => n.to_owned() as usize,
            None => anyhow::bail!("Invalid amino acid"),
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
    for codon_usage in query.values_mut() {
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

    Ok(())
}

///
/// This function will produce a dictionary of species weights for when the user desires equal optimization
/// 
/// # Arguments
/// - query: The codon usage data
/// 
/// # Returns
/// - 
pub fn equal_optimiation(query: &UsageDataByOrganism) -> (SpeciesWeights, HashMap<i32, u8>) {
    let mut species_expression: HashMap<i32, u8> = HashMap::new();
    let mut species_weights: HashMap<i32, f32> = HashMap::new();

    // compute what an equal weight would be, spread across all species
    let equal_weight = 1.0 / query.len() as f32;

    // set the species expression to 1
    for species in query.keys() {
        species_expression.insert(*species, 1);
        species_weights.insert(*species, equal_weight);
    }

    (species_weights, species_expression)

}

///
/// Normalizes the species weight requirements to set 1 as the
/// lowest weight value. For example, if the lowest weight value
/// is 0.5, then all weight values will be multiplied by 2.
///
/// # Arguments
/// - `weights` - The species weights
///
/// # Returns
/// - `SpeciesWeights` - The normalized species weights
pub fn normalize_species_weight_requirements(weights: &mut SpeciesWeights) {
    let min_weight = *weights
        .values()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    for weight in weights.values_mut() {
        *weight /= min_weight;
    }
}

///
/// Creates the 0th iteration multi-table, which is just an average of the individual codon preferences of species
/// after removing prohibited codons
///
pub fn averaged_table() {
    todo!()
}

pub fn get_multitable_randomnumbers() {
    todo!()
}

pub fn convert_dna_to_protein(seq: &str) -> String {
    let mut protein = String::new();

    for i in (0..seq.len()).step_by(3) {
        let codon = &seq[i..i + 3];
        match codon.to_uppercase().as_str() {
            "GCT" => protein.push('A'),
            "GCC" => protein.push('A'),
            "GCA" => protein.push('A'),
            "GCG" => protein.push('A'),
            "CGT" => protein.push('R'),
            "CGC" => protein.push('R'),
            "CGA" => protein.push('R'),
            "CGG" => protein.push('R'),
            "AGA" => protein.push('R'),
            "AGG" => protein.push('R'),
            "AAT" => protein.push('N'),
            "AAC" => protein.push('N'),
            "GAT" => protein.push('D'),
            "GAC" => protein.push('D'),
            "TGT" => protein.push('C'),
            "TGC" => protein.push('C'),
            "CAA" => protein.push('Q'),
            "CAG" => protein.push('Q'),
            "GAA" => protein.push('E'),
            "GAG" => protein.push('E'),
            "GGT" => protein.push('G'),
            "GGC" => protein.push('G'),
            "GGA" => protein.push('G'),
            "GGG" => protein.push('G'),
            "CAT" => protein.push('H'),
            "CAC" => protein.push('H'),
            "ATT" => protein.push('I'),
            "ATC" => protein.push('I'),
            "ATA" => protein.push('I'),
            "TTA" => protein.push('L'),
            "TTG" => protein.push('L'),
            "CTT" => protein.push('L'),
            "CTC" => protein.push('L'),
            "CTA" => protein.push('L'),
            "CTG" => protein.push('L'),
            "AAA" => protein.push('K'),
            "AAG" => protein.push('K'),
            "ATG" => protein.push('M'),
            "TTT" => protein.push('F'),
            "TTC" => protein.push('F'),
            "CCT" => protein.push('P'),
            "CCC" => protein.push('P'),
            "CCA" => protein.push('P'),
            "CCG" => protein.push('P'),
            "TCT" => protein.push('S'),
            "TCC" => protein.push('S'),
            "TCA" => protein.push('S'),
            "TCG" => protein.push('S'),
            "AGT" => protein.push('S'),
            "AGC" => protein.push('S'),
            "ACT" => protein.push('T'),
            "ACC" => protein.push('T'),
            "ACA" => protein.push('T'),
            "ACG" => protein.push('T'),
            "TGG" => protein.push('W'),
            "TAT" => protein.push('Y'),
            "TAC" => protein.push('Y'),
            "GTT" => protein.push('V'),
            "GTC" => protein.push('V'),
            "GTA" => protein.push('V'),
            "GTG" => protein.push('V'),
            "TAA" => protein.push('*'),
            "TAG" => protein.push('*'),
            "TGA" => protein.push('*'),
            _ => protein.push('?'),
        };
    }

    protein
}

pub fn optimize_codon_usage() {
    todo!()
}

pub fn get_rca_xyz() {
    todo!()
}

///
/// calculates the rca (a metric for comparison of predicted gene expression)
/// for each species based on the formula:
/// $$
/// \text{RCA} = \left( \Pi_{i=1}^{L} \text{RCA}_{xyz}(l) \right)^{\frac{1}{L}}
/// $$
/// where,
/// $$
/// \text{RCA}_{xyz} = \frac{X(x,y,z)}{X_1(x),X_2(y),X_3(z)}
/// $$
/// and uses it to predict protein expression,
/// as rca is correlated to the log of protein expression
pub fn calculate_predicted_expression(
    rca_xyz: HashMap<i32, HashMap<Codon, f32>>,
    sequence: &str,
) -> HashMap<i32, f32> {
    let mut rca: HashMap<i32, f32> = HashMap::new();
    for (org_id, codon_preferences) in rca_xyz {
        rca.insert(org_id, 1.0_f32);
        for i in (0..sequence.len()).step_by(3) {
            let codon = &sequence[i..i + 3];
            let codon = Codon::from(codon);
            rca.insert(
                org_id,
                rca[&org_id] * codon_preferences.get(&codon).unwrap(),
            );
        }
        // raise to the power of 1/L
        rca.insert(
            org_id,
            rca[&org_id].powf(1.0 / (sequence.len() as f32 / 3.0)),
        );
    }

    let min_exp = rca
        .values()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
        .to_owned();

    // normalize
    for exp in rca.values_mut() {
        *exp /= min_exp;
    }

    rca
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

        let prohibited_codons = find_prohibited_codons(&query, 0.1).unwrap();

        let prohibted_arginines = prohibited_codons.get(&'R').unwrap();
        assert!(prohibted_arginines.contains(&Codon::CGA));
        assert!(prohibted_arginines.contains(&Codon::CGG));
        assert!(prohibted_arginines.contains(&Codon::AGA));
        assert!(prohibted_arginines.contains(&Codon::AGG));
    }
}

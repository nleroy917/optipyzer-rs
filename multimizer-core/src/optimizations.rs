use std::collections::HashMap;

use anyhow::Result;

use crate::{
    consts::SequenceType,
    models::Codon,
    utils::{compute_rca, compute_rca_xyz_table, detect_sequence_type, select_random_codon_from_usage_table, translate_dna_sequence},
};

// type names for readability
pub type SpeciesWeights = HashMap<i32, f64>;
pub type CodonUsageAsFracs = HashMap<Codon, f64>;
pub type CodonUsageByResidue = HashMap<char, CodonUsageAsFracs>;
pub type CodonUsageByResidueByOrganism = HashMap<i32, HashMap<char, CodonUsageAsFracs>>;

pub struct OptimizationOptions {
    pub max_iterations: i32,
    pub seed: i32,
    pub prohibited_preference_threshold: f64,
    pub min_error: f64,
}

impl Default for OptimizationOptions {
    fn default() -> Self {
        OptimizationOptions {
            max_iterations: 1_000,
            seed: 42,
            prohibited_preference_threshold: 0.1,
            min_error: 0.01,
        }
    }
}

pub struct OptimizationResult {
    pub seq: String,
    pub iterations: i32,
    pub translated_seq: String,
    pub rca_value: f64
}

///
/// Optimize a query sequence for a particular organism. This sequence
/// can be either protein or DNA.
/// 
/// # Arguments
/// - query seq
/// - codon usage data for organism
/// - options for the optimization algorithm
/// 
/// # Returns
/// - optimized sequence
/// 
pub fn optimize_for_single_organism(
    query: &str,
    codon_usage: &CodonUsageByResidue,
    options: &OptimizationOptions,
) -> Result<OptimizationResult> {
    
    // detect sequence type, translate if necessary
    let seq_type = detect_sequence_type(query)?;
    let rca_xyz_table = compute_rca_xyz_table(codon_usage);
    let query = match seq_type {
        SequenceType::Dna => {
            // otherwise translate the sequence
            let query = translate_dna_sequence(query)?;
            query.to_string()
        }
        SequenceType::Protein => query.to_string(),
    };

    let mut optimized_sequence = String::new();
    
    for residue in query.chars() {
        let random_codon =
            select_random_codon_from_usage_table(residue, codon_usage, Some(options.seed))?;
        optimized_sequence.push_str(&random_codon.to_string());
    }

    let rca = compute_rca(&optimized_sequence, &rca_xyz_table)?;    
    let translated_seq = translate_dna_sequence(&optimized_sequence)?;

    Ok(OptimizationResult {
        seq: optimized_sequence,
        translated_seq,
        iterations: 1,
        rca_value: rca
    })
}

///
/// Dummy test function
/// 
/// This is not to be used.
/// 
pub fn optimize_seq_test(query: &str) -> String {
    String::from(query)
}

#[cfg(test)]
mod tests {
    use super::*;

    // use pretty_assertions::assert_eq;
    // use rstest::{fixture, rstest};
    use rstest::fixture;

    // const EPSILON: f64 = 1e-6;

    // fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
    //     (a - b).abs() < epsilon
    // }

    #[fixture]
    fn org_usage1() -> HashMap<char, HashMap<Codon, f32>> {
        HashMap::from([
            (
                'A',
                HashMap::from([
                    (Codon::GCT, 0.1),
                    (Codon::GCC, 0.2),
                    (Codon::GCA, 0.3),
                    (Codon::GCG, 0.4),
                ]),
            ),
            (
                'R',
                HashMap::from([
                    (Codon::CGT, 0.1),
                    (Codon::CGC, 0.2),
                    (Codon::CGA, 0.3),
                    (Codon::CGG, 0.4),
                ]),
            ),
        ])
    }

    #[fixture]
    fn org_usage2() -> HashMap<char, HashMap<Codon, f32>> {
        HashMap::from([
            (
                'A',
                HashMap::from([
                    (Codon::GCT, 0.2),
                    (Codon::GCC, 0.3),
                    (Codon::GCA, 0.4),
                    (Codon::GCG, 0.1),
                ]),
            ),
            (
                'R',
                HashMap::from([
                    (Codon::CGT, 0.2),
                    (Codon::CGC, 0.3),
                    (Codon::CGA, 0.4),
                    (Codon::CGG, 0.1),
                ]),
            ),
        ])
    }

    #[fixture]
    fn org_weights() -> SpeciesWeights {
        HashMap::from([(1, 0.33), (2, 0.67)])
    }
}

use std::collections::HashMap;

use anyhow::Result;

use crate::models::Codon;

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
}

pub fn optimize_for_single_organism(
    query: &str,
    codon_usage: &CodonUsageByResidue,
    options: &OptimizationOptions,
) -> Result<()> {
    Ok(())
}

pub fn optimize_seq_test(query: &str) -> String {
    String::from(query)
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};

    const EPSILON: f64 = 1e-6;

    fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }

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

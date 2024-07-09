use std::collections::HashMap;

use anyhow::Result;

use crate::consts::DEFAULT_PROHIBITED_CODON_THRESHOLD;
use crate::models::Codon;

// custom types for readability
type SpeciesWeights = HashMap<i32, f32>;
type CodonUsageAsFracs = HashMap<Codon, f32>;
type CodonUsageByResidue = HashMap<char, CodonUsageAsFracs>;
type CodonUsageByResidueByOrganism = HashMap<i32, HashMap<char, CodonUsageAsFracs>>;

///
/// Calculate the average codon usage across all organisms, given their weights. It works
/// in two steps:
///
/// 1. Compute the average codon usage across all organisms
/// 2. Remove "prohibited codons" from the average codon usage.
///
/// Prohibited codons are codons who's usage is below a certain threshold.
///
/// # Arguments
/// - weights: The weights of each organism
/// - codon_usages: The codon usages of each organism
///
/// # Returns
/// - `CodonUsageAsFracs` - The average codon usage
pub fn average_codon_usage(
    weights: &SpeciesWeights,
    codon_usages: &CodonUsageByResidueByOrganism,
    prohibited_codon_threshold: Option<f32>,
) -> Result<CodonUsageByResidue> {
    // init the average codon usage table that we will populate
    let mut average_codon_usage: CodonUsageByResidue = HashMap::new();

    // iterate over each organism and their codon usage
    for (org_id, codon_usage_by_residue) in codon_usages.iter() {
        if let Some(org_weight) = weights.get(org_id) {
            for (residue, codon_usage_as_fracs) in codon_usage_by_residue.iter() {
                for (codon, frac) in codon_usage_as_fracs.iter() {
                    // if the residue is not in the average codon usage table, add it
                    if !average_codon_usage.contains_key(residue) {
                        average_codon_usage.insert(*residue, HashMap::new());
                    }

                    // if the codon is not in the average codon usage table, add it
                    if !average_codon_usage
                        .get(residue)
                        .unwrap()
                        .contains_key(codon)
                    {
                        average_codon_usage
                            .get_mut(residue)
                            .unwrap()
                            .insert(codon.clone(), 0.0);
                    }

                    // update the average codon usage table with the new value
                    let current_frac = average_codon_usage
                        .get(residue)
                        .unwrap()
                        .get(codon)
                        .unwrap()
                        + (frac * (*org_weight));

                    average_codon_usage
                        .get_mut(residue)
                        .unwrap()
                        .insert(codon.clone(), current_frac);
                }
            }
        } else {
            anyhow::bail!("No weight found for organism ID: {}", org_id);
        }
    }
    
    let prohibited_threshold = prohibited_codon_threshold.unwrap_or(DEFAULT_PROHIBITED_CODON_THRESHOLD);

    // remove prohibited codons
    for (_, codon_usage_as_fracs) in average_codon_usage.iter_mut() {
        let mut total_frac = 0.0;

        for (_, frac) in codon_usage_as_fracs.iter() {
            total_frac += frac;
        }

        let mut prohibited_codons = Vec::new();

        for (codon, frac) in codon_usage_as_fracs.iter_mut() {
            if *frac / total_frac < prohibited_threshold {
                prohibited_codons.push(codon.clone());
            }
        }

        for codon in prohibited_codons.iter() {
            codon_usage_as_fracs.remove(codon);
        }
    }

    // renormalize the codon usage
    for (_, codon_usage_as_fracs) in average_codon_usage.iter_mut() {
        let mut total_frac = 0.0;

        for (_, frac) in codon_usage_as_fracs.iter() {
            total_frac += frac;
        }

        for (_, frac) in codon_usage_as_fracs.iter_mut() {
            *frac /= total_frac;
        }
    }

    Ok(average_codon_usage)
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};

    const EPSILON: f32 = 1e-6;

    fn approx_equal(a: f32, b: f32, epsilon: f32) -> bool {
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

    #[rstest]
    fn test_average_codon_usage(
        org_usage1: HashMap<char, HashMap<Codon, f32>>,
        org_usage2: HashMap<char, HashMap<Codon, f32>>,
        org_weights: SpeciesWeights,
    ) {
        let codon_usages = HashMap::from([(1, org_usage1), (2, org_usage2)]);

        let averaged_table = average_codon_usage(&org_weights, &codon_usages, Some(0.1)).unwrap();

        assert_eq!(averaged_table.contains_key(&'A'), true);
        assert_eq!(averaged_table.contains_key(&'R'), true);
        assert_eq!(
            averaged_table.get(&'A').unwrap().contains_key(&Codon::GCT),
            true
        );
        assert_eq!(approx_equal(
            *averaged_table.get(&'A').unwrap().get(&Codon::GCT).unwrap(),
            0.167,
            EPSILON
        ), true);
    }
}

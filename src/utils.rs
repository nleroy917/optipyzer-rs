use std::collections::HashMap;

use anyhow::Result;

use crate::models::Codon;
use crate::optimizations::{CodonUsageByResidueByOrganism, SpeciesWeights};

/// 
/// This function does three things in turn:
/// 1. Identify "prohibited codons" -- codons that fall below a usage threshold and should not be used in the sequence.
/// 2. Remove these codons from the usage data
/// 3. Recalculate usage with removed codons
/// 
/// # Arguments
/// - usage_data: Codon usage data by species
/// - prohibited_threshold: Threshold to use to be considered "prohibited"
/// 
/// # Returns
/// - the new, recomputed table
pub fn remove_prohibited_codons(usage_data: &CodonUsageByResidueByOrganism, prohibited_threshold: f64) -> Result<CodonUsageByResidueByOrganism> {

    let mut corrected_usage_data: CodonUsageByResidueByOrganism = HashMap::new();
    let mut renormalized_usage_data: CodonUsageByResidueByOrganism = HashMap::new();
    let mut prohibited_codons: HashMap<char, Vec<Codon>> = HashMap::new();

    // step 1 -- identify prohibited codons
    for org_usage_data in usage_data.values() {
        for (aa, preferences) in org_usage_data {
            for (codon, pref) in preferences {
                if *pref < prohibited_threshold {
                    if prohibited_codons.contains_key(aa) {
                        prohibited_codons.get_mut(aa).unwrap().push(codon.clone());
                    } else {
                        prohibited_codons.insert(*aa, vec![codon.clone()]);
                    }
                }
            }
        }
    }

    // step 2 -- remove prohibited codons
    for (org_id, org_usage_data) in usage_data {
        let mut corrected_org_usage_data: HashMap<char, HashMap<Codon, f64>> = HashMap::new();
        for (aa, preferences) in org_usage_data {
            let mut corrected_preferences: HashMap<Codon, f64> = HashMap::new();
            for (codon, pref) in preferences {
                if !prohibited_codons.contains_key(aa) || !prohibited_codons.get(aa).unwrap().contains(codon) {
                    corrected_preferences.insert(codon.clone(), *pref);
                }
            }
            corrected_org_usage_data.insert(*aa, corrected_preferences);
        }
        corrected_usage_data.insert(*org_id, corrected_org_usage_data);
    }

    // step 3 -- recalculate usage with removed codons
    for (org_id, org_usage_data) in corrected_usage_data {
        let mut renormalized_org_usage_data: HashMap<char, HashMap<Codon, f64>> = HashMap::new();
        for (aa, preferences) in org_usage_data {
            let total: f64 = preferences.values().sum();
            let mut renormalized_preferences: HashMap<Codon, f64> = HashMap::new();
            for (codon, pref) in preferences {
                renormalized_preferences.insert(codon.clone(), pref / total);
            }
            renormalized_org_usage_data.insert(aa, renormalized_preferences);
        }
        renormalized_usage_data.insert(org_id, renormalized_org_usage_data);
    }

    Ok(renormalized_usage_data)
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
    fn org_usage1() -> HashMap<char, HashMap<Codon, f64>> {
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
    fn org_usage2() -> HashMap<char, HashMap<Codon, f64>> {
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
    fn test_remove_prohibited_codons(org_usage1: HashMap<char, HashMap<Codon, f64>>) {

        let usage_data: CodonUsageByResidueByOrganism = HashMap::from([(1, org_usage1)]);

        let prohibited_threshold = 0.2;
        let corrected_usage_data = remove_prohibited_codons(&usage_data, prohibited_threshold).unwrap();

        let expected_usage_data: CodonUsageByResidueByOrganism = HashMap::from([
            (
                1,
                HashMap::from([
                    (
                        'A',
                        HashMap::from([
                            // (Codon::GCT, 0.1), // --> gets removed!
                            (Codon::GCC, 0.22222222),
                            (Codon::GCA, 0.33333333),
                            (Codon::GCG, 0.44444444),
                        ]),
                    ),
                    (
                        'R',
                        HashMap::from([
                            // (Codon::CGT, 0.1), // --> gets removed!
                            (Codon::CGC, 0.22222222),
                            (Codon::CGA, 0.33333333),
                            (Codon::CGG, 0.44444444),
                        ]),
                    ),
                ])
            ),
        ]);

        for (org_id, org_usage_data) in corrected_usage_data {
            for (aa, preferences) in org_usage_data {
                for (codon, pref) in preferences {
                    assert_eq!(approx_equal(pref, *expected_usage_data.get(&org_id).unwrap().get(&aa).unwrap().get(&codon).unwrap(), EPSILON), true);
                }
            }
        }

        
    }

}

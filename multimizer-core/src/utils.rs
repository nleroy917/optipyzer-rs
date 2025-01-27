use std::collections::HashMap;
use std::io::Read;

use anyhow::Result;
use bio::io::fasta;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::consts::{CodonToAA, SequenceType, VALID_AMINO_ACIDS, VALID_NUCLEOTIDES};
use crate::models::Codon;
use crate::optimizations::{CodonUsageByResidue, CodonUsageByResidueByOrganism, SpeciesWeights};

pub type RCAxyzTable = HashMap<Codon, f64>;

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
///
pub fn remove_prohibited_codons(
    usage_data: &CodonUsageByResidueByOrganism,
    prohibited_threshold: f64,
) -> Result<CodonUsageByResidueByOrganism> {
    let mut corrected_usage_data: CodonUsageByResidueByOrganism = HashMap::new();
    let mut renormalized_usage_data: CodonUsageByResidueByOrganism = HashMap::new();
    let mut prohibited_codons: HashMap<char, Vec<Codon>> = HashMap::new();

    let num_codons_by_residue = HashMap::from([
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

    // step 1 -- identify prohibited codons
    for org_usage_data in usage_data.values() {
        for (aa, preferences) in org_usage_data {
            for (codon, pref) in preferences {
                if *pref < prohibited_threshold {
                    if prohibited_codons.contains_key(aa) {
                        prohibited_codons.get_mut(aa).unwrap().push(*codon);
                    } else {
                        prohibited_codons.insert(*aa, vec![*codon]);
                    }
                }
            }
        }
    }

    // step 2 -- identify inaccessable residues (residues with all codons prohibited)
    for (aa, codons) in prohibited_codons.iter() {
        if codons.len() == *num_codons_by_residue.get(aa).unwrap() as usize {
            todo!("What do we do with residues that have no accessible codons?");
            return Err(anyhow::anyhow!("Residue {} has no accessible codons", aa));
        }
    }

    // step 3 -- remove prohibited codons
    for (org_id, org_usage_data) in usage_data {
        let mut corrected_org_usage_data: HashMap<char, HashMap<Codon, f64>> = HashMap::new();
        for (aa, preferences) in org_usage_data {
            let mut corrected_preferences: HashMap<Codon, f64> = HashMap::new();
            for (codon, pref) in preferences {
                if !prohibited_codons.contains_key(aa)
                    || !prohibited_codons.get(aa).unwrap().contains(codon)
                {
                    corrected_preferences.insert(*codon, *pref);
                }
            }
            corrected_org_usage_data.insert(*aa, corrected_preferences);
        }
        corrected_usage_data.insert(*org_id, corrected_org_usage_data);
    }

    // step 4 -- recalculate usage with removed codons
    for (org_id, org_usage_data) in corrected_usage_data {
        let mut renormalized_org_usage_data: HashMap<char, HashMap<Codon, f64>> = HashMap::new();
        for (aa, preferences) in org_usage_data {
            let total: f64 = preferences.values().sum();
            let mut renormalized_preferences: HashMap<Codon, f64> = HashMap::new();
            for (codon, pref) in preferences {
                renormalized_preferences.insert(codon, pref / total);
            }
            renormalized_org_usage_data.insert(aa, renormalized_preferences);
        }
        renormalized_usage_data.insert(org_id, renormalized_org_usage_data);
    }

    Ok(renormalized_usage_data)
}

///
/// This function builds an averaged codon usage table based on the provided usage data and species weights.
///
/// # Arguments
/// - usage_data: Codon usage data by species
/// - weights: Weights to use for averaging
///
/// # Returns
/// - the averaged table
///
pub fn build_averaged_table(
    usage_data: &CodonUsageByResidueByOrganism,
    weights: &SpeciesWeights,
) -> CodonUsageByResidue {
    let mut averaged_table: CodonUsageByResidue = HashMap::new();

    for (org_id, org_usage_data) in usage_data {
        for (aa, preferences) in org_usage_data {
            for (codon, pref) in preferences {
                if averaged_table.contains_key(aa) {
                    if averaged_table.get(aa).unwrap().contains_key(codon) {
                        let current_pref =
                            averaged_table.get_mut(aa).unwrap().get_mut(codon).unwrap();
                        *current_pref += pref * weights.get(org_id).unwrap();
                    } else {
                        averaged_table
                            .get_mut(aa)
                            .unwrap()
                            .insert(*codon, pref * weights.get(org_id).unwrap());
                    }
                } else {
                    let mut new_preferences: HashMap<Codon, f64> = HashMap::new();
                    new_preferences.insert(*codon, pref * weights.get(org_id).unwrap());
                    averaged_table.insert(*aa, new_preferences);
                }
            }
        }
    }

    averaged_table
}

///
/// Read in a pasted user input of FASTA sequences and parse them into a HashMap.
/// The key is the sequence name and the value is the sequence itself.
///
/// # Arguments
/// - input: The user input string
///
/// # Returns
/// - the parsed sequences
///
pub fn parse_fasta_sequences_from_string(input: &str) -> Result<HashMap<String, String>> {
    let fa_reader = fasta::Reader::new(input.as_bytes());
    let mut sequences: HashMap<String, String> = HashMap::new();

    for record in fa_reader.records() {
        let record = record?;
        let name = record.id().to_string();
        let mut buf = String::new();

        record.seq().read_to_string(&mut buf)?;
        sequences.insert(name, buf);
    }

    Ok(sequences)
}

///
/// Detect the type of the sequence (either DNA or Protein)
/// if the sequence type can't be determined, we return None
///
/// # Arguments
/// - query
///
/// # Returns
/// - sequence type enum
///
pub fn detect_sequence_type(query: &str) -> Result<SequenceType> {
    // if everything is ATCG --> DNA
    if query
        .chars()
        .all(|residue| VALID_NUCLEOTIDES.contains(residue))
    {
        Ok(SequenceType::Dna)
    // else if everything is ACDEFGHIKLMNPQRSTVWY*
    } else if query
        .chars()
        .all(|residue| VALID_AMINO_ACIDS.contains(residue))
    {
        Ok(SequenceType::Protein)
    // otherwise you gave me something weird
    } else {
        for (pos, r) in query.chars().enumerate() {
            if !VALID_NUCLEOTIDES.contains(r) && !VALID_AMINO_ACIDS.contains(r) {
                anyhow::bail!(
                    "Could not determine the sequence type. Invalid residue {r} at position {pos}"
                )
            }
        }
        anyhow::bail!("Unknown error occured determining the sequence type! How did we get here?")
    }
}

///
/// Converts a DNA sequence to a protein sequence
///
/// # Arguments
/// - query
///
/// # Returns
/// - translated_sequence
///
pub fn translate_dna_sequence(query: &str) -> Result<String> {
    let codon_to_aa_map = CodonToAA::new();
    let mut translated_sequence = String::new();

    // verify sequence length
    if query.len() % 3 != 0 {
        anyhow::bail!("The sequence cannot be translated because it is not divisible by 3!")
    }

    for codon in query.chars().collect::<Vec<char>>().chunks(3) {
        let codon: String = codon.iter().collect();
        let codon = Codon::try_from(codon.as_str());

        match codon {
            Ok(c) => {
                if let Some(aa) = codon_to_aa_map.convert(&c) {
                    translated_sequence.push(aa);
                } else {
                    anyhow::bail!("Invalid codon found in sequence: {c}")
                }
            }
            Err(e) => anyhow::bail!(e),
        }
    }

    Ok(translated_sequence)
}

///
/// Selects a random codon give a residue according to the weights in the usage data
///
/// # Arguments
/// - residue to select for
/// - usage data
///
/// # Returns
/// - selected codon
///
pub fn select_random_codon_from_usage_table(
    residue: char,
    usage_data: &CodonUsageByResidue,
    seed: Option<i32>,
) -> Result<Codon> {
    let seed = seed.unwrap_or(42);
    let mut rng = ChaCha8Rng::seed_from_u64(seed as u64);
    if let Some(usage_for_residue) = usage_data.get(&residue) {
        // quickly iterate to get paired list of codons and weights
        let (codons, weightings): (Vec<Codon>, Vec<f64>) = usage_for_residue.iter().unzip();
        let dist = WeightedIndex::new(weightings)?;
        Ok(codons[dist.sample(&mut rng)])
    } else {
        anyhow::bail!("Invalid residue passed in: {residue}")
    }
}

///
/// Compute the RCAxyz table for a given codon usage data table
///
/// It works according to the following formula:
/// $$
/// rca_{xyz}(codon) = \frac{f(codon)}{f1(x) * f2(y) * f3(z)}
/// $$
///
/// # Arguments
/// - codon counds for an organism
///
/// # Returns
/// - the computed rca table
///
pub fn compute_rca_xyz_table(codon_usage: &CodonUsageByResidue) -> RCAxyzTable {
    // 1) Compute base frequency by position
    //    base_position[pos][base], with pos in {0,1,2} for codon positions
    let mut base_position: [HashMap<char, f64>; 3] =
        [HashMap::new(), HashMap::new(), HashMap::new()];
    // We'll need these sums to normalize the frequencies
    let mut base_position_sums = [0.0, 0.0, 0.0];

    for codons in codon_usage.values() {
        for (codon, &count) in codons {
            // codon is something like "ATG"
            let count_f64 = count;
            for (i, base_char) in codon.to_string().chars().enumerate() {
                *base_position[i].entry(base_char).or_insert(0.0) += count_f64;
            }
        }
    }
    // Convert each position’s counts into frequencies
    // First find sum per position
    for (pos, base_map) in base_position.iter().enumerate() {
        let sum_pos: f64 = base_map.values().sum();
        base_position_sums[pos] = sum_pos;
    }
    // Normalize: base_position[pos][base] /= sum for that pos
    for pos in 0..3 {
        for base_val in base_position[pos].values_mut() {
            *base_val /= base_position_sums[pos];
        }
    }

    // 2) Finally compute rca_xyz(codon) = f(xyz) / (f1(x)*f2(y)*f3(z))
    let mut rca_xyz: HashMap<Codon, f64> = HashMap::new();
    for codons in codon_usage.values() {
        for (cdon, &freq) in codons {
            let mut pos_factor = 1.0;
            for (i, base_char) in cdon.to_string().chars().enumerate() {
                if let Some(bfreq) = base_position[i].get(&base_char) {
                    pos_factor *= bfreq;
                } else {
                    pos_factor = 0.0;
                    break;
                }
            }
            if pos_factor > 0.0 {
                rca_xyz.insert(*cdon, freq / pos_factor);
            } else {
                rca_xyz.insert(*cdon, 0.0);
            }
        }
    }

    rca_xyz
}

/// TODO: this was from chat... lets verify that this works.
/// Compute the overall RCA for a given DNA string, using the rca_xyz_table.
/// We'll assume optimized_dna.len() is a multiple of 3, and every codon is present in the table.
pub fn compute_rca(optimized_dna: &str, rca_xyz_table: &RCAxyzTable) -> Result<f64> {
    let length = optimized_dna.len();
    if length % 3 != 0 {
        anyhow::bail!("DNA length is not multiple of 3!")
    }

    let num_codons = length / 3;
    let mut product = 1.0f64;

    for chunk in optimized_dna.as_bytes().chunks(3) {
        let codon = std::str::from_utf8(chunk).unwrap(); // e.g. "ATG"
        let codon = match Codon::try_from(codon) {
            Ok(codon) => codon,
            Err(e) => anyhow::bail!("There was an error reading the sequence codon: {e}"),
        };

        if let Some(&rca_val) = rca_xyz_table.get(&codon) {
            product *= rca_val;
        } else {
            // skip!
        }
    }

    let rca = product.powf(1.0 / (num_codons as f64));

    Ok(rca)
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
    fn test_detect_sequence_type() {
        // successful cases
        assert_eq!(detect_sequence_type("ATCG").unwrap(), SequenceType::Dna);
        assert_eq!(
            detect_sequence_type("ACDEFGHIKLMNPQRSTVWY*").unwrap(),
            SequenceType::Protein
        );

        // unsuccessful cases -- X is invalid!
        assert_eq!(detect_sequence_type("ATCGX").is_err(), true);
        assert_eq!(
            detect_sequence_type("ACDEFGHIKLMNPQRSTVWY*Z").is_err(),
            true
        );
    }

    #[rstest]
    fn test_translate_dna_sequence() {
        // successful case
        assert_eq!(translate_dna_sequence("ATGGCC").unwrap(), "MA");

        // unsuccessful cases
        assert_eq!(translate_dna_sequence("ATGGC").is_err(), true); // Not divisible by 3
        assert_eq!(translate_dna_sequence("ATGXCC").is_err(), true); // Invalid codon
    }

    #[rstest]
    fn test_remove_prohibited_codons(org_usage1: HashMap<char, HashMap<Codon, f64>>) {
        let usage_data: CodonUsageByResidueByOrganism = HashMap::from([(1, org_usage1)]);

        let prohibited_threshold = 0.2;
        let corrected_usage_data =
            remove_prohibited_codons(&usage_data, prohibited_threshold).unwrap();

        let expected_usage_data: CodonUsageByResidueByOrganism = HashMap::from([(
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
            ]),
        )]);

        for (org_id, org_usage_data) in corrected_usage_data {
            for (aa, preferences) in org_usage_data {
                for (codon, pref) in preferences {
                    assert_eq!(
                        approx_equal(
                            pref,
                            *expected_usage_data
                                .get(&org_id)
                                .unwrap()
                                .get(&aa)
                                .unwrap()
                                .get(&codon)
                                .unwrap(),
                            EPSILON
                        ),
                        true
                    );
                }
            }
        }
    }

    #[rstest]
    fn test_average_codon_table(
        org_usage1: HashMap<char, HashMap<Codon, f64>>,
        org_usage2: HashMap<char, HashMap<Codon, f64>>,
        org_weights: SpeciesWeights,
    ) {
        let usage_data: CodonUsageByResidueByOrganism =
            HashMap::from([(1, org_usage1), (2, org_usage2)]);

        let averaged_table = build_averaged_table(&usage_data, &org_weights);

        let expected_averaged_table: CodonUsageByResidue = HashMap::from([
            (
                'A',
                HashMap::from([
                    (Codon::GCT, 0.167),
                    (Codon::GCC, 0.267),
                    (Codon::GCA, 0.367),
                    (Codon::GCG, 0.199),
                ]),
            ),
            (
                'R',
                HashMap::from([
                    (Codon::CGT, 0.167),
                    (Codon::CGC, 0.267),
                    (Codon::CGA, 0.367),
                    (Codon::CGG, 0.199),
                ]),
            ),
        ]);

        for (aa, preferences) in averaged_table {
            for (codon, pref) in preferences {
                assert_eq!(
                    approx_equal(
                        pref,
                        *expected_averaged_table
                            .get(&aa)
                            .unwrap()
                            .get(&codon)
                            .unwrap(),
                        EPSILON
                    ),
                    true
                );
            }
        }
    }
}

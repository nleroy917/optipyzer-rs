// #[cfg(test)]
// mod tests {
//     use super::*;

//     use pretty_assertions::assert_eq;
//     use rstest::{fixture, rstest};

//     const EPSILON: f64 = 1e-6;

//     fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
//         (a - b).abs() < epsilon
//     }

//     #[fixture]
//     fn org_usage1() -> HashMap<char, HashMap<Codon, f32>> {
//         HashMap::from([
//             (
//                 'A',
//                 HashMap::from([
//                     (Codon::GCT, 0.1),
//                     (Codon::GCC, 0.2),
//                     (Codon::GCA, 0.3),
//                     (Codon::GCG, 0.4),
//                 ]),
//             ),
//             (
//                 'R',
//                 HashMap::from([
//                     (Codon::CGT, 0.1),
//                     (Codon::CGC, 0.2),
//                     (Codon::CGA, 0.3),
//                     (Codon::CGG, 0.4),
//                 ]),
//             ),
//         ])
//     }

//     #[fixture]
//     fn org_usage2() -> HashMap<char, HashMap<Codon, f32>> {
//         HashMap::from([
//             (
//                 'A',
//                 HashMap::from([
//                     (Codon::GCT, 0.2),
//                     (Codon::GCC, 0.3),
//                     (Codon::GCA, 0.4),
//                     (Codon::GCG, 0.1),
//                 ]),
//             ),
//             (
//                 'R',
//                 HashMap::from([
//                     (Codon::CGT, 0.2),
//                     (Codon::CGC, 0.3),
//                     (Codon::CGA, 0.4),
//                     (Codon::CGG, 0.1),
//                 ]),
//             ),
//         ])
//     }

//     #[fixture]
//     fn org_weights() -> SpeciesWeights {
//         HashMap::from([(1, 0.33), (2, 0.67)])
//     }
// }

use crate::SizedContraction;
use std::collections::{HashMap, HashSet};

pub enum SingletonMethod {
    Identity,
    Permutation,
    Summation,
    Diagonalization,
    PermutationAndSummation,
    DiagonalizationAndSummation,
}

pub struct SingletonSummary {
    num_summed_axes: usize,
    num_diagonalized_axes: usize,
    num_reordered_axes: usize,
}

impl SingletonSummary {
    pub fn new(sc: &SizedContraction) -> Self {
        assert_eq!(sc.contraction.operand_indices.len(), 1);
        let output_indices = &sc.contraction.output_indices;
        let input_indices = &sc.contraction.operand_indices[0];

        SingletonSummary::from_indices(&input_indices, &output_indices)
    }

    fn from_indices(input_indices: &[char], output_indices: &[char]) -> Self {
        let mut input_counts = HashMap::new();
        for &c in input_indices.iter() {
            *input_counts.entry(c).or_insert(0) += 1;
        }
        let num_summed_axes = input_counts.len() - output_indices.len();
        let num_diagonalized_axes = input_counts.iter().filter(|(_, &v)| v > 1).count();
        let num_reordered_axes = output_indices
            .iter()
            .zip(input_indices.iter())
            .filter(|(&output_char, &input_char)| output_char != input_char)
            .count();

        SingletonSummary {
            num_summed_axes,
            num_diagonalized_axes,
            num_reordered_axes,
        }
    }

    pub fn get_strategy(&self) -> SingletonMethod {
        match (
            self.num_summed_axes,
            self.num_diagonalized_axes,
            self.num_reordered_axes,
        ) {
            (0, 0, 0) => SingletonMethod::Identity,
            (0, 0, _) => SingletonMethod::Permutation,
            (_, 0, 0) => SingletonMethod::Summation,
            (0, _, _) => SingletonMethod::Diagonalization,
            (_, 0, _) => SingletonMethod::PermutationAndSummation,
            (_, _, _) => SingletonMethod::DiagonalizationAndSummation,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PairMethod {
    HadamardProduct,
    HadamardProductGeneral,
    TensordotFixedPosition,
    TensordotGeneral,
    ScalarMatrixProduct,
    ScalarMatrixProductGeneral,
    MatrixScalarProduct,
    MatrixScalarProductGeneral,
    BroadcastProductGeneral,
    StackedTensordotGeneral,
}

#[derive(Debug, Clone)]
pub struct PairSummary {
    num_stacked_axes: usize,
    num_lhs_outer_axes: usize,
    num_rhs_outer_axes: usize,
    num_contracted_axes: usize,
}

impl PairSummary {
    pub fn new(sc: &SizedContraction) -> Self {
        assert_eq!(sc.contraction.operand_indices.len(), 2);
        let output_indices = &sc.contraction.output_indices;
        let lhs_indices = &sc.contraction.operand_indices[0];
        let rhs_indices = &sc.contraction.operand_indices[1];

        PairSummary::from_indices(&lhs_indices, &rhs_indices, &output_indices)
    }

    fn from_indices(lhs_indices: &[char], rhs_indices: &[char], output_indices: &[char]) -> Self {
        let lhs_uniques: HashSet<char> = lhs_indices.iter().cloned().collect();
        let rhs_uniques: HashSet<char> = rhs_indices.iter().cloned().collect();
        let output_uniques: HashSet<char> = output_indices.iter().cloned().collect();
        assert_eq!(lhs_indices.len(), lhs_uniques.len());
        assert_eq!(rhs_indices.len(), rhs_uniques.len());
        assert_eq!(output_indices.len(), output_uniques.len());

        let lhs_and_rhs: HashSet<char> = lhs_uniques.intersection(&rhs_uniques).cloned().collect();
        let stacked: HashSet<char> = lhs_and_rhs.intersection(&output_uniques).cloned().collect();

        let num_stacked_axes = stacked.len();
        let num_contracted_axes = lhs_and_rhs.len() - num_stacked_axes;
        let num_lhs_outer_axes = lhs_uniques.len() - num_stacked_axes - num_contracted_axes;
        let num_rhs_outer_axes = rhs_uniques.len() - num_stacked_axes - num_contracted_axes;

        PairSummary {
            num_stacked_axes,
            num_lhs_outer_axes,
            num_rhs_outer_axes,
            num_contracted_axes,
        }
    }

    pub fn get_strategy(&self) -> PairMethod {
        match (
            self.num_contracted_axes,
            self.num_lhs_outer_axes,
            self.num_rhs_outer_axes,
            self.num_stacked_axes,
        ) {
            (0, 0, 0, _) => PairMethod::HadamardProductGeneral,
            (0, 0, _, 0) => PairMethod::ScalarMatrixProductGeneral,
            (0, _, 0, 0) => PairMethod::MatrixScalarProductGeneral,
            (_, _, _, 0) => PairMethod::TensordotGeneral,
            (_, _, _, _) => PairMethod::StackedTensordotGeneral,
        }
    }
}

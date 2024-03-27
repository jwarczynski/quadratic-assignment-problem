use crate::{argsort, instance::Instance};

use super::{Solution, Solver, SolvingError};

pub struct HeuristicSolver<'a> {
    instance: &'a Instance,
}

impl<'a> HeuristicSolver<'a> {
    pub fn new(instance: &Instance) -> HeuristicSolver {
        HeuristicSolver { instance }
    }
}

impl<'a> Solver for HeuristicSolver<'a> {
    fn solve(&mut self, _initial_solution: Vec<usize>) -> Result<Solution, SolvingError> {
        let a_rows_sums: Vec<usize> = self
            .instance
            .matrix_a
            .iter()
            .map(|row| row.iter().sum())
            .collect();
        let b_rows_sums: Vec<usize> = self
            .instance
            .matrix_b
            .iter()
            .map(|row| row.iter().sum())
            .collect();

        let sorted_indices_a = argsort(&a_rows_sums, true);
        let sorted_indices_b = argsort(&b_rows_sums, false);

        let mut permuatation = vec![0; a_rows_sums.len()];
        for i in 0..a_rows_sums.len() {
            permuatation[sorted_indices_a[i]] = sorted_indices_b[i];
        }

        Ok(Solution {
            permutation: permuatation,
            evaluations: 0,
            solution_changes: 0,
        })
    }

    fn get_name(&self) -> String {
        "HeuristicSolver".to_string()
    }

    fn get_instance(&self) -> &Instance {
        self.instance
    }

    fn set_time_limit(&mut self, time_limit: u128) {}
}

use crate::{get_random_permutation, instance::Instance};

use super::{Solution, Solver, SolvingError};

pub struct RandomSearchSolver<'a> {
    instance: &'a Instance,
    max_iterations: usize,
}

impl<'a> RandomSearchSolver<'a> {
    pub fn new(instance: &Instance, max_iterations: usize) -> RandomSearchSolver {
        RandomSearchSolver {
            instance,
            max_iterations,
        }
    }
}

impl<'a> Solver for RandomSearchSolver<'a> {
    fn solve(&mut self, _initial_solution: Vec<usize>) -> Result<Solution, SolvingError> {
        let mut best_permutation = get_random_permutation(self.instance.size);
        let mut best_cost = self.instance.evaluate(best_permutation.as_ref());
        let mut solution_changes = 0;

        for _ in 0..self.max_iterations {
            let permutation = get_random_permutation(self.instance.size);
            let cost = self.instance.evaluate(permutation.as_ref());
            if cost < best_cost {
                best_cost = cost;
                best_permutation = permutation.clone();
                solution_changes += 1;
            }
        }
        Ok(Solution {
            permutation: best_permutation,
            evaluations: self.max_iterations,
            solution_changes,
        })
    }

    fn get_name(&self) -> String {
        "RandomSearchSolver".to_string()
    }

    fn get_instance(&self) -> &Instance {
        self.instance
    }
}

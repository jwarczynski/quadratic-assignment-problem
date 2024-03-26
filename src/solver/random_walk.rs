use rand::random;

use crate::instance::Instance;

use super::{compute_num_neighbours, eval_diff, move_to_neighbour, Solution, Solver, SolvingError};

pub struct RandomWalkSolver<'a> {
    instance: &'a Instance,
    max_iterations: usize,
    best_solution: Vec<usize>,
}

impl<'a> RandomWalkSolver<'a> {
    pub fn new(instance: &Instance, max_iterations: usize) -> RandomWalkSolver {
        RandomWalkSolver {
            instance,
            max_iterations,
            best_solution: Vec::with_capacity(instance.size),
        }
    }
}

impl<'a> Solver for RandomWalkSolver<'a> {
    fn solve(&mut self, mut initial_solution: Vec<usize>) -> Result<Solution, SolvingError> {
        let num_neighbours = compute_num_neighbours(self.instance.size);
        let mut evaluations = 0;
        let mut solution_changes = 0;
        for _ in 0..self.max_iterations {
            let random_neighbour_idx = random::<usize>() % num_neighbours;
            let diff = eval_diff(self.instance, &initial_solution, random_neighbour_idx);
            initial_solution = move_to_neighbour(initial_solution, random_neighbour_idx);
            evaluations += 1;

            if diff > 0 {
                self.best_solution = initial_solution.clone();
                solution_changes += 1;
            }
        }
        Ok(Solution {
            permutation: self.best_solution.clone(),
            evaluations,
            solution_changes,
        })
    }
    fn get_name(&self) -> String {
        "RandomWalkSolver".to_string()
    }

    fn get_instance(&self) -> &Instance {
        self.instance
    }
}

pub fn random_walk(
    instance: &Instance,
    mut initial_solution: Vec<usize>,
    max_iterations: usize,
) -> Vec<usize> {
    let mut best_solution = initial_solution.clone();
    for _ in 0..max_iterations {
        let random_neighbour_idx = random::<usize>() % instance.size;
        let diff = eval_diff(instance, &best_solution, random_neighbour_idx);
        initial_solution = move_to_neighbour(initial_solution, random_neighbour_idx);

        if diff > 0 {
            best_solution = initial_solution.clone();
        }
    }
    best_solution
}

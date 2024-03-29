use std::cmp::max;
use rand::random;

use crate::{
    instance::Instance,
    solver::{compute_num_neighbours, eval_diff, move_to_neighbour, Result, Solution, Solver},
};

pub struct SteepestSolver<'a> {
    instance: &'a Instance,
    max_time: u128,
}

impl<'a> SteepestSolver<'a> {
    pub fn new(instance: &Instance, max_time: u128) -> SteepestSolver {
        SteepestSolver { instance, max_time }
    }
}

impl<'a> Solver for SteepestSolver<'a> {
    fn solve(&mut self, mut starting_perm: Vec<usize>) -> Result<Solution> {
        let max_plateau_moves = 10;
        let num_neighbours = compute_num_neighbours(starting_perm.len());

        let mut solutions_evaluated = 0;
        let mut solutions_changes = 0;
        let mut best_neighbours_num = 0;
        let mut best_neighbours = vec![0; num_neighbours];
        let mut best_neighbour_diff = 0;
        let start = std::time::Instant::now();

        let mut plateau_moves = 0;
        loop {
            for neighbour_idx in 0..num_neighbours {
                match eval_diff(self.instance, &starting_perm, neighbour_idx) {
                    diff if diff == best_neighbour_diff && best_neighbour_diff == 0 => {
                        best_neighbours_num += 1;
                        best_neighbours[best_neighbours_num - 1] = neighbour_idx;

                    }
                    diff if diff > best_neighbour_diff => {
                        best_neighbour_diff = diff;
                        best_neighbours[0] = neighbour_idx;
                        best_neighbours_num = 1;
                        plateau_moves = 0;
                    }
                    _ => {}
                }
                if start.elapsed().as_nanos() > self.max_time {
                    break;
                }
            }

            if best_neighbours_num == 0 || plateau_moves >= max_plateau_moves || start.elapsed().as_nanos() > self.max_time {
                break;
            }

            if best_neighbour_diff == 0 {
                plateau_moves += 1;
            }

            let best_neighbour_idx = random::<usize>() % (best_neighbours_num);
            best_neighbours_num = 0;
            best_neighbour_diff = 0;
            solutions_evaluated += num_neighbours;
            starting_perm = move_to_neighbour(starting_perm, best_neighbours[best_neighbour_idx]);
            solutions_changes += 1;
        }

        Ok(Solution {
            permutation: starting_perm,
            evaluations: solutions_evaluated,
            solution_changes: solutions_changes,
        })
    }

    fn get_name(&self) -> String {
        "SteepestSolver".to_string()
    }

    fn get_instance(&self) -> &Instance {
        self.instance
    }

    fn set_time_limit(&mut self, time_limit: u128) {
        self.max_time = time_limit;
    }

    fn get_time_limit(&self) -> u128 {
        self.max_time
    }
}

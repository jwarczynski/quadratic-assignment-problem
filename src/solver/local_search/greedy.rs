use crate::{
    get_random_permutation,
    instance::Instance,
    solver::{
        compute_num_neighbours, eval_diff, move_to_neighbour, Solution, Solver, SolvingError,
    },
};

pub struct GreedySolver<'a> {
    instance: &'a Instance,
    max_time: u128,
}

impl<'a> GreedySolver<'a> {
    pub fn new(instance: &Instance, max_time: u128) -> GreedySolver {
        GreedySolver { instance, max_time }
    }
}

impl<'a> Solver for GreedySolver<'a> {
    fn solve(&mut self, mut starting_perm: Vec<usize>) -> Result<Solution, SolvingError> {
        let num_neighbours = compute_num_neighbours(starting_perm.len());

        let mut solutions_evaluated = 0;
        let mut solutions_changes = 0;
        let start = std::time::Instant::now();

        loop {
            let mut found_improvement = false;
            let mut best_neighbour_idx = 0;

            for neighbour_idx in get_random_permutation(num_neighbours) {
                let diff = eval_diff(self.instance, &starting_perm, neighbour_idx);
                solutions_evaluated += 1;

                if diff > 0 || start.elapsed().as_nanos() > self.max_time {
                    best_neighbour_idx = neighbour_idx;
                    found_improvement = true;
                    break;
                }
            }

            if !found_improvement || start.elapsed().as_nanos() > self.max_time {
                break;
            }

            starting_perm = move_to_neighbour(starting_perm, best_neighbour_idx);
            solutions_changes += 1;
        }

        Ok(Solution {
            permutation: starting_perm,
            evaluations: solutions_evaluated,
            solution_changes: solutions_changes,
        })
    }

    fn get_name(&self) -> String {
        "GreedySolver".to_string()
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

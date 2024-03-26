use crate::{
    get_random_permutation,
    instance::Instance,
    solver::{
        compute_num_neighbours, eval_diff, move_to_neighbour, Solution, Solver, SolvingError,
    },
};

pub struct GreedySolver<'a> {
    instance: &'a Instance,
}

impl<'a> GreedySolver<'a> {
    pub fn new(instance: &Instance) -> GreedySolver {
        GreedySolver { instance }
    }
}

impl<'a> Solver for GreedySolver<'a> {
    fn solve(&mut self, mut starting_perm: Vec<usize>) -> Result<Solution, SolvingError> {
        let num_neighbours = compute_num_neighbours(starting_perm.len());

        let mut solutions_evaluated = 0;
        let mut solutions_changes = 0;

        loop {
            let mut found_improvement = false;
            let mut best_neighbour_idx = 0;

            for neighbour_idx in get_random_permutation(num_neighbours) {
                let diff = eval_diff(self.instance, &starting_perm, neighbour_idx);
                solutions_evaluated += 1;

                if diff > 0 {
                    best_neighbour_idx = neighbour_idx;
                    found_improvement = true;
                    break;
                }
            }

            if !found_improvement {
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
}

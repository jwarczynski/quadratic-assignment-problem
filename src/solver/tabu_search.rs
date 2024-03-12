use std::collections::BinaryHeap;

use crate::instance;

use super::{calculate_swap_indices, eval_diff, move_to_neighbour};

const MAX_ITERATIONS_WITHOUT_IMPROVEMENT: usize = 100;
const CANDIDATE_LIST_SIZE: usize = 10;
const TABU_TENURE: usize = 5;

pub struct TabuSearchAlgorithm<'a> {
    instance: &'a instance::Instance,
    current_solution: Vec<usize>,
    best_solution: Vec<usize>,
    current_cost: usize,
    best_cost: usize,
    tabu_list: Vec<Vec<usize>>,
    iterations_without_improvement: usize,
    iteration: usize,
    candidate_threshold_cost: usize,
    candidate_list: BinaryHeap<(usize, isize)>,
}

impl<'a> TabuSearchAlgorithm<'a> {
    pub fn new(instance: &instance::Instance, initial_solution: Vec<usize>) -> TabuSearchAlgorithm {
        let n = instance.size;
        let tabu_list = vec![vec![0; n]; n];
        let candidate_list = BinaryHeap::with_capacity(CANDIDATE_LIST_SIZE);
        let candidate_threshold_cost = 0;
        let iterations_without_improvement = 0;
        let iteration = 0;
        let current_cost = instance.evaluate(&initial_solution);
        let best_solution = initial_solution.clone();

        TabuSearchAlgorithm {
            instance,
            current_solution: initial_solution,
            best_solution,
            current_cost,
            best_cost: current_cost,
            tabu_list,
            iterations_without_improvement,
            iteration,
            candidate_threshold_cost,
            candidate_list,
        }
    }

    pub fn solve(mut self) -> Vec<usize> {
        self.generate_candidate_list();

        while self.iterations_without_improvement < MAX_ITERATIONS_WITHOUT_IMPROVEMENT {
            let (neighbour_idx, cost_diff) = self.choose_from_candidate_list();

            // todo!("below 2 instructions should be in one method");
            self.current_solution = move_to_neighbour(self.current_solution, neighbour_idx);
            self.current_cost += cost_diff as usize;

            self.update_tabu_list(neighbour_idx);

            if self.current_cost < self.best_cost {
                self.best_solution = self.current_solution.clone();
                self.best_cost = self.current_cost;
                self.iterations_without_improvement = 0;
            } else {
                self.iterations_without_improvement += 1;
            }

            self.iteration += 1;
        }
        self.best_solution
    }

    fn generate_candidate_list(&mut self) {
        let num_neighbours = self.instance.size * (self.instance.size - 1) / 2;
        let mut worst_candidate_cost;

        for neighbour_idx in 0..CANDIDATE_LIST_SIZE {
            let diff = eval_diff(self.instance, &self.current_solution, neighbour_idx);
            self.candidate_list.push((neighbour_idx, diff as isize));
        }

        worst_candidate_cost = self
            .candidate_list
            .peek()
            .expect("Candidate list should not be empty")
            .1;

        for neighbour_idx in CANDIDATE_LIST_SIZE..num_neighbours {
            let diff = eval_diff(self.instance, &self.current_solution, neighbour_idx) as isize;
            if diff < worst_candidate_cost {
                self.candidate_list.pop();
                self.candidate_list.push((neighbour_idx, diff));
                worst_candidate_cost = self
                    .candidate_list
                    .peek()
                    .expect("Candidate list should not be empty")
                    .1;
            }
        }
    }

    fn choose_from_candidate_list(&mut self) -> (usize, isize) {
        let (worst_candidate_idx, worst_candidate_cost) = self
            .candidate_list
            .peek()
            .expect("Candidate list should not be empty");

        let mut best_candidate_cost = *worst_candidate_cost;
        let mut best_candidate_idx = *worst_candidate_idx;

        for (idx, diff) in self.candidate_list.iter() {
            let (i, j) = calculate_swap_indices(self.instance.size as isize, *idx as isize);
            if *diff < best_candidate_cost && self.tabu_list[i][j] == 0 {
                best_candidate_cost = *diff;
                best_candidate_idx = *idx
            }
        }

        if best_candidate_cost < self.candidate_threshold_cost as isize {
            (best_candidate_idx, best_candidate_cost)
        } else {
            self.candidate_list.clear();
            self.generate_candidate_list();
            self.choose_from_candidate_list()
        }
    }

    fn update_tabu_list(&mut self, neighbour_idx: usize) {
        let (i, j) = calculate_swap_indices(self.instance.size as isize, neighbour_idx as isize);
        self.tabu_list[i][j] = TABU_TENURE;
    }
}

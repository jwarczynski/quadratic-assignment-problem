use crate::instance::Instance;

use std::fmt;

type Result<T> = std::result::Result<T, SolvingError>;

#[derive(Debug)]
pub struct SolvingError {
    pub message: String,
}

impl fmt::Display for SolvingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while solving: {}", self.message)
    }
}

pub struct Solution {
    pub permutation: Vec<usize>,
    pub evaluations: usize,
    pub solution_changes: usize,
}

pub trait Solver {
    fn solve(&mut self, initial_solution: Vec<usize>) -> Result<Solution>;
    fn get_name(&self) -> String;
    fn get_instance(&self) -> &Instance;
    fn set_time_limit(&mut self, time_limit: u128);
}

// fn compute_cost(instance: &Instance, permutation: &[usize]) -> usize {
//     instance
//         .matrix_a
//         .iter()
//         .zip(permutation.iter().map(|&index| &instance.matrix_b[index]))
//         .map(|(row_a, row_b)| dot_product_permuted(row_a, row_b, permutation))
//         .sum()
// }
//
pub fn move_to_neighbour(mut perm: Vec<usize>, neighbour_idx: usize) -> Vec<usize> {
    let n = perm.len();
    let (swap_index_0, swap_index_1) = calculate_swap_indices(n as isize, neighbour_idx as isize);

    perm.swap(swap_index_0, swap_index_1);
    perm
}

fn calculate_swap_indices(n: isize, k: isize) -> (usize, usize) {
    let numerator = -8 * k + 4 * n * (n - 1) - 7;
    let i = n - 2 - ((numerator as f64).sqrt() / 2.0 - 0.5).floor() as isize;

    let j = k + i + 1 - n * (n - 1) / 2 + (n - i) * ((n - i) - 1) / 2;

    (i as usize, j as usize)
}

fn compute_num_neighbours(n: usize) -> usize {
    n * (n - 1) / 2
}

fn eval_diff(instance: &Instance, perm: &[usize], neighbour_idx: usize) -> i32 {
    let n = perm.len();
    let (swap_index_0, swap_index_1) = calculate_swap_indices(n as isize, neighbour_idx as isize);

    let mut before_swap = dot_product_permuted(
        &instance.matrix_a[swap_index_0],
        &instance.matrix_b[perm[swap_index_0]],
        perm,
    ) + dot_product_permuted(
        &instance.matrix_a[swap_index_1],
        &instance.matrix_b[perm[swap_index_1]],
        perm,
    );

    let mut after_swap = dot_product_permuted_with_swap(
        &instance.matrix_a[swap_index_0],
        &instance.matrix_b[perm[swap_index_1]],
        perm,
        swap_index_0,
        swap_index_1,
    ) + dot_product_permuted_with_swap(
        &instance.matrix_a[swap_index_1],
        &instance.matrix_b[perm[swap_index_0]],
        perm,
        swap_index_0,
        swap_index_1,
    );

    for i in 0..n {
        if i != swap_index_0 && i != swap_index_1 {
            let row_a = &instance.matrix_a[i];
            let row_b = &instance.matrix_b[perm[i]];
            before_swap += row_a[swap_index_0] * row_b[perm[swap_index_0]]
                + row_a[swap_index_1] * row_b[perm[swap_index_1]];

            after_swap += row_a[swap_index_1] * row_b[perm[swap_index_0]]
                + row_a[swap_index_0] * row_b[perm[swap_index_1]];
        }
    }

    before_swap as i32 - after_swap as i32
}

pub fn dot_product_permuted(row_a: &[usize], row_b: &[usize], perm: &[usize]) -> usize {
    row_a
        .iter()
        .zip(perm.iter().map(|&index| row_b[index]))
        .map(|(&a, b)| a * b)
        .sum()
}

pub fn dot_product_permuted_with_swap(
    row_a: &[usize],
    row_b: &[usize],
    perm: &[usize],
    swap_idx_0: usize,
    swap_idx_1: usize,
) -> usize {
    let mut sum = 0;
    for i in 0..row_a.len() {
        if i == swap_idx_0 {
            sum += row_a[i] * row_b[perm[swap_idx_1]];
        } else if i == swap_idx_1 {
            sum += row_a[i] * row_b[perm[swap_idx_0]];
        } else {
            sum += row_a[i] * row_b[perm[i]];
        }
    }
    sum
}
pub mod heuristic_solver;
pub mod local_search;
pub mod random_search;
pub mod random_walk;
pub mod simulated_annealing;
pub mod tabu_search;

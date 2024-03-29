use std::usize;

use rand::random;

use super::compute_num_neighbours;
use super::eval_diff;
use super::move_to_neighbour;
use super::Solution;
use super::SolvingError;
use crate::get_random_permutation;
use crate::instance::Instance;

const NUM_INITIAL_TEMPERATURE_SAMPLES: usize = 100;
const MAX_NO_IMPROVEMENT_ITERAIONS: usize = 200_000;

pub struct SimulatedAnnealingSolver<'a> {
    instance: &'a Instance,
    max_time: u128,
}

impl<'a> SimulatedAnnealingSolver<'a> {
    pub fn new(instance: &Instance, max_time: u128) -> SimulatedAnnealingSolver {
        SimulatedAnnealingSolver { instance, max_time }
    }
}

impl<'a> super::Solver for SimulatedAnnealingSolver<'a> {
    fn solve(&mut self, starting_perm: Vec<usize>) -> Result<Solution, SolvingError> {
        Ok(simulated_annealing(self.instance, starting_perm))
    }

    fn get_name(&self) -> String {
        "Simulated Annealing".to_string()
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

pub fn simulated_annealing(instance: &Instance, mut starting_solution: Vec<usize>) -> Solution {
    let mut temperature = set_initial_temperatrue(instance);
    let neighbours_num = compute_num_neighbours(instance.get_size());
    let iter_per_temperature = 1000;
    let mut no_improvement_iterations = 0;

    while no_improvement_iterations < MAX_NO_IMPROVEMENT_ITERAIONS {
        for _ in 0..iter_per_temperature {
            let neighbour_idx = generate_random_neighbour(neighbours_num);
            let delta = eval_diff(instance, &starting_solution, neighbour_idx);

            if delta > 0 {
                starting_solution = move_to_neighbour(starting_solution, neighbour_idx);
                no_improvement_iterations = 0;
            } else {
                no_improvement_iterations += 1;
                let probability = calculate_probability(delta as f64, temperature);
                if probability > generate_random_number() {
                    starting_solution = move_to_neighbour(starting_solution, neighbour_idx);
                }
            }
        }

        temperature = update_temperature(temperature);
    }

    Solution {
        permutation: starting_solution,
        evaluations: 0,
        solution_changes: 0,
    }
}

fn set_initial_temperatrue(instance: &Instance) -> f64 {
    // "
    //     uniformly sample solution space
    //     calculate the average delta
    //     solve e^(-avg_delta / temperature) = 0.9
    // "

    let mut total_delta = 0.0;
    for _ in 0..NUM_INITIAL_TEMPERATURE_SAMPLES {
        let perm = get_random_permutation(instance.get_size());
        let idx = random::<usize>() % compute_num_neighbours(instance.get_size());
        let delta = eval_diff(instance, &perm, idx).abs();
        total_delta += delta as f64;
    }

    let avg_delta = total_delta / NUM_INITIAL_TEMPERATURE_SAMPLES as f64;
    -avg_delta / (0.9f64.ln())
}

fn generate_random_neighbour(num_neighbours: usize) -> usize {
    rand::random::<usize>() % num_neighbours
}

fn calculate_probability(delta: f64, temperature: f64) -> f64 {
    (delta / temperature).exp()
}

fn generate_random_number() -> f64 {
    rand::random::<f64>()
}

fn update_temperature(temperature: f64) -> f64 {
    temperature * 0.95
}

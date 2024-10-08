pub mod io;

use instance::Instance;
use solver::Solver;

pub struct Metrics {
    pub duration: u128,
    pub instance_name: String,
    pub cost: usize,
    pub evaluated_solutions: usize,
    pub solution_changes: usize,
    pub optimal_cost: usize,
    pub initial_cost: usize,
    pub time_limit: u128,
    pub solution_distance: usize,
}

pub fn measure_time(
    solver: &mut dyn Solver,
    instance: &Instance,
    instance_name: &str,
    min_runs: usize,
) -> Vec<Metrics> {
    let mut iteration: usize = 0;
    let mut total_elapsed = 0;
    let mut total_cost = 0;
    let mut metrics: Vec<Metrics> = Vec::new();
    while total_elapsed < 1 || iteration < min_runs as usize {
        let starting_perm = get_random_permutation(instance.get_size());
        let initial_cost = instance.evaluate(starting_perm.as_ref());
        let start = std::time::Instant::now();
        let solution = solver.solve(starting_perm).expect("Failed to solve");
        let elapsed = start.elapsed().as_nanos();
        total_elapsed += elapsed;
        let cost = solver.get_instance().evaluate(&solution.permutation);
        total_cost += cost;
        iteration += 1;

        metrics.push(Metrics {
            duration: elapsed,
            instance_name: instance_name.to_string(),
            cost,
            evaluated_solutions: solution.evaluations,
            solution_changes: solution.solution_changes,
            optimal_cost: solver.get_instance().optimal_cost,
            initial_cost,
            time_limit: solver.get_time_limit(),
            solution_distance: instance.get_solutions_distance(&solution.permutation),
        });
    }
    metrics
}

pub mod instance {
    use crate::argsort;
    use crate::solver::dot_product_permuted;

    #[derive(Debug)]
    pub struct Instance {
        pub matrix_a: Vec<Vec<usize>>,
        pub matrix_b: Vec<Vec<usize>>,
        pub size: usize,
        pub optimal_cost: usize,
        pub optimal_permutation: Vec<usize>,
    }

    impl Instance {
        pub fn new(
            matrix_a: Vec<Vec<usize>>,
            matrix_b: Vec<Vec<usize>>,
            optimal_cost: usize,
            optimal_permutation: Vec<usize>,
        ) -> Instance {
            let size = matrix_a.len();
            Instance {
                matrix_a,
                matrix_b,
                size,
                optimal_cost,
                optimal_permutation,
            }
        }

        pub fn get_solutions_distance(&self, perm: &Vec<usize>) -> usize {
            self.optimal_permutation
                .iter().zip(perm.iter())
                .map(|(p1,p2)| p2 != p1)
                .filter(|&x| x)
                .count()
        }

        pub fn get_matrix_a(&self) -> &Vec<Vec<usize>> {
            &self.matrix_a
        }

        pub fn get_matrix_b(&self) -> &Vec<Vec<usize>> {
            &self.matrix_b
        }

        pub fn get_size(&self) -> usize {
            self.size
        }

        pub fn evaluate(&self, solution: &[usize]) -> usize {
            let matrix_a = &self.matrix_a;
            let matrix_b = &self.matrix_b;

            matrix_a
                .iter()
                .zip(solution.iter().map(|&index| &matrix_b[index]))
                .map(|(row_a, row_b)| dot_product_permuted(row_a, row_b, solution))
                .sum()
        }
    }

    pub struct Solver<'a> {
        instance: &'a Instance,
    }

    impl<'q> Solver<'q> {
        pub fn new(instance: &'q Instance) -> Solver<'q> {
            Solver { instance }
        }

        pub fn solve(&self) -> (Box<[usize]>, Box<[usize]>, usize) {
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

            let a_cols_sums = self.get_columns_sum(&self.instance.matrix_a);
            let b_cols_sums = self.get_columns_sum(&self.instance.matrix_a);

            let row_permutation = self.greedy_mapping(&*a_rows_sums, &*b_rows_sums);
            let col_permutation = self.greedy_mapping(&*a_cols_sums, &*b_cols_sums);
            let cost = self.compute_cost(
                &*row_permutation,
                &*col_permutation,
                &self.instance.matrix_a,
                &self.instance.matrix_b,
            );
            println!("row permutation: {:?}", row_permutation);
            println!("col permutation: {:?}", col_permutation);
            println!("cost: {}", cost);

            (row_permutation, col_permutation, cost)
        }

        fn get_columns_sum(&self, matrix: &Vec<Vec<usize>>) -> Vec<usize> {
            let size = matrix.len();
            (0..size)
                .map(|col| matrix.iter().map(|row| row[col]).sum())
                .collect()
        }

        fn greedy_mapping(&self, sums_a: &[usize], sums_b: &[usize]) -> Box<[usize]> {
            let sorted_indices_a = argsort(sums_a, true);
            let sorted_indices_b = argsort(sums_b, false);

            let mut permuatation = vec![0; sums_a.len()];
            for i in 0..sums_a.len() {
                permuatation[sorted_indices_a[i]] = sorted_indices_b[i];
            }

            permuatation.into_boxed_slice()
        }

        fn compute_cost(
            &self,
            row_permutation: &[usize],
            col_permutation: &[usize],
            matrix_a: &Vec<Vec<usize>>,
            matrix_b: &Vec<Vec<usize>>,
        ) -> usize {
            let mut cost = 0;
            for i in 0..row_permutation.len() {
                for j in 0..row_permutation.len() {
                    cost += matrix_a[i][j] * matrix_b[row_permutation[i]][col_permutation[j]];
                }
            }
            cost
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn permutation_correct() {
            let sums_a = [2, 4, 5, 1];
            let sums_b = [1, 7, 2, 3];
            todo!("add optimal cost to instance");
            let instance = Instance {
                matrix_a: vec![
                    vec![1, 1, 0, 0],
                    vec![0, 1, 1, 1],
                    vec![1, 0, 1, 1],
                    vec![0, 0, 1, 0],
                ],
                matrix_b: vec![
                    vec![0, 1, 0, 0],
                    vec![1, 1, 1, 1],
                    vec![0, 0, 1, 1],
                    vec![0, 1, 0, 0],
                ],
                size: 4,
                optimal_cost: 0,
            };
            let solver = Solver::new(&instance);
            let permutation = solver.greedy_mapping(&sums_a, &sums_b);

            assert_eq!([3, 2, 0, 1], *permutation);
        }
    }
}

pub mod solver;

pub fn get_random_permutation(size: usize) -> Vec<usize> {
    let mut numbers: Vec<usize> = (0..size).collect();
    let mut permuatation: Vec<usize> = vec![0; size];
    for i in 0..size - 1 {
        let index = rand::random::<usize>() % (size - i);
        permuatation[i] = numbers[index];
        numbers.swap_remove(index);
    }
    permuatation[size - 1] = numbers[0];
    permuatation
}

pub fn get_random_pair(n: usize) -> (usize, usize) {
    let x1: usize = rand::random::<usize>() % n;
    let x2: usize = (rand::random::<usize>() % (n - 1) + x1 + 1) % n;
    (x1, x2)
}

pub fn argsort<T: Ord>(arr: &[T], ascending: bool) -> Box<[usize]> {
    let mut indices: Vec<usize> = (0..arr.len()).collect();

    if ascending {
        indices.sort_by(|&a, &b| arr[a].cmp(&arr[b]));
    } else {
        indices.sort_by(|&a, &b| arr[b].cmp(&arr[a]));
    }

    indices.into_boxed_slice()
}

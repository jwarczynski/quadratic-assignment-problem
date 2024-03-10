pub mod io;

use instance::Instance;
use rand;

pub fn measure_time<F>(f: F, instance: &Instance, startting_perm: Vec<usize>) -> (u128, usize)
where
    F: Fn(&Instance, Vec<usize>) -> Vec<usize>,
{
    let mut iteration: usize = 0;
    let mut total_elapsed = 0;
    while total_elapsed < 1 || iteration < 10 {
        let startting_perm = startting_perm.clone();
        let start = std::time::Instant::now();
        f(instance, startting_perm);
        total_elapsed += start.elapsed().as_nanos();
        iteration += 1;
    }
    (total_elapsed, iteration)
}

pub mod instance {
    use crate::argsort;
    use crate::solver::dot_product_permuted;

    #[derive(Debug)]
    pub struct Instance {
        pub matrix_a: Vec<Vec<usize>>,
        pub matrix_b: Vec<Vec<usize>>,
        pub size: usize,
    }

    impl Instance {
        pub fn new(matrix_a: Vec<Vec<usize>>, matrix_b: Vec<Vec<usize>>) -> Instance {
            let size = matrix_a.len();
            Instance {
                matrix_a,
                matrix_b,
                size,
            }
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

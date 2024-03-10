use rand::random;

use super::compute_num_neighbours;
use super::eval_diff;
use super::move_to_neighbour;
use crate::instance::Instance;

pub fn local_search(instance: &Instance, mut starting_perm: Vec<usize>) -> Vec<usize> {
    let mut best_neighbours_num = 0;
    let num_neighbours = compute_num_neighbours(starting_perm.len());
    let mut best_neighbours = vec![0; num_neighbours];
    let mut best_neighbour_diff = 0;

    // let mut iter = 0;
    loop {
        // iter += 1;
        for neighbour_idx in 0..num_neighbours {
            match eval_diff(instance, &starting_perm, neighbour_idx) {
                diff if diff == best_neighbour_diff => {
                    best_neighbours_num += 1;
                    best_neighbours[best_neighbours_num - 1] = neighbour_idx;
                }
                diff if diff > best_neighbour_diff => {
                    best_neighbour_diff = diff;
                    best_neighbours[0] = neighbour_idx;
                    best_neighbours_num = 1;
                }
                _ => {}
            }
        }
        // println!(
        //     "iter: {:?}, best_neighbour_diff: {:?}, current perm: {:?}",
        //     iter, best_neighbour_diff, starting_perm,
        // );

        if best_neighbours_num == 0 {
            break;
        }

        let best_neighbour_idx = random::<usize>() % (best_neighbours_num);
        best_neighbours_num = 0;
        best_neighbour_diff = 0;
        starting_perm = move_to_neighbour(starting_perm, best_neighbours[best_neighbour_idx]);
    }

    starting_perm
}

use quadratic_assignment_problem::measure_time;
use quadratic_assignment_problem::solver::local_search::local_search;
use quadratic_assignment_problem::solver::simulated_annealing::simulated_annealing;
use quadratic_assignment_problem::{get_random_permutation, io::InstanceReader};

fn main() {
    let instance_reader = InstanceReader::new("qap/instances");
    let instance = instance_reader
        .read_instance("chr12a.dat")
        .expect("Failed to read instance file");

    let perm = get_random_permutation(instance.size);
    println!("starting perm: {:?}", perm);
    // let (total_time, iterations) = measure_time(local_search, &instance, perm);
    // println!(
    //     "total time: {:?}\t avg time: {:?}\t iterations: {:?}",
    //     total_time,
    //     (total_time / iterations as u128),
    //     iterations
    // );
    // let solution = local_search(&instance, perm);
    // let optimum_perm = [6, 4, 11, 1, 0, 2, 8, 10, 9, 5, 7, 3];
    // let optimum_cost = instance.evaluate(&optimum_perm);
    // println!("optimum perm: {:?}\t cost: {optimum_cost}", optimum_perm);

    let (total_time, iterations) = measure_time(simulated_annealing, &instance, perm);
    println!(
        "total time: {:?}\t avg time: {:?}\t iterations: {:?}",
        total_time / 1_000_000,
        (total_time / 1_000_000 / iterations as u128),
        iterations
    );
    // let solution = simulated_annealing(&instance, perm);
    // let cost = instance.evaluate(&solution);
    // println!("solution perm: {:?}\t cost: {cost}", solution);
    // let solver = instance::Solver::new(&instance);
    // let solution = solver.solve();
    //
    // println!("{:?}", solution);
    // println!("{:?}", get_random_pair(2));
    // println!("{:?}", get_random_permutation(5));
}

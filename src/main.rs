use quadratic_assignment_problem::io::experiments::{
    run_alg_with_time_constrains, run_all_algorithms, run_all_algorithms_with_time_constrains,
};
use quadratic_assignment_problem::io::save_metrics_to_csv;
// use quadratic_assignment_problem::measure_time;
// use quadratic_assignment_problem::solver::random_search::RandomSearchSolver;
// use quadratic_assignment_problem::solver::random_walk::RandomWalkSolver;
// use quadratic_assignment_problem::solver::simulated_annealing::SimulatedAnnealingSolver;
use quadratic_assignment_problem::solver::{heuristic_solver, local_search, Solver};
use quadratic_assignment_problem::{get_random_permutation, io::InstanceReader, measure_time};
use quadratic_assignment_problem::solver::local_search::greedy::GreedySolver;
use quadratic_assignment_problem::solver::local_search::steepest::SteepestSolver;
use quadratic_assignment_problem::solver::random_walk::RandomWalkSolver;

fn main() {
    let instances = ["chr12a", "chr15a", "chr18a", "chr20a", "chr22a", "chr25a"];
    let instances = ["lipa20a", "lipa30a", "lipa40a", "lipa50a", "lipa60a", "lipa70a", "lipa80a", "lipa90a"];

    let instance_reader = InstanceReader::new("qap/instances");
    let instance = instance_reader
        .read_instance(instances[0])
        .expect("Failed to read instance file");

    println!("{:?}", instance.matrix_a[0].len());

    let file = std::fs::File::open("qap/instances/lipa30a.dat").expect("Failed to open file");
    let reader = std::io::BufReader::new(file);

    let mut solver = RandomWalkSolver::new(&instance, 10_000, 2_000_000);
    // let solution = solver.solve(get_random_permutation(instance.size)).unwrap();
    // run_all_algorithms(&instances, "lipa");

    // let mut limits: Vec<u128> = Vec::new();
    // for exp in 4..=10 {
    //     let num = 5 * 10u64.pow(exp);
    //     limits.push(num as u128);
    // }
    //
    // let current_instance = instances[5];
    //
    // let instance_reader = InstanceReader::new("qap/instances");
    // let instance = instance_reader
    //     .read_instance(instances[1])
    //     .expect("Failed to read instance file");
    //
    // let mut steepest_solver = Box::new(SteepestSolver::new(
    //     &instance, u128::MAX,
    // ));
    //
    // let mut greedy_solver = Box::new(GreedySolver::new(
    //     &instance, u128::MAX,
    // ));
    //
    // let metrics1 = measure_time(&mut *steepest_solver, &instance, current_instance, 10);
    // let metrics2 = measure_time(&mut *greedy_solver, &instance, current_instance, 10);
    // let _ = save_metrics_to_csv(&format!("output/{}/{}.csv", current_instance ,steepest_solver.get_name()), &metrics1);
    // let _ = save_metrics_to_csv(&format!("output/{}/{}.csv", current_instance, greedy_solver.get_name()), &metrics2);
    //

    // run_all_algorithms_with_time_constrains();
    // run_alg_with_time_constrains("GreedySolver", instances[0], &limits);
    // for mut solver in solvers {
    // println!(
    //     "{:?}:\ttotal time: {:?}\t avg time: {:?}\t avg cost: {:?}\t iterations: {:?}",
    //     solver.get_name(),
    //     total_time,
    //     (total_time / iterations as u128),
    //     avg_cost,
    //     iterations
    // );
    // }

    // let mut random_search_solver = RandomSearchSolver::new(&instance, 10_000);
    // let random_search_result = random_search_solver.solve(perm).unwrap();
    // println!("random search result: {:?}", random_search_result);
    // println!(
    //     "cost: {:?}",
    //     instance.evaluate(random_search_result.as_ref())
    // );
    //
    // let mut random_walk_solver = RandomWalkSolver::new(&instance, 10_000_000);
    // let random_walk_result = random_walk_solver.solve(perm).unwrap();
    // println!("random walk result: {:?}", random_walk_result);
    // println!("cost: {:?}", instance.evaluate(random_walk_result.as_ref()));
    //
    // let mut greedy_solver = greedy_solver::GreedySolver::new(&instance);
    // let greedy_result = greedy_solver.solve(perm).unwrap();
    // println!("greedy result: {:?}", greedy_result);
    // println!("cost: {:?}", instance.evaluate(greedy_result.as_ref()));

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

    // let (total_time, iterations) = measure_time(simulated_annealing, &instance, perm);
    // println!(
    //     "total time: {:?}\t avg time: {:?}\t iterations: {:?}",
    //     total_time / 1_000_000,
    //     (total_time / 1_000_000 / iterations as u128),
    //     iterations,
    // );
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

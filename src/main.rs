use quadratic_assignment_problem::io::save_metrics_to_csv;
use quadratic_assignment_problem::measure_time;
use quadratic_assignment_problem::solver::random_search::RandomSearchSolver;
use quadratic_assignment_problem::solver::random_walk::RandomWalkSolver;
use quadratic_assignment_problem::solver::simulated_annealing::SimulatedAnnealingSolver;
use quadratic_assignment_problem::solver::{heuristic_solver, local_search, Solver};
use quadratic_assignment_problem::{get_random_permutation, io::InstanceReader};

fn main() {
    let instance_reader = InstanceReader::new("qap/instances");

    let instances = ["chr12a", "chr15a", "chr18a", "chr20a", "chr22a", "chr25a"];

    // for mut solver in solvers {
    instances.iter().for_each(|instance_name| {
        let instance = instance_reader
            .read_instance(instance_name)
            .expect("Failed to read instance file");

        let perm = get_random_permutation(instance.size);
        println!("{:?}:\tstarting perm: {:?}", instance_name, perm);

        let mut solvers: Vec<Box<dyn Solver>> = vec![
            Box::new(RandomSearchSolver::new(&instance, 10_000, 150_000_000)),
            Box::new(RandomWalkSolver::new(&instance, 10_000, 150_000_000)),
            Box::new(heuristic_solver::HeuristicSolver::new(&instance)),
            Box::new(local_search::greedy::GreedySolver::new(
                &instance,
                150_000_000,
            )),
            Box::new(local_search::steepest::SteepesSolver::new(
                &instance,
                150_000_000,
            )),
            // Box::new(SimulatedAnnealingSolver::new(&instance)),
        ];

        solvers.iter_mut().for_each(|solver| {
            println!("{:?}", solver.get_name());
            let metrics = measure_time(&mut **solver, &instance, perm.clone(), instance_name);
            let _ = save_metrics_to_csv(&format!("output/{}.csv", solver.get_name()), &metrics);
        });
    });

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

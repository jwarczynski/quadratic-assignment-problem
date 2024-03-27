use crate::{
    get_random_permutation,
    io::{save_metrics_to_csv, InstanceReader},
    measure_time,
    solver::{
        heuristic_solver, local_search, random_search::RandomSearchSolver,
        random_walk::RandomWalkSolver, Solver,
    },
};

pub fn run_all_algorithms(instances: &[&str]) {
    // for mut solver in solvers {
    instances.iter().for_each(|instance_name| {
        let instance_reader = InstanceReader::new("qap/instances");
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
            let _ =
                save_metrics_to_csv(&format!("output/times/{}.csv", solver.get_name()), &metrics);
        });
    });
}

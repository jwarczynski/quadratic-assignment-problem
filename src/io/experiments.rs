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

pub fn run_alg_with_time_constrains(solver: &mut dyn Solver, instance_name: &str, limits: &[u128]) {
    let instance_reader = InstanceReader::new("qap/instances");
    let instance = instance_reader
        .read_instance(instance_name)
        .expect("Failed to read instance file");

    let perm = get_random_permutation(instance.size);
    println!("{:?}:\tstarting perm: {:?}", instance_name, perm);

    limits.iter().for_each(|limit| {
        solver.set_time_limit(*limit);
        let metrics = measure_time(solver, &instance, perm.clone(), instance_name);
        let _ = save_metrics_to_csv(
            &format!("output/times2/{}.csv", solver.get_name()),
            &metrics,
        );
    });
}

pub fn run_all_algorithms_with_time_constrains() {
    let instance_reader = InstanceReader::new("qap/instances");
    let instance = instance_reader
        .read_instance("chr12a")
        .expect("Failed to read instance file");

    let mut steepst_solver = Box::new(local_search::steepest::SteepesSolver::new(
        &instance, 10_000_000,
    ));

    for limit in (10_000_000..150_000_000).step_by(10_000_000) {
        steepst_solver.set_time_limit(limit);
        let metrics = measure_time(
            &mut *steepst_solver,
            &instance,
            get_random_permutation(instance.size),
            "chr12a",
        );

        let _ = save_metrics_to_csv("output/times2/steepst.csv", &metrics);
    }
}

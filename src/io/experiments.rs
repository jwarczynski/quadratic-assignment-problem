use std::usize;
use crate::{
    get_random_permutation,
    io::{save_metrics_to_csv, InstanceReader},
    measure_time,
    solver::{
        heuristic_solver, local_search, random_search::RandomSearchSolver,
        random_walk::RandomWalkSolver, Solver,
    },
};
use crate::instance::Instance;

pub fn initial_quality_experiment(instances: &[&str], out_dir: &str, runs: usize) {
    let instance_reader = InstanceReader::new("qap/instances");
    for instance_name in instances {
        let instance = instance_reader
            .read_instance(instance_name)
            .expect("Failed to read instance file");

        let mut solvers: Vec<Box<dyn Solver>> = get_local_search_solvers(&instance, u128::MAX);
        solvers.iter_mut().for_each(|solver| {
            println!("{:?}", solver.get_name());
            let metrics = measure_time(&mut **solver, &instance, instance_name, runs);
            let _ =
                save_metrics_to_csv(&format!("output/{}/{}.csv", out_dir, solver.get_name()), &metrics);
        });
    }
}

pub fn run_all_algorithms(instances: &[&str], out_dir: &str, limits: &[u128]) {
    let instance_reader = InstanceReader::new("qap/instances");
    for (i, instance_name) in instances.iter().enumerate() {
        let instance = instance_reader
            .read_instance(instance_name)
            .expect("Failed to read instance file");

        let mut solvers: Vec<Box<dyn Solver>> = get_all_solvers(&instance, u128::MAX);
        solvers.iter_mut().for_each(|solver| {
            println!("{:?}", solver.get_name());
            solver.set_time_limit(limits[i]);
            let metrics = measure_time(&mut **solver, &instance, instance_name, 10);
            let _ =
                save_metrics_to_csv(&format!("output/{}/{}.csv", out_dir, solver.get_name()), &metrics);
        });
    };
}

pub fn run_alg_with_time_constrains(solver_name: &str, instance_name: &str, time_limits: &[u128]) {
    let instance_reader = InstanceReader::new("qap/instances");
    let instance = instance_reader
        .read_instance(instance_name)
        .expect("Failed to read instance file");

    let mut solvers = get_all_solvers(&instance, 2_500_000);
    let mut solver = solvers.iter_mut().find(|s| s.get_name() == solver_name).expect("Solver not found");

    time_limits.iter().for_each(|limit| {
        solver.set_time_limit(*limit);
        let metrics = measure_time(&mut **solver, &instance, instance_name, 100);
        let _ = save_metrics_to_csv(
            &format!("output/times2/{}.csv", solver.get_name()),
            &metrics,
        );
    });
}

fn get_all_solvers<'i>(instance: &'i Instance, max_time: u128) -> Vec<Box<dyn Solver + 'i>> {
    let mut solvers: Vec<Box<dyn Solver>> = vec![
        Box::new(RandomSearchSolver::new(&instance, usize::MAX, max_time)),
        Box::new(RandomWalkSolver::new(&instance, usize::MAX, max_time)),
        Box::new(heuristic_solver::HeuristicSolver::new(&instance)),
        Box::new(local_search::greedy::GreedySolver::new(
            &instance,
            max_time,
        )),
        Box::new(local_search::steepest::SteepestSolver::new(
            &instance,
            max_time,
        )),
    ];
    solvers
}

pub fn get_local_search_solvers<'i>(instance: &'i Instance, max_time: u128) -> Vec<Box<dyn Solver + 'i>> {
    vec![
        Box::new(local_search::greedy::GreedySolver::new(&instance, max_time)),
        Box::new(local_search::steepest::SteepestSolver::new(&instance, max_time)),
    ]
}

pub fn run_all_algorithms_with_time_constrains() {
    let instance_reader = InstanceReader::new("qap/instances");
    let instance = instance_reader
        .read_instance("chr12a")
        .expect("Failed to read instance file");

    let mut steepest_solver = Box::new(local_search::steepest::SteepestSolver::new(
        &instance, 10_000_000,
    ));

    let mut limits = Vec::new();
    for exp in 2..=10 {
        let num = 10u64.pow(exp);
        limits.push(num);
    }
    // for limit in (1_000..=15_000).step_by(1000) {
    for limit in limits {
        steepest_solver.set_time_limit(limit as u128);
        let metrics = measure_time(
            &mut *steepest_solver,
            &instance,
            "chr12a",
            100,
        );

        let _ = save_metrics_to_csv(
            &format!("output/times2/{}.csv", steepest_solver.get_name()),
            &metrics,
        );
    }
}

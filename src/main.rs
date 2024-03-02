use quadratic_assignment_problem::{io::InstanceReader, get_random_pair, instance, get_random_permutation};

fn main() {
    let instance_reader = InstanceReader::new("qap/instances");
    let instance  = instance_reader.read_instance("chr12a.dat").expect("Failed to read instance file");
    let solver = instance::Solver::new(&instance);
    let solution = solver.solve();

    println!("{:?}", solution);
    println!("{:?}", get_random_pair(2));
    println!("{:?}", get_random_permutation(5));
}

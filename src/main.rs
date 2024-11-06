mod io_operations;
mod types;
mod solvers;
use types::GraphType;
use solvers::{BruteForceSolver, GreedySolver, RandomSolver, Solver};

fn main() {
    println!("Given graph:");
    let graph: GraphType =
        io_operations::read_graph_fron_file("6x6_105.atsp").expect("Failed to read graph");
    io_operations::display_graph(&graph);
    
    println!("\nRandomSolver SOLVING...");

    let mut random_solver:RandomSolver = solvers::RandomSolver::new();
    let (cost, path) = random_solver.solve_from_node(&graph, 1).unwrap();

    println!("Cost: {}", cost);
    io_operations::display_path(&path, " -> ");
    
    println!("\nGreedySolver SOLVING...");

    let mut greedy_solver:GreedySolver = solvers::GreedySolver::new();
    let (cost, path) = greedy_solver.solve_from_node(&graph, 5).unwrap();

    println!("Cost: {}", cost);
    io_operations::display_path(&path, " -> ");
    
    println!("\nBruteForceSolver SOLVING...");

    let mut brute_force_solver:BruteForceSolver = solvers::BruteForceSolver::new();
    let (cost, path) = brute_force_solver.solve_from_node(&graph, 5).unwrap();

    println!("Cost: {}", cost);
    io_operations::display_path(&path, " -> ");
}

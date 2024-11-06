use crate::types::{GraphType, IndexType, PathType, ValueType};
use rand::Rng;
use permutohedron::Heap;

fn calculate_cost(graph: &GraphType, path: &PathType) -> ValueType{
    let mut cost: ValueType = 0;
    for i in 0..path.len()-1{
        let current_node: IndexType = path[i];
        let next_node: IndexType = path[i+1];
        cost += graph[current_node][next_node];
    }
    cost
}

pub trait Solver{
    fn solve_from_node(&mut self, graph: &GraphType, starting_node: IndexType) -> Option<(ValueType,PathType)>;
    fn solve(&mut self, graph: &GraphType) -> Option<(ValueType,PathType)>;
    fn get_cost(&self) ->  Option<ValueType>;
    fn get_path(&self) ->  Option<PathType>;
}

pub struct RandomSolver{
    optimal_cost: Option<ValueType>,
    optimal_path: Option<PathType>,
}

impl RandomSolver 
{
    pub fn new() -> Self{
        Self{
            optimal_path: None,
            optimal_cost: None,
        }
    }
}

impl Solver for RandomSolver {
    fn solve_from_node(&mut self, graph: &GraphType, starting_node: IndexType) -> Option<(ValueType,PathType)>{
        if starting_node >= graph.len(){
            return None;
        }

        let problem_size = graph.len();
        let mut path: PathType = Vec::new();
        let mut unvisited: Vec<IndexType> = Vec::from_iter(0..problem_size);

        path.push(starting_node);
        unvisited.remove(starting_node);

        while !unvisited.is_empty(){
            let unvisited_nodes = unvisited.len();
            let chosen_index: usize = rand::thread_rng().gen_range(0..unvisited_nodes);
            let next_node:IndexType = unvisited.remove(chosen_index);
            path.push(next_node);           
        }

        path.push(starting_node);

        self.optimal_path = Some(path.clone());

        let cost:ValueType = calculate_cost(graph, &path);
        self.optimal_cost = Some(cost);

        Some((cost, path))
        
    }

    fn solve(&mut self, graph: &GraphType) -> Option<(ValueType,PathType)> {
        self.solve_from_node(graph, 0)
    }

    fn get_cost(&self) ->  Option<ValueType> {
        self.optimal_cost
    }

    fn get_path(&self) ->  Option<PathType> {
        self.optimal_path.clone()
    }
}


pub struct GreedySolver{
    optimal_cost: Option<ValueType>,
    optimal_path: Option<PathType>,
}

impl GreedySolver 
{
    pub fn new() -> Self{
        Self{
            optimal_path: None,
            optimal_cost: None,
        }
    }

    fn find_nearest_node(&self, graph: &GraphType, current_node: IndexType, unvisited: &Vec<IndexType>) -> IndexType{
        let mut min_cost: ValueType = ValueType::MAX;
        let mut nearest_node_index: IndexType = 0;
        for i in 0..unvisited.len(){
            let next_node = unvisited[i];
            let cost = graph[current_node][next_node];
            if cost < min_cost{
                min_cost = cost;
                nearest_node_index = i;
            }
        }
        nearest_node_index
    }
}

impl Solver for GreedySolver {
    fn solve_from_node(&mut self, graph: &GraphType, starting_node: IndexType) -> Option<(ValueType,PathType)>{
        if starting_node >= graph.len(){
            return None;
        }

        let problem_size = graph.len();
        let mut path: PathType = Vec::new();
        let mut unvisited: Vec<IndexType> = Vec::from_iter(0..problem_size);
        let mut current_node: IndexType = starting_node;

        path.push(starting_node);
        unvisited.remove(starting_node);

        while !unvisited.is_empty(){
            let next_node:IndexType = unvisited.remove(self.find_nearest_node(graph, current_node, &unvisited));
            path.push(next_node);
            current_node = next_node;           
        }

        path.push(starting_node);

        self.optimal_path = Some(path.clone());

        let cost:ValueType = calculate_cost(graph, &path);
        self.optimal_cost = Some(cost);

        Some((cost, path))
        
    }

    fn solve(&mut self, graph: &GraphType) -> Option<(ValueType,PathType)> {
        self.solve_from_node(graph, 0)
    }

    fn get_cost(&self) ->  Option<ValueType> {
        self.optimal_cost
    }

    fn get_path(&self) ->  Option<PathType> {
        self.optimal_path.clone()
    }
}

pub struct BruteForceSolver{
    optimal_cost: Option<ValueType>,
    optimal_path: Option<PathType>,
}

impl BruteForceSolver 
{
    pub fn new() -> Self{
        Self{
            optimal_path: None,
            optimal_cost: None,
        }
    }
}


impl Solver for BruteForceSolver {
    fn solve_from_node(&mut self, graph: &GraphType, starting_node: IndexType) -> Option<(ValueType,PathType)>{
        if starting_node >= graph.len(){
            return None;
        }

        let problem_size = graph.len();
        let mut path: PathType = Vec::new();
        let mut unvisited: Vec<IndexType> = Vec::from_iter(0..problem_size);


        unvisited.remove(starting_node);
        let mut min_cost: ValueType = ValueType::MAX;
        let mut heap = Heap::new(&mut unvisited);


        while let Some(p) = heap.next_permutation(){        
            let considered_path: PathType = [vec![starting_node], p.clone(), vec![starting_node]].concat();
            let considered_cost: ValueType = calculate_cost(graph, &considered_path);
            if considered_cost < min_cost{
                min_cost = considered_cost;
                path = considered_path;
            }
            
        }


        self.optimal_path = Some(path.clone());

        let cost:ValueType = calculate_cost(graph, &path);
        self.optimal_cost = Some(cost);

        Some((cost, path))
        
    }

    fn solve(&mut self, graph: &GraphType) -> Option<(ValueType,PathType)> {
        self.solve_from_node(graph, 0)
    }

    fn get_cost(&self) ->  Option<ValueType> {
        self.optimal_cost
    }

    fn get_path(&self) ->  Option<PathType> {
        self.optimal_path.clone()
    }
}

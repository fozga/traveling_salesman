use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};

use crate::types::{GraphType, IndexType, PathType, RowType, ValueType};

pub fn read_graph_fron_file(filename: &str) -> Result<GraphType, std::io::Error>{
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut problem_size = String::new();
    reader.read_line(&mut problem_size)?;
    let problem_size: IndexType = problem_size
    .trim()
    .parse::<IndexType>().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    let mut graph:GraphType = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut row:RowType = Vec::new();

        for number in line.split_whitespace() {
            row.push(number.parse::<ValueType>().map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
        }
        graph.push(row);
    }

    if graph.len() != problem_size{
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid graph size"));
    }

    Ok(graph)
}

pub fn display_graph(graph: &GraphType){
    for row in graph.iter(){
        for number in row.iter(){
            print!("{}\t", number);
        }
        println!();
    }
}

pub fn display_path(path: &PathType, separator: &str){
    for (i, node) in path.iter().enumerate() {
        print!("{}", node);
        if i < path.len() - 1 {
            print!("{}", separator);
        }
    }
    println!();
}
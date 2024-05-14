mod matrix_reader;
mod tsp;
use tsp::{tsp_dp};
use matrix_reader::read_matrix_from_file;

use std::io::{self};

fn main() -> io::Result<()> {
    println!("Enter the filename containing the matrix:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let filename = input.trim();

    let dist = read_matrix_from_file(filename)?;
    let n = dist.len();

    let (result, route) = tsp_dp(&dist, n);

    if result == i32::MAX {
        println!("No feasible path found due to infinite or invalid weights.");
    } else {
        println!("\nThe minimum cost is: {}", result);
        println!("Path taken (starting from city 1): {}", 
            route.iter().map(|&x| (x + 1).to_string()).collect::<Vec<_>>().join(" -> "));

        if route.len() > 1 {
            println!("\nDetailed route:");
            for i in 0..route.len() - 1 {
                let from = route[i];
                let to = route[i + 1];
                println!("Travel from city {} to city {}: {} units",
                         from + 1, to + 1, dist[from][to]);
            }
        }
    }

    Ok(())
}
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const INFINITY: i32 = i32::MAX;

pub fn read_matrix_from_file(filename: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut matrix = Vec::new();

    for line in reader.lines() {
        let line = line?.trim().to_string(); 
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|num| {
                if num == "∞" { 
                    INFINITY
                } else {
                    num.parse().unwrap_or_else(|_| {
                        eprintln!("Error parsing number: {}", num);
                        std::process::exit(1);
                    })
                }
            })
            .collect();
        matrix.push(numbers);
    }
    Ok(matrix)
}
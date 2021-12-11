use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()>{
    println!("Day 2: Dive Aim");

    let file = File::open("puzzle_input.txt")?;
    let reader = BufReader::new(file);

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in reader.lines() {

        let l = line?;
        let vec: Vec<&str> = l.split_whitespace().collect();

        let command = vec[0];
        let amount = vec[1].parse::<i32>().unwrap();

        match command {
            "forward" => {
                horizontal += amount;
                depth += aim * amount;
            },
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => println!("Unknown command {}", command)
        }
    }

    println!("Final position Horizontal: {}  Depth: {}", horizontal, depth);
    println!("Result: {}", horizontal * depth);
    Ok(())
}

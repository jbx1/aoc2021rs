use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()>{
    println!("Day 1: Sonar Sweep");

    let file = File::open("puzzle_input.txt")?;
    let reader = BufReader::new(file);

    let mut count = 0;
    let mut previous = 0;
    for line in reader.lines() {
        let next = line?.parse::<i32>().unwrap();
        if (next > previous) && (previous > 0) {
            count += 1;
        }
        previous = next;
    }

    println!("Total increases: {}", count);

    Ok(())
}

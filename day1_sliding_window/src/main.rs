use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()>{
    println!("Day 1: Sliding Window Sweep");

    let file = File::open("puzzle_input.txt")?;
    let reader = BufReader::new(file);

    let mut count = 0;
    let mut index = 0;
    let mut previous_sum = 0;
    let mut window : [i32; 3] = [0, 0, 0];
    for line in reader.lines() {
        let next = line?.parse::<i32>().unwrap();
        index += 1;

        window[0] = window[1];
        window[1] = window[2];
        window[2] = next;

        let sum = window[0] + window[1] + window[2];

        if (index > 3) && (sum > previous_sum) {
            count += 1;
        }
        previous_sum = sum;
    }

    println!("Total increases: {}", count);

    Ok(())
}

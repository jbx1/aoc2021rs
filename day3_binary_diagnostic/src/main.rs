use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    println!("Day 3: Binary diagnostic");

    let file = File::open("puzzle_input.txt")?;
    let reader = BufReader::new(file);

    let mut lines = 0;
    let mut ones: [i32; 12] = [0; 12];
    for line in reader.lines() {

        lines += 1;
        let str = line.unwrap();
        let chars = str.as_str().char_indices();
        for (i, c) in chars {
            if c == '1' {
                ones[i] += 1;
            }
        }
    }

    let half = lines / 2;
    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..ones.len() {
        if ones[i] > half {
            gamma |= 1 << (11 - i);
        } else {
            epsilon |= 1 << (11 - i);
        }
    }

    println!("Gamma {} Epsilon {}", gamma, epsilon);
    println!("Result: {}", gamma * epsilon);

    Ok(())
}

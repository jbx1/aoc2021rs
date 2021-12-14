use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn partition(input: &Vec<String>, index: i32) -> (Vec<String>, Vec<String>) {
    input.into_iter()
        .map(|s| s.clone())
        .partition(|s| s.chars().nth(index as usize).unwrap() == '1')
}

fn recursive_partition_higher(input: &Vec<String>, index: i32) -> String {
    let (ones, zeros) = partition(input, index);
    if ones.len() >= zeros.len() {
        //take ones
        if ones.len() == 1 {
            return ones.first().cloned().unwrap();
        } else {
            return recursive_partition_higher(&ones, index + 1);
        }
    } else {
        //take zero
        if zeros.len() == 1 {
            return zeros.first().cloned().unwrap();
        } else {
            return recursive_partition_higher(&zeros, index + 1);
        }
    }
}

fn recursive_partition_lower(input: &Vec<String>, index: i32) -> String {
    let (ones, zeros) = partition(input, index);
    if zeros.len() <= ones.len() {
        //take zeros
        if zeros.len() == 1 {
            return zeros.first().cloned().unwrap();
        } else {
            return recursive_partition_lower(&zeros, index + 1);
        }
    } else {
        //take ones
        if ones.len() == 1 {
            return ones.first().cloned().unwrap();
        } else {
            return recursive_partition_lower(&ones, index + 1);
        }
    }
}

fn read_file() -> Vec<String> {
    let file = File::open("puzzle_input.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .map(|l| l.expect("Unable to read line"))
        .collect()
}

fn binary_string_to_int(s: String) -> i32 {
    return isize::from_str_radix(s.as_str(), 2).unwrap() as i32;
}

fn main() -> io::Result<()> {
    println!("Day 3: Life support");

    let input = read_file();
    let oxygen = recursive_partition_higher(&input, 0);
    let co2 = recursive_partition_lower(&input, 0);

    println!("Oxygen {} co2 {}", oxygen, co2);

    let o2 = binary_string_to_int(oxygen);
    let co2_int = binary_string_to_int(co2);
    println!("Oxygen {} co2 {}", o2, co2_int);
    println!("Result {}", o2 * co2_int);

    Ok(())
}
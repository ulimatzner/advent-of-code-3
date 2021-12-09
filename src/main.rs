use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
fn main() {
    let path = Path::new("data/input.txt");
    println!("Solution: {}", run(path));
}

fn run(path: &Path) -> u32 {
    println!("oxygen:");
    let oxygen = find(path, find_most_used_value);
    println!("oxygen = {}", oxygen);

    println!("co2_scrubber:");
    let co2_scrubber = find(path, find_least_used_value);

    println!("co2_scrubber = {}", co2_scrubber);

    return u32::try_from(oxygen * co2_scrubber).unwrap();
}

fn find(path: &Path, filter: fn(u32, u32) -> char) -> u32 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut remaining_lines = reader.lines().map(
        | line | -> String {
            line.unwrap()
        }
    ).collect::<Vec<String>>();
    for i in 0..12 {
        let mut zeros = 0;
        let mut ones = 0;
        for line in &remaining_lines {
            let number = line.chars().nth(i).unwrap().to_digit(2).unwrap();
            match number {
                0 => zeros += 1,
                1 => ones += 1,
                _ => panic!("digit not 0 or 1: {}", number)
            }
        }
        let filter_for = filter(zeros, ones);
        remaining_lines.retain(|line| line.chars().nth(i).unwrap() == filter_for);
        println!("Iteration: {}. ones: {}, zeros: {}", i, ones, zeros);
        if remaining_lines.len() == 1 {
            return u32::from_str_radix(remaining_lines[0].as_str(), 2).unwrap();
        }
    }
    panic!("Iteraton return something other than 1 value. remaining_lines = \n {:?}", remaining_lines);
}

fn find_most_used_value(zeros: u32, ones: u32) -> char {
    if zeros > ones {
        return '0';
    }

    return '1';
}

fn find_least_used_value(zeros: u32, ones: u32) -> char {
    if ones < zeros {
        return '1';
    }

    return '0';
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let test_path = Path::new("tests/input.txt");

        let solution = run(test_path);
        assert_eq!(198, solution)
    }
}

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
fn main() {
    let path = Path::new("data/input.txt");
    println!("Solution: {}", run(path));
}

fn run(path: &Path) -> i32 {
    let file = File::open(path).unwrap();    
    let reader = BufReader::new(file);

    let mut all_lines = reader.lines();

    let mut bits = initialize(&mut all_lines);
    let mut count = 1;

    for line_res in all_lines {
        let line = line_res.unwrap();
        let mut char_count = 0;
        for char in line.chars() {
            let number = char.to_digit(10).unwrap();
            bits[char_count] += number;
            char_count += 1;
        }
        count += 1;
    }

    let mut gamma_as_string = "".to_string();
    for ones in bits {
        let zeros = count - ones;
        if zeros < ones {
            gamma_as_string.push_str("1");
            continue;
        }
        if zeros > ones {
            gamma_as_string.push_str("0");
            continue;
        }
        panic!("Can't have as many ones as zeros. Ones: {}, Zeros: {}",ones, zeros);
    } 
    
//    let gamma = gamma_as_string.parse::<u32>().unwrap();
    let gamma = u32::from_str_radix(gamma_as_string.as_str(), 2).unwrap();

    let base: u32 = 2;
    let max: u32 = base.checked_pow(u32::try_from(gamma_as_string.len()).unwrap()).unwrap() - 1;
    let epsilon = max - gamma;
    return i32::try_from(gamma * epsilon).unwrap();
}

fn initialize(all_lines: &mut std::io::Lines<BufReader<File>>) -> Vec<u32> {
    let first_line = all_lines.next().unwrap().unwrap();
    let mut bits: Vec<u32> = Vec::new();
    for char in first_line.chars() {
        let number = char.to_digit(10).unwrap();
        bits.push(number);
    }
    bits
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

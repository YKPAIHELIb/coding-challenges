use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();

    part_two(input);
}

fn part_one(input: String) {
    let mut last4 = [' '; 4];

    for (i, char) in input.chars().enumerate() {
        last4[i % 4] = char;
        if i > 3 && has_no_duplicates(&last4) {
            println!("The result is {}", i + 1);
            return;
        }
    }
}

fn part_two(input: String) {
    let mut last4 = [' '; 14];

    for (i, char) in input.chars().enumerate() {
        last4[i % 14] = char;
        if i > 13 && has_no_duplicates(&last4) {
            println!("The result is {}", i + 1);
            return;
        }
    }
}

fn has_no_duplicates(ascii_slice: &[char]) -> bool {
    let mut seen = [false; 128];

    for &c in ascii_slice {
        if c as usize >= 128 || seen[c as usize] {
            return false;
        }
        seen[c as usize] = true;
    }
    true
}

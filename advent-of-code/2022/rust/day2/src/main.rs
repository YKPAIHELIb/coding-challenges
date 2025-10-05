use std::io::{self, BufRead, Result};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    part_two(lines);
}

fn part_one(lines: io::Lines<io::StdinLock<'_>>) {
    let sum: i32 = lines
        .map(|line| match line.unwrap().trim() {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2,
            "C Z" => 3 + 3,
            _ => 0,
        })
        .sum();

    println!("{sum}");
}

fn part_two(lines: io::Lines<io::StdinLock<'_>>) {
    let sum: i32 = lines
        .map(|line| {
            let binding = line.unwrap();
            let parts = binding.trim().split(' ').collect::<Vec<&str>>();
            (parts[0].to_string(), parts[1].to_string())
        })
        .map(|(left, right)| match right.as_str() {
            "Y" => 3 + get_shape_score(left.as_str()),
            "X" => match left.as_str() {
                "A" => get_shape_score("C"),
                "B" => get_shape_score("A"),
                "C" => get_shape_score("B"),
                _ => panic!("invalid input"),
            },
            "Z" => {
                6 + match left.as_str() {
                    "A" => get_shape_score("B"),
                    "B" => get_shape_score("C"),
                    "C" => get_shape_score("A"),
                    _ => panic!("invalid input"),
                }
            }
            _ => panic!("invalid input"),
        })
        .sum();

    fn get_shape_score(shape: &str) -> i32 {
        match shape {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => panic!("invalid input"),
        }
    }
    println!("{sum}");
}

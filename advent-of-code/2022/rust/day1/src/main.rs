use std::io::{self, BufRead, Result};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let result = part_two(lines);
    println!("{result}");
}

fn part_one<I>(lines: I) -> i32
where
    I: Iterator<Item = Result<String>>,
{
    let mut max = 0;
    let mut current = 0;
    for line in lines {
        if let Ok(number) = line.unwrap().trim().parse::<i32>() {
            current += number;
            if current > max {
                max = current;
            }
        } else {
            current = 0;
        }
    }
    max
}

fn part_two<I>(lines: I) -> i32
where
    I: Iterator<Item = Result<String>>,
{
    let mut group_sums: Vec<_> = lines
        .map(|line| line.unwrap().trim().parse::<i32>().unwrap_or(0))
        .scan(0, |state, x| {
            *state += x;
            if x == 0 {
                let sum = *state;
                *state = 0;
                Some(Some(sum))
            } else {
                Some(None)
            }
        })
        .flatten()
        .collect();

    group_sums.sort_by(|a, b| b.cmp(a));

    let max_of_3: i32 = group_sums.into_iter().take(3).sum();

    max_of_3
}

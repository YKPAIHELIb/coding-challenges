use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    if let Err(_) = part_two(lines) {
        println!("Input has errors");
    }
}

fn part_one(lines: io::Lines<io::StdinLock<'_>>) -> Result<(), InputError> {
    let count = count_matching_pairs(
        lines,
        |first, second| first.contains(&second) || second.contains(&first))?;

    println!("The count of lines with full range containment: {count}");
    Ok(())
}

fn part_two(lines: io::Lines<io::StdinLock<'_>>) -> Result<(), InputError> {
    let count = count_matching_pairs(
        lines,
        |first, second| first.overlaps(second))?;

    println!("The count of lines with overlap of ranges: {count}");
    Ok(())
}

fn count_matching_pairs<F>(lines: io::Lines<io::StdinLock<'_>>, predicate: F) ->  Result<u32, InputError> 
where
    F: Fn(&Range, &Range) -> bool,
{
    lines
        .map(|line| {
            let line = line.unwrap();
            let (first, second) = parse_ranges(&line)?;

            let matches = predicate(&first, &second);
            Ok(if matches { 1 } else { 0 })
            
        })
        .try_fold(0, |acc, x| x.map(|number| acc + number))
}

fn parse_ranges(line: &str) -> Result<(Range, Range), InputError> {
    let mut parts = line.split(',');
    let (first, second) = match (parts.next(), parts.next(), parts.next()) {
        (Some(first), Some(second), None) => (first, second),
        _ => return Err(InputError)
    };

    Ok((Range::from_input(first)?, Range::from_input(second)?))
}

struct InputError;

struct Range {
    low: u32,
    high: u32,
}

impl Range {
    fn from_input(input: &str) -> Result<Self, InputError> {
        let mut parts = input.split('-');
        match (parts.next(), parts.next(), parts.next()) {
            (Some(left), Some(right), None) => match (left.parse::<u32>(), right.parse::<u32>()) {
                (Ok(low), Ok(high)) => Ok(Range { low, high }),
                _ => Err(InputError)
            },
            _ => Err(InputError)
        }
    }

    fn contains(&self, other: &Self) -> bool {
        self.low <= other.low && other.high <= self.high
    }

    fn overlaps(&self, other: &Self) -> bool {
        !(other.low > self.high || self.low > other.high)
    }
}



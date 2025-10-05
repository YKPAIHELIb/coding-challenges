use std::io::{self, BufRead};

#[derive(Debug)]
enum PriorityCalculationError {
    UnsupportedCharacter,
    NoDuplication,
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    part_two(lines);
}

fn part_one(lines: io::Lines<io::StdinLock<'_>>) {
    let sum: usize = lines
        .map(|line| {
            let line = line.unwrap();
            let parts: [&str; 2] = line.split_at(line.len() / 2).into();
            return get_duplication_priority(parts.into_iter())
                .unwrap_or_else(|err| {
                    println!("Error: {:?}", err);
                    panic!();
                });
        })
        .sum();

    println!("The sum of priorities is {sum}");
}

fn part_two(lines: io::Lines<io::StdinLock<'_>>) {
    let lines: Vec<_> = lines
        .map(|line| line.unwrap())
        .collect();

    let mut sum: usize = 0;
    for i in (0..lines.len()).step_by(3) {
        let end_i = std::cmp::min(i + 3, lines.len());
        let chunk = &lines[i..end_i];

        sum += get_duplication_priority(chunk.iter().map(|s| s.as_str()))
            .unwrap_or_else(|err| {
                println!("Error: {:?}", err);
                panic!();
            });
    }

    println!("The sum of priorities is {sum}");
}


fn get_duplication_priority<'a, I: Iterator<Item = &'a str>>(lines: I) -> Result<usize, PriorityCalculationError> {
    let mut existed_in_all = [false; 52];
    let mut existed_in_current = [false; 52];
    
    for (i, line) in lines.enumerate() {
        if i == 0 {
            for c in line.chars() {
                existed_in_all[get_priority(c)? as usize] = true;
                existed_in_current[get_priority(c)? as usize] = true;
            };
        } else {
            // clear existed_in_current 
            for flag in existed_in_current.iter_mut() {
                *flag = false;
            }

            // fill existed_in_current from line
            for c in line.chars() {
                existed_in_current[get_priority(c)? as usize] = true;
            };

            // merge existed_in_current into existed_in_all
            for i in 0..existed_in_all.len() {
                existed_in_all[i] &= existed_in_current[i];
            }
        }
    }

    existed_in_all.iter().position(|&x| x).map(|x| x + 1).ok_or(PriorityCalculationError::NoDuplication)
}

fn get_priority(c: char) -> Result<u8, PriorityCalculationError> {
    match c {
        'a'..='z' => Ok(c as u8 - b'a'),
        'A'..='Z' => Ok(c as u8 - b'A' + 26 as u8),
        _ => return Err(PriorityCalculationError::UnsupportedCharacter),
    }
}


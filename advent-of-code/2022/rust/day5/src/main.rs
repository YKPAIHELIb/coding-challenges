use std::io::{self, BufRead};
use regex::Regex;
use anyhow::{Result, anyhow};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    if let Err(err) = part_two(lines) {
        println!("Input has error: {:?}", err);
    }
}

fn part_one(lines: io::Lines<io::StdinLock<'_>>) -> Result<()> {
    let (cargo_input, stacks_count, commands) = parse_input(lines)?;
    let mut cargo = Cargo::from_input(&cargo_input, stacks_count);

    for command in commands {
        for _ in 0..command.amount {
            cargo.move_box(command.from_stack, command.to_stack);
        }
    }

    println!("The result is: {}", cargo.top_of_stacks());
    Ok(())
}

fn part_two(lines: io::Lines<io::StdinLock<'_>>) -> Result<()> {
    let (cargo_input, stacks_count, commands) = parse_input(lines)?;
    let mut cargo = Cargo::from_input(&cargo_input, stacks_count);

    for command in commands {
        cargo.move_boxes(command.from_stack, command.to_stack, command.amount);
    }

    println!("The result is: {}", cargo.top_of_stacks());
    Ok(())
}

fn parse_input(lines: io::Lines<io::StdinLock<'_>>) -> Result<(Vec<String>, usize, Vec<Command>)> {
    let mut cargo_input: Vec<String> = vec![];
    let mut stacks_count: usize = 0;
    let mut commands: Vec<Command> = vec![];

    for line in lines {
        let line = line.unwrap();
        if line.starts_with('[') {
            cargo_input.push(line);
        }
        else if line.starts_with(" 1") {
            let re = Regex::new(r"\b(\d+)\b").unwrap();
            let last_match = re.captures_iter(&line).last().ok_or_else(||anyhow!("No matches of number"))?;

            stacks_count = last_match
                .get(1).ok_or_else(||anyhow!("No number captured"))?
                .as_str()
                .parse()?;
        }
        else if line.starts_with("move") {
            let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
            let command_match = re.captures(&line).ok_or_else(||anyhow!("No regex match for move line"))?;

            let amount = command_match.get(1).ok_or_else(||anyhow!("No amount part catured"))?
                .as_str().parse()?;

            let from: i32 = command_match.get(2).ok_or_else(||anyhow!("No from_stack part catured"))?
                .as_str().parse()?;
            let from = from - 1;

            let to: i32 = command_match.get(3).ok_or_else(||anyhow!("No from_stack part catured"))?
                .as_str().parse()?;
            let to = to - 1;

            commands.push(Command { 
                from_stack: from as usize, 
                to_stack: to as usize, 
                amount 
            });
        }
    }
    Ok((cargo_input, stacks_count, commands))
}

#[derive(Debug)]
struct Cargo {
    stacks: Vec<Vec<ShipBox>>
}

impl Cargo {
    fn from_input(input: &Vec<String>, stacks_count: usize) -> Self {
        let mut cargo = Cargo{stacks: vec![vec![]; stacks_count]};

        for line in input.iter().rev().map(|l| l.as_str()) {
            for i in (1..line.len()).step_by(4) {
                let letter_or_space = line.as_bytes()[i] as char;
                if letter_or_space != ' ' {
                    let stack_index = i / 4;
                    cargo.stacks[stack_index].push(ShipBox(letter_or_space));
                }
            }
        }

        cargo
    }

    fn move_box(&mut self, from: usize, to: usize) {
        let ship_box = self.stacks[from].pop().ok_or_else(||anyhow!("No boxes left in stack to move")).unwrap();
        self.stacks[to].push(ship_box);
    }
    
    fn move_boxes(&mut self, from: usize, to: usize, amount: usize) {
        let from_stack = &mut self.stacks[from];
        let to_move = from_stack.split_off(from_stack.len() - amount);

        self.stacks[to].extend(to_move);
    }

    fn top_of_stacks(&self) -> String {
        let mut result = String::with_capacity(self.stacks.len());
        for stack in self.stacks.iter() {
            result.push(stack.last().ok_or_else(||anyhow!("Empty stack at the end")).unwrap().0);
        }
        result
    }
}

#[derive(Copy, Clone, Debug)]
struct ShipBox(char);

#[derive(Debug)]
struct Command {
    from_stack: usize,
    to_stack: usize,
    amount: usize,
}

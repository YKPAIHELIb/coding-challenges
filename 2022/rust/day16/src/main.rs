use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
    ops::{Add, Sub},
    rc::{Rc, Weak},
    str::FromStr,
};

fn main() -> Result<(), String> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.map_err(|e| e.to_string()));
    let diamonds = parse(lines)?;
    let result_1 = part_one(diamonds.clone());
    let result_2 = part_two(diamonds)
        .map(|r| r.to_string())
        .unwrap_or("No result".to_owned());
    println!("Result 1: {result_1}");
    println!("Result 2: {result_2}");
    Ok(())
}

fn part_one(diamonds: Vec<Diamond>) -> u32 {
    0
}

#[derive(Debug)]
struct Scene {
    valves: Vec<Rc<Valve>>,
    current: Rc<Valve>,
}

impl Scene {
    fn parse<I>(lines: I) -> Result<Self, String>
    where
        I: Iterator<Item = Result<String, String>>,
    {
        "Valve DX has flow rate=3; tunnels lead to valves BO, QL, BP, OF, QG";
        for line in lines {
            let (_, l) = line?.split_once("Valve ").ok_or("Invalid input")?;
            let (name, line) = l.split_once(" has flow rate=").ok_or("Invalid input")?;

        }

        todo!()
    }
}

#[derive(Debug)]
struct Valve {
    name: String,
    rate: u32,
    connected: Vec<Weak<Valve>>,
}

impl Valve {
    fn new(name: String, rate: u32) -> Self {
        Valve {
            name,
            rate,
            connected: vec![],
        }
    }
}

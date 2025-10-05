use std::str::FromStr;
use std::io::{self, BufRead};
use std::collections::hash_set::HashSet;
use anyhow::{Result, anyhow, Ok};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    part_two(lines).unwrap();
}

fn part_one(lines: io::Lines<io::StdinLock<'_>>) -> Result<()> {
    let rope = Rope::new(2)?;
    calculate_visible(lines, rope)
}

fn part_two(lines: io::Lines<io::StdinLock<'_>>) -> Result<()> {
    let rope = Rope::new(10)?;
    calculate_visible(lines, rope)
}

fn calculate_visible(lines: io::Lines<io::StdinLock<'_>>, mut rope: Rope) -> std::prelude::v1::Result<(), anyhow::Error> {
    let mut visited_by_tail = HashSet::new();
    
    for line in lines {
        let line = line?;
        let (d, n) = &line.split_once(' ').ok_or_else(|| anyhow!("Parsing error"))?;
        let d: Direction = d.parse()?;
        let n: u32 = n.parse()?;

        for _ in 0..n {
            rope.make_step(d);
            visited_by_tail.insert(rope.get_tail());
        }
    }

    println!("Visited by tail count: {}", visited_by_tail.len());

    Ok(())
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => std::result::Result::Ok(Direction::Up),
            "D" => std::result::Result::Ok(Direction::Down),
            "L" => std::result::Result::Ok(Direction::Left),
            "R" => std::result::Result::Ok(Direction::Right),
            _ => Err(anyhow!("Failed to parse Direction from {s}")),
        }
    }
   
}

struct Rope {
    head: Point,
    body: Vec<Point>,
}

impl Rope {
    fn new(length: usize) -> Result<Self> {
        if length < 2 {
            return Err(anyhow!("Length of rope must be at least 2 knots"));
        }
        Ok(Rope {
            head: Point { x: 0, y: 0 },
            body: vec![Point { x: 0, y: 0 }; length - 1]
        })
    }

    fn get_tail(&self) -> Point {
        *self.body.last().unwrap()
    }

    fn make_step(&mut self, d: Direction) {
        match d {
            Direction::Up => self.head.y += 1,
            Direction::Down =>  self.head.y -= 1,
            Direction::Left => self.head.x -= 1,
            Direction::Right => self.head.x += 1,
        }

        let mut heading_knot = self.head;
        for knot in &mut self.body {
            heading_knot = match try_move_body_knot(knot, heading_knot) {
                Some(h) => h,
                None => return,
            }
        }

        fn try_move_body_knot(knot_to_move: &mut Point, heading_knot: Point) -> Option<Point> {
            if (heading_knot.x - knot_to_move.x).abs() <= 1 && (heading_knot.y - knot_to_move.y).abs() <= 1 {
                return None;
            }

            if heading_knot.x != knot_to_move.x {
                knot_to_move.x += (heading_knot.x - knot_to_move.x).signum();
            }
            if heading_knot.y != knot_to_move.y {
                knot_to_move.y += (heading_knot.y - knot_to_move.y).signum();
            }

            Some(*knot_to_move)
        }
    }
}

use std::io::{self, BufRead};
use std::{iter, usize};
use std::collections::VecDeque;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    part_two(lines.map(|x|x.unwrap()));
}

fn part_one(lines: impl Iterator<Item = String>) {
    let map = parse_input(lines);

    let end_options = vec![map.end];
    let steps_count = breadh_first_search(map.start, &end_options, &map.height_map, get_next_steps);
    match steps_count {
        Some(steps_count) => println!("Steps count is {steps_count}"),
        None => println!("There's no way!"),
    }
    
    fn get_next_steps<'a>(current_point: Coordinate, height_map: &'a Vec<Vec<u8>>) -> Box<dyn Iterator<Item = Coordinate> + 'a> {
        let directions = [current_point.above(), current_point.below(), current_point.left(), current_point.right()];
        let mut i = 0;
        Box::new(iter::from_fn(move || {
            while i < directions.len() {
                let step = directions[i];
                i += 1;

                if step.x >= height_map[0].len() || step.y >= height_map.len() {
                    continue;
                }

                let current_height = height_map[current_point.y][current_point.x];
                let step_height = height_map[step.y][step.x];
                if current_height + 1 >= step_height {
                    return Some(step);
                }
            }

            None
        }))
    }
}

fn part_two(lines: impl Iterator<Item = String>) {
    let map = parse_input(lines);

    let start = map.end; // starting from top
    let end_options = map.height_map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row
            .iter()
            .enumerate()
            .filter_map(move |(x, &height)| {
                if height == 0 {
                    Some(Coordinate {x,y})
                } else {
                    None
                }
            }))
        .collect::<Vec<_>>();

    let steps_count = breadh_first_search(start, &end_options, &map.height_map, get_next_steps);
    match steps_count {
        Some(steps_count) => println!("Steps count is {steps_count}"),
        None => println!("There's no way!"),
    }
    
    fn get_next_steps<'a>(current_point: Coordinate, height_map: &'a Vec<Vec<u8>>) -> Box<dyn Iterator<Item = Coordinate> + 'a> {
        let directions = [current_point.above(), current_point.below(), current_point.left(), current_point.right()];
        let mut i = 0;
        Box::new(iter::from_fn(move || {
            while i < directions.len() {
                let step = directions[i];
                i += 1;

                if step.x >= height_map[0].len() || step.y >= height_map.len() {
                    continue;
                }

                let current_height = height_map[current_point.y][current_point.x];
                let step_height = height_map[step.y][step.x];
                if step_height + 1 >= current_height {
                    return Some(step);
                }
            }

            None
        }))
    }
}

fn parse_input(lines: impl Iterator<Item = String>) -> Map {
    let mut start = Coordinate { x: 0, y: 0 };
    let mut end = Coordinate { x: 0, y: 0 };
    let height_map = lines
        .enumerate()
        .map(|(y, l)| l
            .chars()
            .enumerate()
            .map(|(x, c)| match c {
                'S' => {
                    start.x = x;
                    start.y = y;
                    0
                },
                'E' => {
                    end.x = x;
                    end.y = y;
                    'z' as u8 - 'a' as u8
                },
                _ => c as u8 - 'a' as u8
            })
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Map { start, end, height_map }
}

fn breadh_first_search<'a, F>(start: Coordinate, end: &Vec<Coordinate>, height_map: &'a Vec<Vec<u8>>, get_next_steps: F) -> Option<u32> 
where
    F: Fn(Coordinate, &'a Vec<Vec<u8>>) -> Box<dyn Iterator<Item = Coordinate> + 'a>
{
    let mut visited = BitVector::new(height_map.len(), height_map[0].len());
    let mut queue = VecDeque::new();

    visited.set_visited(start);
    queue.push_back((start, 0));

    while let Some((coord, steps)) = queue.pop_front() {
        if end.contains(&coord) {
            return Some(steps);
        }

        for next in get_next_steps(coord, height_map) {
            if !visited.is_visited(next) {
                visited.set_visited(next);
                queue.push_back((next, steps + 1));
            }
        }
    }

    None
}

struct Map {
    start: Coordinate,
    end: Coordinate,
    height_map: Vec<Vec<u8>>
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize
}

impl Coordinate {
    fn above(self) -> Self {
        Coordinate { x: self.x, y: self.y.wrapping_sub(1) }
    }

    fn below(self) -> Self {
        Coordinate { x: self.x, y: self.y + 1 }
    }

    fn left(self) -> Self {
        Coordinate { x: self.x.wrapping_sub(1), y: self.y }
    }

    fn right(self) -> Self {
        Coordinate { x: self.x + 1, y: self.y }
    }
}

struct BitVector {
    data: Vec<u8>,
    cols: usize,
}

impl BitVector {
    fn new (rows: usize, cols: usize) -> Self {
        let size = (rows * cols + 7) / 8;
        Self {
            data: vec![0; size],
            cols,
        }
    }

    fn set_visited(&mut self, coordinate: Coordinate) {
        let index = coordinate.y * self.cols + coordinate.x;
        self.data[index / 8] |= 1 << (index % 8);
    }

    fn is_visited(&self, coordinate: Coordinate) -> bool {
        let index = coordinate.y * self.cols + coordinate.x;
        (self.data[index / 8] & (1 << (index % 8))) != 0
    }
}

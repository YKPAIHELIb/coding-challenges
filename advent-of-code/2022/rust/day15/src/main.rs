use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
    ops::{Add, Sub},
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

fn parse<I>(lines: I) -> Result<Vec<Diamond>, String>
where
    I: Iterator<Item = Result<String, String>>,
{
    lines.map(|l| l?.parse()).collect()
}

fn part_one(diamonds: Vec<Diamond>) -> u32 {
    const TARGET_Y: i32 = 2_000_000;
    let mut ranges: Vec<_> = diamonds
        .iter()
        .filter_map(|d| d.get_scanned_range_on_target_y(TARGET_Y))
        .sorted_by_key(|r| r.start)
        .collect();

    let mut i = 1;
    while i < ranges.len() {
        if let Some(merged) = ranges[i].merge(&ranges[i - 1]) {
            ranges[i - 1] = merged;
            ranges.remove(i);
        } else {
            i += 1;
        }
    }

    let beacons_at_target_y = diamonds
        .iter()
        .filter_map(|d| {
            if d.beacon.y == TARGET_Y {
                Some(d.beacon.x)
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();

    let mut no_beacons_count = 0;
    for range in ranges {
        let beacons_in_range_count = beacons_at_target_y
            .iter()
            .filter(|x| range.contains(**x))
            .count();
        no_beacons_count += range.length - beacons_in_range_count as u32;
    }

    no_beacons_count
}

fn part_two(diamonds: Vec<Diamond>) -> Option<u64> {
    let mut positive_lines_counts = HashMap::<LineXPlusY, u32>::new();
    let mut negative_lines_counts = HashMap::<LineXMinusY, u32>::new();
    for diamond in diamonds.iter() {
        let (p1, p2, n1, n2) = diamond.get_boundary_lines();
        *positive_lines_counts.entry(p1).or_insert(0) += 1;
        *positive_lines_counts.entry(p2).or_insert(0) += 1;
        *negative_lines_counts.entry(n1).or_insert(0) += 1;
        *negative_lines_counts.entry(n2).or_insert(0) += 1;
    }

    positive_lines_counts.retain(|_, c| *c >= 2);
    negative_lines_counts.retain(|_, c| *c >= 2);
    let candidates: Vec<_> = positive_lines_counts
        .iter()
        .flat_map(|(lp, _)| negative_lines_counts.iter().map(move |(ln, _)| (lp, ln)))
        .map(|(lp, ln)| lp.intersect(ln))
        .filter(|c| c.x >= 0 && c.x <= 4000000 && c.y >= 0 && c.y <= 4000000)
        .collect();

    println!("Number of candidates: {}", candidates.len());
    let found_beacon = candidates
        .iter()
        .find(|c| diamonds.iter().all(|d| !d.contains(**c)));

    found_beacon.map(|b| b.beacon_frequency())
}

#[derive(Clone, Copy)]
struct Range {
    start: i32,
    length: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct LineXPlusY {
    c: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct LineXMinusY {
    c: i32,
}

#[derive(Clone, Copy)]
struct Vector {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Diamond {
    sensor: Vector,
    beacon: Vector,
}

impl Range {
    fn new(start: i32, end: i32) -> Self {
        Range {
            start,
            length: (end - start + 1) as u32,
        }
    }

    fn intersects(&self, other: &Self) -> bool {
        self.start.max(other.start) <= self.end().min(other.end())
    }

    fn end(&self) -> i32 {
        self.start + self.length as i32 - 1
    }

    fn merge(&self, other: &Self) -> Option<Self> {
        if self.intersects(other) {
            Some(Self::new(
                self.start.min(other.start),
                self.end().max(other.end()),
            ))
        } else {
            None
        }
    }

    fn contains(&self, x: i32) -> bool {
        self.start <= x && x <= self.end()
    }
}

impl LineXPlusY {
    fn intersect(&self, other: &LineXMinusY) -> Vector {
        // Solve system:
        // x + y = c1
        // x - y = c2
        let x = (self.c + other.c) / 2;
        let y = (self.c - other.c) / 2;
        Vector { x, y }
    }
}

impl Vector {
    fn flip_h(&self) -> Self {
        Self {
            x: -self.x,
            y: self.y,
        }
    }

    fn flip_v(&self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
        }
    }

    fn positive(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    fn get_all_flipped(&self) -> [Self; 4] {
        let variants = [*self, self.flip_h(), self.flip_v(), self.flip_h().flip_v()];

        let mut sorted = variants;
        sorted.sort_by_key(|v| match (v.x >= 0, v.y >= 0) {
            (true, true) => 0,   // Q1
            (false, true) => 1,  // Q2
            (false, false) => 2, // Q3
            (true, false) => 3,  // Q4
        });

        sorted
    }

    fn x_plus_y(&self) -> LineXPlusY {
        LineXPlusY { c: self.x + self.y }
    }

    fn x_minus_y(&self) -> LineXMinusY {
        LineXMinusY { c: self.x - self.y }
    }

    fn manhattan(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }

    fn beacon_frequency(&self) -> u64 {
        self.x as u64 * 4000000 + self.y as u64
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl FromStr for Diamond {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s.split_once("x=").ok_or("Invalid input")?;
        let (sensor_x, s) = s.split_once(", y=").ok_or("Invalid input")?;
        let (sensor_y, s) = s
            .split_once(": closest beacon is at x=")
            .ok_or("Invalid input")?;
        let (beacon_x, beacon_y) = s.split_once(", y=").ok_or("Invalid input")?;

        let sensor = Vector {
            x: sensor_x.parse::<i32>().map_err(|e| e.to_string())?,
            y: sensor_y.parse::<i32>().map_err(|e| e.to_string())?,
        };
        let beacon = Vector {
            x: beacon_x.parse::<i32>().map_err(|e| e.to_string())?,
            y: beacon_y.parse::<i32>().map_err(|e| e.to_string())?,
        };

        Ok(Diamond {
            sensor: sensor,
            beacon: beacon,
        })
    }
}

impl Diamond {
    fn manhattan(&self) -> u32 {
        self.get_vector().manhattan()
    }

    fn get_vector(&self) -> Vector {
        self.beacon - self.sensor
    }

    fn contains(&self, vector: Vector) -> bool {
        (vector - self.sensor).manhattan() <= self.manhattan()
    }

    fn get_scanned_range_on_target_y(&self, target_y: i32) -> Option<Range> {
        let distance_to_target_y = self.sensor.y.abs_diff(target_y);
        let available_margin_x = self.manhattan().checked_sub(distance_to_target_y)?;

        Some(Range {
            start: self.sensor.x - available_margin_x as i32,
            length: available_margin_x * 2 + 1,
        })
    }

    fn get_boundary_lines(&self) -> (LineXPlusY, LineXPlusY, LineXMinusY, LineXMinusY) {
        let boundary_vector = (self.beacon - self.sensor).positive();
        let boundary_vector = Vector {
            x: boundary_vector.x + 1,
            y: boundary_vector.y,
        };
        let [q1, q2, q3, q4] = boundary_vector.get_all_flipped();
        (
            (self.sensor + q1).x_plus_y(),
            (self.sensor + q3).x_plus_y(),
            (self.sensor + q2).x_minus_y(),
            (self.sensor + q4).x_minus_y(),
        )
    }
}

use std::io::{self, BufRead};
use anyhow::{Result, anyhow, Ok};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let map = Map::parse_from_lines(lines.map(|x|x.unwrap()));
    part_two(map);
}

fn part_one(map: Map) {
    let visible_count = (0..map.0.len())
        .fold(0, |acc_row, row| acc_row + (0..map.0[0].len())
              .into_iter()
              .filter(|col| map.is_visible(row, *col).unwrap())
              .count());

    println!("Visible count is {visible_count}");
}

fn part_two(map: Map) {
    let scenic_score = (0..map.0.len())
        .flat_map(|i| (0..map.0[0].len()).map(move |j| (i, j)))
        .map(|(i, j)| map.scenic_score(i, j).unwrap())
        .max().unwrap();

    println!("Scenic score is {scenic_score}");
}

struct Map(Vec<Vec<u8>>);

impl Map {
    fn parse_from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut rows = vec![];
        for line in lines {
            let mut row = vec![];
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap() as u8)
            }
            rows.push(row);
        }

        Self(rows)
    }

    fn is_visible(&self, row: usize, col: usize) -> Result<bool> {
        if row >= self.0.len() || col >= self.0[0].len() {
            return Err(anyhow!("Index out of range"));
        }
        
        if row == 0 || row == self.0.len() - 1 || col == 0 || col == self.0[0].len() - 1 {
            return Ok(true);
        }

        let current_height = self.0[row][col];

        // from top
        let mut visible = self.0[row][..col]
            .into_iter()
            .all(|x| x < &current_height);

        // from bottom
        visible |= self.0[row][(col + 1)..]
            .into_iter()
            .all(|x| x < &current_height);

        // from left
        visible |= self.0[..row]
            .into_iter()
            .map(|x| x[col])
            .all(|x| x < current_height);

        // from bottom
        visible |= self.0[(row + 1)..]
            .into_iter()
            .map(|x| x[col])
            .all(|x| x < current_height);
            
        Ok(visible)
    }

    fn scenic_score(&self, row: usize, col: usize) -> Result<u32> {
        if row >= self.0.len() || col >= self.0[0].len() {
            return Err(anyhow!("Index out of range"));
        }
        
        if row == 0 || row == self.0.len() - 1 || col == 0 || col == self.0[0].len() - 1 {
            return Ok(0);
        }

        let current_height = self.0[row][col];

        let trees_left = (0..col).rev().map(|j| self.0[row][j]);
        let trees_right = ((col + 1)..self.0[0].len()).map(|j| self.0[row][j]);
        let trees_top = (0..row).rev().map(|i| self.0[i][col]);
        let trees_bottom = ((row + 1)..self.0.len()).map(|i| self.0[i][col]);

        let scenic_score = 
            visible_count(trees_left, current_height) * 
            visible_count(trees_right, current_height) * 
            visible_count(trees_top, current_height) * 
            visible_count(trees_bottom, current_height);

        return Ok(scenic_score);

        fn visible_count(sequence: impl Iterator<Item=u8>, height: u8) -> u32 {
            let mut count: u32 = 0;
            for i in sequence {
                count += 1;
                if i >= height {
                    break;
                }
            }

            count
        }
    }
}

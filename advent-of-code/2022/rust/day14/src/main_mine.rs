use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());

    let scene = Scene::new(lines);
    part_two(scene);
}

fn part_one(mut scene: Scene) {
    let mut rest_count = 0;
    while let tick_result = scene.tick()
        && tick_result != TickResult::Infinite
    {
        if tick_result == TickResult::Rest {
            rest_count += 1;
        }
    }

    println!("Result is {rest_count}");
}

fn part_two(mut scene: Scene) {
    let y_max = scene
        .map
        .iter()
        .enumerate()
        .flat_map(|(x, col)| col.iter().enumerate().map(move |(y, m)| (x, y, m)))
        .filter(|tuple| *tuple.2 == Material::Rock)
        .map(|tuple| tuple.1)
        .max()
        .unwrap();

    let floor = y_max + 2;
    for x in 0..Scene::SIZE_X {
        scene.map[x][floor] = Material::Rock;
    }

    let mut rest_count = 0;
    while let tick_result = scene.tick()
        && tick_result != TickResult::Blocked
    {
        if tick_result == TickResult::Rest {
            rest_count += 1;
        }
    }

    println!("Result is {rest_count}");
}

struct Scene {
    map: Vec<Vec<Material>>,
    falling_sand_position: Coord,
}

type Coord = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Material {
    Air,
    Rock,
    Sand,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TickResult {
    ContinueFalling,
    Rest,
    Blocked,
    Infinite,
}

impl Scene {
    const SIZE_X: usize = 700;
    const SIZE_Y: usize = 300;

    fn new(lines: impl Iterator<Item = String>) -> Self {
        let mut map = vec![vec![Material::Air; Scene::SIZE_Y]; Scene::SIZE_X];

        for line in lines {
            let coords = line.split(" -> ").map(|p| {
                p.split_once(",")
                    .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                    .unwrap()
            });

            let mut prev_coord: Option<Coord> = None;

            for coord in coords {
                if let Some((x1, y1)) = prev_coord {
                    let (x2, y2) = coord;

                    if x1 == x2 {
                        for y in y1.min(y2)..=y1.max(y2) {
                            map[x1][y] = Material::Rock;
                        }
                    } else if y1 == y2 {
                        for x in x1.min(x2)..=x1.max(x2) {
                            map[x][y1] = Material::Rock;
                        }
                    }
                }
                prev_coord = Some(coord);
            }
        }

        Self {
            map: map,
            falling_sand_position: (500, 0),
        }
    }

    fn tick(&mut self) -> TickResult {
        let (x, y) = self.falling_sand_position;
        if y > 290 {
            return TickResult::Infinite;
        }
        if self.map[x][y + 1] == Material::Air {
            self.falling_sand_position = (x, y + 1);
            return TickResult::ContinueFalling;
        }
        if self.map[x - 1][y + 1] == Material::Air {
            self.falling_sand_position = (x - 1, y + 1);
            return TickResult::ContinueFalling;
        }
        if self.map[x + 1][y + 1] == Material::Air {
            self.falling_sand_position = (x + 1, y + 1);
            return TickResult::ContinueFalling;
        }
        if self.map[x][y + 1] != Material::Air {
            self.falling_sand_position = (500, 0);
            if x == 500 && y == 0 && self.map[x][y] == Material::Sand {
                return TickResult::Blocked;
            }
            self.map[x][y] = Material::Sand;
            return TickResult::Rest;
        }
        panic!("Not expected")
    }
}

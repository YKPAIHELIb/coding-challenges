use std::io::{self, BufRead};

fn main() -> Result<(), String> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.map_err(|e| e.to_string()));
    let scene = Scene::from_lines(lines)?;

    println!("Part 1: {}", simulate(scene.clone(), false));
    println!("Part 2: {}", simulate(scene, true));

    Ok(())
}

#[derive(Clone)]
struct Scene {
    map: Vec<Vec<Material>>,
    sand_pos: Coord,
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
    const SOURCE: Coord = (500, 0);

    fn from_lines<I>(lines: I) -> Result<Self, String>
    where
        I: Iterator<Item = Result<String, String>>,
    {
        let mut map = vec![vec![Material::Air; Self::SIZE_Y]; Self::SIZE_X];

        for line in lines {
            let line = line?;
            let coords = line
                .split(" -> ")
                .map(|s| {
                    let (x, y) = s
                        .split_once(',')
                        .ok_or("Invalid coordinate format")?;
                    let x = x.parse::<usize>().map_err(|_| "Parse error")?;
                    let y = y.parse::<usize>().map_err(|_| "Parse error")?;
                    Ok((x, y))
                })
                .collect::<Result<Vec<_>, _>>()?;

            for pair in coords.windows(2) {
                let ((x1, y1), (x2, y2)) = (pair[0], pair[1]);
                if x1 == x2 {
                    for y in y1.min(y2)..=y1.max(y2) {
                        map[x1][y] = Material::Rock;
                    }
                } else if y1 == y2 {
                    for x in x1.min(x2)..=x1.max(x2) {
                        map[x][y1] = Material::Rock;
                    }
                } else {
                    return Err("Non-orthogonal path".to_string());
                }
            }
        }

        Ok(Self {
            map,
            sand_pos: Self::SOURCE,
        })
    }

    fn add_floor(&mut self) {
        let y_max = self
            .map
            .iter()
            .flat_map(|col| col.iter().enumerate())
            .filter(|(_, m)| **m == Material::Rock)
            .map(|(y, _)| y)
            .max()
            .unwrap_or(0);
        let floor_y = y_max + 2;
        if floor_y < Self::SIZE_Y {
            for x in 0..Self::SIZE_X {
                self.map[x][floor_y] = Material::Rock;
            }
        }
    }

    fn tick(&mut self) -> TickResult {
        let (x, y) = self.sand_pos;
        if y + 1 >= Self::SIZE_Y {
            return TickResult::Infinite;
        }

        for dx in [0_i32, -1, 1] {
            let nx = x.wrapping_add_signed(dx);
            let ny = y + 1;
            if nx < Self::SIZE_X && self.map[nx][ny] == Material::Air {
                self.sand_pos = (nx, ny);
                return TickResult::ContinueFalling;
            }
        }

        if self.map[x][y] == Material::Sand {
            return TickResult::Blocked;
        }

        self.map[x][y] = Material::Sand;
        self.sand_pos = Self::SOURCE;
        TickResult::Rest
    }
}

fn simulate(mut scene: Scene, with_floor: bool) -> usize {
    if with_floor {
        scene.add_floor();
    }

    let mut count = 0;
    loop {
        match scene.tick() {
            TickResult::Rest => count += 1,
            TickResult::ContinueFalling => {}
            TickResult::Infinite => break,
            TickResult::Blocked => {
                count += 1;
                break;
            }
        }
    }

    count
}

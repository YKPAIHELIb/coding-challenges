use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let program_routine = ProgramRoutine::from_commands(lines.map(|x|x.unwrap()));
    part_two(program_routine);
}

fn part_one(program_routine: ProgramRoutine<impl Iterator<Item = String>>) {
    let mut sum = 0;
    for (i, value) in program_routine.enumerate() {
        let i = i as i32 + 1;
        if i == 20 || i == 60 || i == 100 || i == 140 || i == 180 || i == 220 {
            sum += i * value;
        }
    }

    println!("Signal strength sum is: {sum}");
}

fn part_two(program_routine: ProgramRoutine<impl Iterator<Item = String>>) {
    let mut drawing = String::with_capacity(240);
    for (i, value) in program_routine.enumerate() {
        let i = i as i32 % 40;
        let to_draw = if i < value - 1 || i > value + 1 { '.' } else { '#' };
        drawing.push(to_draw);
    }

    println!("CRT drawing:");
    for i in 0..6 {
        println!("{}", &drawing[i*40..i*40+40]);
    }
}

struct ProgramRoutine<T> {
    x_value: i32,
    next_x_value: i32,
    skip_next: bool,
    inner: T,
}

impl<T> ProgramRoutine<T> 
where T: Iterator<Item = String> {
    fn from_commands(commands: T) -> Self {
        ProgramRoutine {
            x_value: 1,
            next_x_value: 1,
            skip_next: false,
            inner: commands,
        }
    }
}

impl<T> Iterator for ProgramRoutine<T>
where T: Iterator<Item = String> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.skip_next {
            self.skip_next = false;
            return Some(self.x_value);
        }

        self.x_value = self.next_x_value;
        let command = self.inner.next()?;
        if command.starts_with("addx") {
            let (_, val) = command.split_at(5);
            self.next_x_value = self.x_value + val.parse::<i32>().unwrap();
            self.skip_next = true;
        }

        Some(self.x_value)
    }
}



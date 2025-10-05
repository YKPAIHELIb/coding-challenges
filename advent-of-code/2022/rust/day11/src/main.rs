use std::fmt;
use std::io::{self, Read};
use anyhow::{Result, Ok};
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();

    let scene = Scene::parse(&input).unwrap();

    part_one(scene)
}

fn part_one(mut scene: Scene) {
    for _ in 0..20 {
        scene.run_round_with_releif();
    }

    let most_active_monkeys = scene.get_most_active_monkeys();
    let level_of_monkey_business = most_active_monkeys.into_iter()
        .take(2)
        .map(|x| x.inspected_items_count)
        .fold(1, |a,b| a * b);

    println!("The level of monkey business is: {level_of_monkey_business}");
}

fn part_two(mut scene: Scene) {
    for _ in 0..10000 {
        scene.run_round_no_releif();
    }

    let most_active_monkeys = scene.get_most_active_monkeys();
    let level_of_monkey_business = most_active_monkeys.into_iter()
        .take(2)
        .map(|x| x.inspected_items_count)
        .fold(1, |a,b| a as u64 * b as u64);

    println!("The level of monkey business is: {level_of_monkey_business}");
}

#[derive(Clone, Copy)]
struct ItemWorryLevel(u64);

struct Monkey {
    items: VecDeque<ItemWorryLevel>,
    operation: Box<dyn Fn(u64) -> u64>,
    test_divisibility_to: u64,
    throw_to_if_divisible: usize,
    throw_to_if_not_divisible: usize,
    inspected_items_count: u32,
}

struct Scene {
    monkeys: Vec<Monkey>,
    overall_modulo: u64,
}

impl Scene {
    fn parse(s: &str) -> Result<Scene> {
        lazy_static! {
            static ref MONKEY_RE: Regex = Regex::new(r"Monkey\s\d+:\s+Starting\sitems:\s([\d, ]+)\s+Operation:\snew\s=(.+)\s*Test:\sdivisible\sby\s(\d+)\s+If\strue:\sthrow\sto\smonkey\s(\d+)\s+If\sfalse:\sthrow\sto\smonkey\s(\d+)").unwrap();
            static ref OPERATIONS_RE: Regex = Regex::new(r"(\w+)\s([+\-\*/])\s(\w+)").unwrap();
        }

        let mut monkeys = vec![];
        for cap in MONKEY_RE.captures_iter(s) {
            monkeys.push(Monkey {
                items: cap[1]
                    .split(',')
                    .map(|i|i.trim().parse::<u64>().unwrap())
                    .map(|x|ItemWorryLevel(x))
                    .collect(),
                operation: parse_operation(&cap[2]),
                test_divisibility_to: cap[3].parse()?,
                throw_to_if_divisible: cap[4].parse()?,
                throw_to_if_not_divisible: cap[5].parse()?, 
                inspected_items_count: 0,
            });
        }

        let overall_modulo = monkeys.iter().map(|monkey| monkey.test_divisibility_to).fold(1, |a, b| a * b);

        return Ok(Scene { monkeys, overall_modulo, });

        fn parse_operation(operation: &str) -> Box<dyn Fn(u64) -> u64> {
            let parts: Vec<&str> = operation.split_whitespace().collect();
            match parts.as_slice() {
                ["old", "*", "old"] => {
                    Box::new(move |x| x * x)
                }
                ["old", "*", factor] => {
                    let factor = factor.parse::<u64>().unwrap();
                    Box::new(move |x| x * factor)
                }
                ["old", "+", addend] => {
                    let addend = addend.parse::<u64>().unwrap();
                    Box::new(move |x| x + addend)
                }
                _ => panic!("Unknown operation"),
            }
        }
    }

    fn run_round_with_releif(&mut self) {
        for i in 0..self.monkeys.len() {
            while let Some(mut item) = self.monkeys[i].items.pop_front() {
                let monkey = &mut self.monkeys[i];
                monkey.inspected_items_count += 1;

                item.0 = (monkey.operation)(item.0);
                item.0 = (item.0 as f64 / 3.0).floor() as u64;
                let throw_to_monkey = if item.0 % monkey.test_divisibility_to == 0 { monkey.throw_to_if_divisible } 
                                      else { monkey.throw_to_if_not_divisible };

                self.monkeys[throw_to_monkey].items.push_back(item);
            }
        }
    }

    fn run_round_no_releif(&mut self) {
        for i in 0..self.monkeys.len() {
            while let Some(mut item) = self.monkeys[i].items.pop_front() {
                let monkey = &mut self.monkeys[i];
                monkey.inspected_items_count += 1;

                item.0 = (monkey.operation)(item.0) % self.overall_modulo;
                let throw_to_monkey = if item.0 % monkey.test_divisibility_to == 0 { monkey.throw_to_if_divisible } 
                                      else { monkey.throw_to_if_not_divisible };

                self.monkeys[throw_to_monkey].items.push_back(item);
            }
        }
    }

    fn get_most_active_monkeys(&self) -> Vec<&Monkey> {
        let mut monkeys_refs = self.monkeys.iter().collect::<Vec<_>>();
        monkeys_refs.sort_unstable_by(|a, b| b.inspected_items_count.cmp(&a.inspected_items_count));
        monkeys_refs
    }
}

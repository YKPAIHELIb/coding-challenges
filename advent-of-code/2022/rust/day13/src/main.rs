use std::{cmp::Ordering, io::{self, BufRead}};
use itertools::Itertools;
use std::str;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let packages = parse_packages(lines.map(|x|x.unwrap()));
    part_two(packages);
}

fn parse_packages(lines: impl Iterator<Item = String>) -> Vec<Package> {
    lines
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            let left = chunk.next().unwrap();
            let right = chunk.next().unwrap();
            Package {
                left: PacketPiece::parse_list(&left.as_bytes()[1..(left.len()-1)]),
                right: PacketPiece::parse_list(&right.as_bytes()[1..(right.len()-1)]),
            }
        })
        .collect()
}

fn part_one(packages: Vec<Package>) {
    let result: usize = packages
        .into_iter()
        .enumerate()
        .map(|(package_position, package)|
            if package.is_right_order() { package_position + 1 } else { 0 })
        .sum();

    println!("Result is {result}");
}

fn part_two(packages: Vec<Package>) {
    let mut sorted_packages = packages
        .into_iter()
        .flat_map(|package| [package.left, package.right])
        .sorted_by(|a,b| get_order(a, b))
        .collect::<Vec<_>>();
    
    let first_divider_packet = vec![PacketPiece::List(vec![PacketPiece::Number(2)])];
    let (first_divider_packet_index, _) = sorted_packages.iter().find_position(|package| get_order(package, &first_divider_packet) == Ordering::Greater).unwrap();

    sorted_packages.insert(first_divider_packet_index, first_divider_packet);

    let second_divider_packet = vec![PacketPiece::List(vec![PacketPiece::Number(6)])];
    let (second_divider_packet_index, _) = sorted_packages.iter().find_position(|package| get_order(package, &second_divider_packet) == Ordering::Greater).unwrap();

    let result = (first_divider_packet_index + 1) * (second_divider_packet_index + 1);
    println!("Result is {result}");
}


fn get_order(left: &[PacketPiece], right: &[PacketPiece]) -> Ordering {
    let mut left_iter = left.into_iter();
    let mut right_iter = right.iter();
    
    loop {
        let left_piece = left_iter.next();
        let right_piece = right_iter.next();

        if let (Some(l), Some(r)) = (left_piece, right_piece) {
            if let (PacketPiece::Number(l_number), PacketPiece::Number(r_number)) = (l, r) {
                if l_number < r_number { return Ordering::Less } 
                else if l_number > r_number { return Ordering::Greater } 
                else { continue; }
            }

            let mut left_array = [PacketPiece::Number(0)];
            let left_list: &[PacketPiece] = match l {
                PacketPiece::Number(number) => {
                    left_array[0] = PacketPiece::Number(*number);
                    &left_array
                },
                PacketPiece::List(vector) => vector,
            };

            let mut right_array = [PacketPiece::Number(0)];
            let right_list: &[PacketPiece] = match r {
                PacketPiece::Number(number) => {
                    right_array[0] = PacketPiece::Number(*number);
                    &right_array
                },
                PacketPiece::List(vector) => vector,
            };

            let recursion_result = get_order(left_list, right_list);
            match recursion_result {
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
                Ordering::Equal => continue,
            }
        }

        if left_piece.is_none() && right_piece.is_some() {
            return Ordering::Less;
        }
        if left_piece.is_some() && right_piece.is_none() {
            return Ordering::Greater;
        }
        if left_piece.is_none() && right_piece.is_none() {
            return Ordering::Equal;
        }
    }
}

struct Package {
    left: Vec<PacketPiece>,
    right: Vec<PacketPiece>
}

impl Package {
    fn is_right_order(&self) -> bool {
        return get_order(&self.left, &self.right) == Ordering::Less;
    }
}

enum PacketPiece {
    Number(u8),
    List(Vec<PacketPiece>)
}

impl PacketPiece {
    fn parse_list(input: &[u8]) -> Vec<PacketPiece> {
        let mut result = vec![];
        let mut input = input;
        while input.len() > 0 {
            if (input[0] as char).is_digit(10) {
                let comma_index = find_comma(input).unwrap_or(input.len());
                let number_piece = str::from_utf8(&input[0..comma_index]).expect("Invalid UTF8 str").parse().expect("Can't parse");
                result.push(Self::Number(number_piece));
                if comma_index + 1 < input.len() {
                    input = &input[comma_index + 1..];
                } else {
                    input = &[];
                }
            } else if (input[0] as char) == '[' {
                let closing_bracket_index = find_closing_bracket(input, 0).unwrap();
                let parsed_list = PacketPiece::parse_list(&input[1..closing_bracket_index]);
                result.push(PacketPiece::List(parsed_list));
                if closing_bracket_index + 2 < input.len() {
                    input = &input[closing_bracket_index + 2..];
                } else {
                    input = &[];
                }
            }
        }
        return result;

        fn find_comma(input_bytes: &[u8]) -> Option<usize> {
            input_bytes.iter()
                .find_position(|&&c| (c as char) == ',')
                .map(|x| x.0)
        }

        fn find_closing_bracket(input: &[u8], opening_bracket_index: usize) -> Option<usize> {
            let mut depth = 0;
            for (i, &c) in input.iter().enumerate().skip(opening_bracket_index) {
                match c as char {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    _ => ()
                }
                if depth == 0 {
                    return Some(i);
                }
            }
            None
        }
    }
}

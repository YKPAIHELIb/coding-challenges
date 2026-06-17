use std::{usize, vec};
use std::collections::HashMap;

/// A multi-map that preserves insertion order.
///
/// Entries are stored sequentially in a `Vec`, while a `HashMap` maintains
/// indices into that vector for fast key-based access.
#[derive(Default)]
pub struct InsertionOrderedMultiMap<K, V> {
    map: HashMap<K, Vec<usize>>,
    order: Vec<(K, V)>,
}

impl<K, V> InsertionOrderedMultiMap<K, V>
where
    K: Eq + std::hash::Hash + Copy,
    V: PartialEq + Copy,
{
    fn push(&mut self, key: K, value: V) {
        self.order.push((key, value));
        self.map.entry(key)
            .or_default()
            .push(self.order.len() - 1);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, (K, V)> {
        self.order.iter()
    }
    
    pub fn get_by_key(&self, key: K) -> Option<impl Iterator<Item = V>> {
        self.map.get(&key)
            .map(|vec| vec.iter().map(|&i| self.order[i].1))
    }

    pub fn contains_key(&self, key: K) -> bool {
        self.map.contains_key(&key)
    }
}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut closed_islands = 0;
        let mut next_index = 0;
        let mut get_next = || {
            let id = next_index;
            next_index += 1;
            id
        };

        let mut prev_row_slices = InsertionOrderedMultiMap::<i32, (usize , usize)>::default();
        let mut curr_row_slices = InsertionOrderedMultiMap::<i32, (usize , usize)>::default();
        for row in &grid {
            let mut prev_iter = prev_row_slices.iter();
            let mut row_slices = Self::parse_slices(row);
            let mut prev_slice_w_i = prev_iter.next();
            let mut curr_slice = row_slices.next();
            let mut curr_slice_i = None;
            
            loop {
                match (prev_slice_w_i, curr_slice, curr_slice_i) {
                    (None, None, _) => break,
                    (None, Some(c), None) => {
                        let i = get_next();
                        curr_slice_i = Some(i);
                        curr_row_slices.push(i, c);
                        // test this should break cause it mutates while prev_iter keeps using immutable ref
                        prev_row_slices.push(i, c);
                    },
                    (None, Some(_), Some(_)) => {}, // skip since it's already pushed
                    (Some(&(id, p)), None, _) => {
                        if !curr_row_slices.contains_key(id) {
                            
                        }
                    },
                    // curr_slice is before prev_slice completely
                    (Some((id, p)), Some(c)) if c.1 <= p.0 => {
                        curr_row_slices.push((c, get_next()));
                        curr_slice = row_slices.next();
                    },

                }

                // move next
                match (prev_slice_w_i, curr_slice) {
                    (None, Some(_)) => {
                        curr_slice = row_slices.next();
                        curr_slice_i = None;
                    },
                    (Some(_), None) => prev_slice_w_i = prev_iter.next(),
                    (Some((_, p)), Some(c)) => {
                        if p.1 <= c.1 {
                            prev_slice_w_i = prev_iter.next();
                        }
                        if p.1 >= c.1 {
                            curr_slice = row_slices.next();
                            curr_slice_i = None;
                        }
                    },
                    _ => {},
                }
                
            }
        }
        
        
        0
    }

    fn parse_slices(row: &[char]) -> impl Iterator<Item = (usize, usize)> {
        let mut i = 0;
        let mut start: Option<usize> = None;
        std::iter::from_fn(move || {
            while i < row.len() {
                let is_island = row[i] == '1';
                let current_i = i;
                i += 1;

                match (is_island, start) {
                    (false, Some(s)) => {
                        start = None;
                        return Some((s, current_i));
                    }
                    (true, None) => {
                        start = Some(current_i);
                    }
                    _ => {}
                }
            }
            if let Some(s) = start {
                start = None;
                Some((s, i))
            } else {
                None
            }
        })
    }
}

#[derive(Debug)]
struct Solution {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {

        let input = r#"01011110101001011000
11111010111010010011
01001100000001101001
10011101111001001000
01111000100000110111
11111110101010101100
11101001101010011100
10111101001001100011
10000011110110001111
00010111110000011110
10101111101011101000
01010011001111010011
00111110111100000001
01001000110010001110
10111000000110000111
00010000110011001110
10011100000001100011
00101010000101001011
11011000110101010000
00111110011001110101"#;

        let input_grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

        println!("{:?}", input_grid);
        for row in input_grid {
            let slices: Vec<_> = Solution::parse_slices(&row).collect();
            println!("{:?}", slices);
        }
    }
}

use std::{usize, vec};
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

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
    
    pub fn get(&self, key: usize) -> Option<&(K, V)> {
        self.order.get(key)
    }
    
    pub fn get_mut(&mut self, key: usize) -> Option<&mut (K, V)> {
        self.order.get_mut(key)
    }
    
    pub fn get_by_key(&self, key: K) -> Option<impl Iterator<Item = V>> {
        self.map.get(&key)
            .map(|vec| vec.iter().map(|&i| self.order[i].1))
    }

    pub fn contains_key(&self, key: K) -> bool {
        self.map.contains_key(&key)
    }

    pub fn get_keys(&self) -> impl Iterator<Item = &K> {
        self.map.keys()
    }

    pub fn merge_to_key(&mut self, key: K, new_key: K) {
        let to_move = self.map.remove(&key).expect("This function must be called for existing key");
        let new_entry = self.map.entry(key).or_default();
        for i in to_move {
            new_entry.push(i);
            self.order[i].0 = new_key;
        }
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.order.clear();
    }
}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut closed_islands = 0;
        let mut next_index = 0;
        let mut get_next_index = || {
            let id = next_index;
            next_index += 1;
            id
        };

        let mut prev_row_slices = InsertionOrderedMultiMap::<i32, (usize , usize)>::default();
        let mut curr_row_slices = InsertionOrderedMultiMap::<i32, (usize , usize)>::default();
        for row in &grid {
            let mut prev_i = 0;
            let mut row_slices = Self::parse_slices(row);
            let mut curr_slice = row_slices.next();
            let mut curr_slice_index = None;
            
            loop {
                match (prev_row_slices.get(prev_i), curr_slice, curr_slice_index) {
                    (None, None, _) => break,
                    (None, Some(c), None) => {
                        let i = get_next_index();
                        curr_slice_index = Some(i);
                        curr_row_slices.push(i, c);
                    },
                    (None, Some(_), Some(_)) => {}, // skip since it's already pushed

                    (Some(&(id, p)), None, _) 
                        if curr_row_slices.contains_key(id) => {}, // already tracked by current row
                    (Some(&(id, p)), None, _) => {
                        if prev_row_slices.get_by_key(id)
                            .expect("It must exist.")
                            .all(|s|s.1 <= p.0) {
                                closed_islands += 1;
                        }
                    },

                    // curr_slice is before prev_slice completely
                    (Some(&(id, p)), Some(c), None) if c.1 <= p.0 => {
                        let i = get_next_index();
                        curr_slice_index = Some(i);
                        curr_row_slices.push(i, c);
                    },
                    (Some(&(id, p)), Some(c), Some(_)) if c.1 <= p.0 => {}, // skip since it's already pushed

                    // prev_slice is before curr_slice completely
                    (Some(&(id, p)), Some(c), _) 
                        if p.1 <= c.0 && curr_row_slices.contains_key(id) => {}, // already tracked by current row
                    (Some(&(id, p)), Some(c), _) if p.1 <= c.0 => {
                        if prev_row_slices.get_by_key(id)
                            .expect("It must exist.")
                            .all(|s|s.1 <= p.0) {
                                closed_islands += 1;
                        }
                    },

                    // intersections
                    (Some(&(id, _)), Some(c), None) => {
                        curr_slice_index = Some(id);
                        curr_row_slices.push(id, c);
                    },
                    (Some(&(id, _)), Some(_), Some(ci)) if id == ci => {},
                    (Some(&(id, _)), Some(_), Some(ci)) => {
                        prev_row_slices.merge_to_key(id, ci);
                    },
                }

                // move next
                match (prev_row_slices.get(prev_i), curr_slice) {
                    (None, Some(_)) => {
                        curr_slice = row_slices.next();
                        curr_slice_index = None;
                    },
                    (Some(_), None) => prev_i += 1,
                    (Some((_, p)), Some(c)) => {
                        if p.1 <= c.1 {
                            prev_i += 1;
                        }
                        if p.1 >= c.1 {
                            curr_slice = row_slices.next();
                            curr_slice_index = None;
                        }
                    },
                    _ => {},
                }
                
            }

            std::mem::swap(&mut prev_row_slices, &mut curr_row_slices);
            curr_row_slices.clear();
        }
        
        closed_islands += prev_row_slices.get_keys().count();
        closed_islands.try_into().unwrap()
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

    #[test]
    fn test_whole() {

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
        let number = Solution::num_islands(input_grid);

        println!("{:?}", number);
    }
}

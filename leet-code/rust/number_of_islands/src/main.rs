use std::{usize, vec};
use std::collections::HashMap;

#[derive(Default)]
pub struct GroupedVecMap<K, V> {
    map: HashMap<K, Vec<usize>>,
    order: Vec<(K, V)>,
}

impl<K, V> GroupedVecMap<K, V>
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

    pub fn iter(&self) -> std::slice::Iter<'_, (K, V)>: {
        self.order.iter()
    }
    
    pub fn get_by_key(&self, K key) ->  {

    }
}

// impl<'a, T> IntoIterator for &'a MyCollection<T> {
//     type Item = &'a T;
//     type IntoIter = Iter<'a, T>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }
// }
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut closed_islands = 0;
        let mut next_index = 0;
        let mut get_next = || {
            let id = next_index;
            next_index += 1;
            id
        };

        let mut prev_row_slices = GroupedVecMap<i32, (usize , usize)>::new();
        let mut curr_row_slices: Vec<((usize, usize), i32)> = vec![];
        for row in &grid {
            let mut prev_iter = prev_row_slices.iter();
            let mut row_slices = Self::parse_slices(row);
            let mut prev_slice_w_i = prev_iter.next();
            let mut curr_slice = row_slices.next();
            
            loop {
                match (prev_slice_w_i, curr_slice) {
                    (None, None) => break,
                    (None, Some(c)) => {
                        curr_row_slices.push((c, get_next()));
                        curr_slice = row_slices.next();
                    },
                    // curr_slice is before prev_slice completely
                    (Some((p, id)), Some(c)) if c.1 <= p.0 => {
                        curr_row_slices.push((c, get_next()));
                        curr_slice = row_slices.next();
                    },

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

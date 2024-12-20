use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

advent_of_code::solution!(20);

pub fn read_data(
    input: &str,
) -> (
    HashMap<(isize, isize), char>,
    (isize, isize),
    (isize, isize),
) {
    let mut map: HashMap<(isize, isize), char> = HashMap::new();
    let mut start: (isize, isize) = (0, 0);
    let mut end: (isize, isize) = (0, 0);
    input.lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, mut c)| {
            if c == 'S' {
                c = '.';
                start = (i as isize, j as isize);
            } else if c == 'E' {
                c = '.';
                end = (i as isize, j as isize);
            }
            map.insert((i as isize, j as isize), c);
        })
    });

    (map, start, end)
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn calculate_dist(
    map: &HashMap<(isize, isize), char>,
    start: &(isize, isize),
    end: &(isize, isize),
) -> HashMap<(isize, isize), isize> {
    let mut cache: HashMap<(isize, isize), isize> = HashMap::from([(*start, 0)]);
    
    let mut loc = *start;
    while loc != *end {
        for &dir in DIRECTIONS.iter() {
            let new_pos = (loc.0 + dir.0, loc.1 + dir.1);
            if !cache.contains_key(&new_pos) && map[&new_pos] == '.' {
                cache.insert(new_pos, cache.get(&loc).unwrap() + 1);
                loc = new_pos;
            }
        }
    }
    
    cache
}

pub fn traverse(
    map: &HashMap<(isize, isize), char>,
    start: &(isize, isize),
    end: &(isize, isize),
    cheat: isize,
) -> usize {
    let distances = calculate_dist(map, start, end);

    let mut result = 0;
    // let result = AtomicUsize::new(0);
    distances.iter().for_each(|(pos, d)| {
        for dy in -cheat..=cheat {
            let rest = cheat - dy.abs();
            for dx in -rest..=rest {
                let new_pos = (pos.0 + dy, pos.1 + dx);
                if distances.contains_key(&new_pos) {
                    let gain = distances.get(&new_pos).unwrap()
                        - dy.abs()
                        - dx.abs()
                        - d;

                    if gain >= 100 {
                        result += 1;
                        // result.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
        }
    });

    result
    // result.into_inner()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (map, start, end) = read_data(input);

    Some(traverse(&map, &start, &end, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (map, start, end) = read_data(input);

    Some(traverse(&map, &start, &end, 20))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

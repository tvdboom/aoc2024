use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::collections::{HashMap, VecDeque};
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

pub struct Loc {
    pos: (isize, isize),
    d: usize,
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn calculate_dist(
    map: &HashMap<(isize, isize), char>,
    position: &(isize, isize),
) -> HashMap<(isize, isize), usize> {
    let mut cache: HashMap<(isize, isize), usize> = HashMap::new();
    let mut queue: VecDeque<Loc> = VecDeque::new();
    queue.push_back(Loc {
        pos: *position,
        d: 0,
    });

    while let Some(loc) = queue.pop_front() {
        if cache.get(&loc.pos).unwrap_or(&usize::MAX) <= &loc.d {
            continue;
        }

        cache.insert(loc.pos, loc.d);
        for &dir in DIRECTIONS.iter() {
            let new_pos = (loc.pos.0 + dir.0, loc.pos.1 + dir.1);
            if map[&new_pos] == '.' && cache.get(&new_pos).unwrap_or(&usize::MAX) > &(loc.d + 1) {
                queue.push_back(Loc {
                    pos: new_pos,
                    d: loc.d + 1,
                });
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
    let cache_start = calculate_dist(map, start);
    let cache_end = calculate_dist(map, end);

    let min_d = cache_end.get(start).unwrap();

    let result = AtomicUsize::new(0);
    cache_start.par_iter().for_each(|(pos, steps)| {
        for dy in -cheat..=cheat {
            let rest = cheat - dy.abs();
            for dx in -rest..=rest {
                let new_pos = (pos.0 + dy, pos.1 + dx);
                match map.get(&new_pos) {
                    Some('.') if cache_end.contains_key(&new_pos) => {
                        let new_d = steps
                            + dy.unsigned_abs()
                            + dx.unsigned_abs()
                            + cache_end.get(&new_pos).unwrap();

                        if min_d >= &(100 + new_d) {
                            result.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                    _ => continue,
                }
            }
        }
    });

    result.into_inner()
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

use std::collections::{HashMap, HashSet};
use regex::Regex;

advent_of_code::solution!(14);

pub struct Robot {
    pos: (isize, isize),
    v: (isize, isize),
}

pub fn read_data(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    input
        .lines()
        .map(|l| {
            let g = re.captures(l).unwrap();
            Robot {
                pos: (g[1].parse().unwrap(), g[2].parse().unwrap()),
                v: (g[3].parse().unwrap(), g[4].parse().unwrap()),
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut robots = read_data(input);

    let (height, width) = (103, 101);
    let sec = 100;
    
    let mut quadrants = [0; 4];
    for r in &mut robots {
        // https://doc.rust-lang.org/std/primitive.isize.html#method.rem_euclid
        r.pos = (
            (r.pos.0 + r.v.0 * sec).rem_euclid(width),
            (r.pos.1 + r.v.1 * sec).rem_euclid(height),
        );

        if r.pos.0 < (width - 1) / 2 && r.pos.1 < (height - 1) / 2 {
            quadrants[0] += 1
        } else if r.pos.0 > (width - 1) / 2 && r.pos.1 < (height - 1) / 2 {
            quadrants[1] += 1
        } else if r.pos.0 < (width - 1) / 2 && r.pos.1 > (height - 1) / 2 {
            quadrants[2] += 1
        } else if r.pos.0 > (width - 1) / 2 && r.pos.1 > (height - 1) / 2 {
            quadrants[3] += 1
        }
    }

    Some(quadrants.iter().product())
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut robots = read_data(input);

    let (height, width) = (103, 101);

    let mut sec = 0;
    loop {
        sec += 1;

        let mut positions: HashMap<isize, HashSet<isize>> = HashMap::new();
        for r in &mut robots {
            let pos =(
                (r.pos.0 + r.v.0 * sec).rem_euclid(width),
                (r.pos.1 + r.v.1 * sec).rem_euclid(height),
            );
            positions.entry(pos.0).or_insert_with(HashSet::new).insert(pos.1);
        }

        // Check if there are at least 30 consecutive robots in the same row
        // Assumption: A christmas tree looks like this:
        // ---#---
        // --###--
        // -#####-
        // #######
        // ---#---
        if positions.values().any(|xs| {
            let mut xs: Vec<_> = xs.iter().copied().collect();
            xs.sort_unstable();
            let mut count = 1;

            for i in 1..xs.len() {
                if xs[i] == xs[i - 1] + 1 {
                    count += 1;
                    if count >= 30 {
                        return true;
                    }
                } else {
                    count = 1;
                }
            }
            false
        }) {
            return Some(sec);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

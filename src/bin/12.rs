use std::collections::{HashMap, HashSet};

advent_of_code::solution!(12);

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

const CORNERS: [[(isize, isize); 3]; 4] = [
    [(0, -1), (1, -1), (1, 0)],
    [(1, 0), (1, 1), (0, 1)],
    [(0, 1), (-1, 1), (-1, 0)],
    [(-1, 0), (-1, -1), (0, -1)],
];

pub fn read_map(input: &str) -> HashMap<(usize, usize), char> {
    let mut map = HashMap::new();
    for (i, l) in input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            map.insert((i, j), c);
        }
    }
    map
}

pub fn resolve_region(
    pos: (usize, usize),
    map: &HashMap<(usize, usize), char>,
    visited: &mut HashSet<(usize, usize)>,
    part2: bool,
) -> (usize, usize, usize) {
    visited.insert(pos);

    let mut area = 1;
    let mut perimeter = 0;
    let mut corners = 0;
    for (dx, dy) in DIRECTIONS.iter() {
        let new_pos = (
            (pos.0 as isize + dx) as usize,
            (pos.1 as isize + dy) as usize,
        );

        if let Some(&c) = map.get(&new_pos) {
            if !visited.contains(&new_pos) && map[&pos] == c {
                let (a, p, c) = resolve_region(new_pos, map, visited, part2);
                area += a;
                perimeter += p;
                corners += c;
            } else if map[&pos] != c {
                perimeter += 1;
            }
        } else {
            perimeter += 1;
        }
    }

    if part2 {
        for corner in CORNERS {
            let any_corner = corner
                .iter()
                .map(|(dx, dy)| {
                    let new_pos = (
                        (pos.0 as isize + dx) as usize,
                        (pos.1 as isize + dy) as usize,
                    );

                    match map.get(&new_pos) {
                        Some(&c) if map[&pos] == c => 1,
                        _ => 0,
                    }
                })
                .collect::<Vec<usize>>();

            if any_corner == [0, 0, 0] || any_corner == [1, 0, 1] || any_corner == [0, 1, 0] {
                corners += 1;
            }
        }
    }

    (area, perimeter, corners)
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = read_map(input);

    let mut result = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for pos in map.keys() {
        if !visited.contains(pos) {
            let (a, p, _) = resolve_region(*pos, &map, &mut visited, false);
            result += a * p;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = read_map(input);

    let mut result = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for pos in map.keys() {
        if !visited.contains(pos) {
            let (a, _, c) = resolve_region(*pos, &map, &mut visited, true);
            result += a * c;
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}

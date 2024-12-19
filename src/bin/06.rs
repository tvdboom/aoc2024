advent_of_code::solution!(6);

use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub fn read_data(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn resolve(
    matrix: &[Vec<char>],
    d: usize,
    mut pos: (usize, usize),
    mut direction: char,
) -> Option<HashSet<((usize, usize), char)>> {
    let mut path: HashSet<((usize, usize), char)> = HashSet::new();

    loop {
        if path.contains(&(pos, direction)) {
            return None; // We are in infinite loop
        }

        path.insert((pos, direction));

        match direction {
            '^' => {
                if pos.0 == 0 {
                    break;
                }
                if matrix[pos.0 - 1][pos.1] == '#' {
                    direction = '>';
                } else {
                    pos.0 -= 1;
                }
            }
            'v' => {
                if pos.0 == d - 1 {
                    break;
                }
                if matrix[pos.0 + 1][pos.1] == '#' {
                    direction = '<';
                } else {
                    pos.0 += 1;
                }
            }
            '<' => {
                if pos.1 == 0 {
                    break;
                }
                if matrix[pos.0][pos.1 - 1] == '#' {
                    direction = '^';
                } else {
                    pos.1 -= 1;
                }
            }
            '>' => {
                if pos.1 == d - 1 {
                    break;
                }
                if matrix[pos.0][pos.1 + 1] == '#' {
                    direction = 'v';
                } else {
                    pos.1 += 1;
                }
            }
            _ => panic!("Invalid direction"),
        }
    }

    Some(path)
}

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = read_data(input);

    // Matrix has dimensions d x d
    let d: usize = matrix.len();

    let direction: char = '^';

    // Get guard's starting position
    let mut pos: (usize, usize) = (0, 0);
    'outer: for i in 0..d {
        for j in 0..d {
            if matrix[i][j] == direction {
                pos = (i, j);
                break 'outer;
            }
        }
    }

    let visited = resolve(&matrix, d, pos, direction).unwrap();

    Some(visited.iter().map(|x| x.0).collect::<HashSet<_>>().len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let matrix = read_data(input);

    // Matrix has dimensions d x d
    let d: usize = matrix.len();

    let direction: char = '^';

    // Get guard's starting position
    let mut pos: (usize, usize) = (0, 0);
    'outer: for i in 0..d {
        for j in 0..d {
            if matrix[i][j] == direction {
                pos = (i, j);
                break 'outer;
            }
        }
    }

    let path = resolve(&matrix, d, pos, direction).unwrap();

    let blocks = Arc::new(Mutex::new(HashSet::new()));
    path.par_iter().for_each(|x| {
        if x.0 != pos {
            let mut new_map = matrix.clone();
            new_map[x.0 .0][x.0 .1] = '#';
            if resolve(&new_map, d, pos, direction).is_none() {
                blocks.lock().unwrap().insert(x.0);
            }
        }
    });

    let result = blocks.lock().unwrap().len();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

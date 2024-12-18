use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(18);

pub fn read_data(input: &str, n: usize) -> HashSet<(isize, isize)> {
    let mut set = HashSet::new();
    input.lines().take(n).for_each(|l| {
        let (x, y) = l.split_once(',').unwrap();
        set.insert((x.parse().unwrap(), y.parse().unwrap()));
    });

    set
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn traverse(d: isize, map: &mut HashSet<(isize, isize)>) -> Option<usize> {
    let mut queue: VecDeque<((isize, isize), usize)> = VecDeque::from([((0, 0), 0)]);

    while let Some((pos, steps)) = queue.pop_front() {
        if pos.0 == d - 1 && pos.1 == d - 1 {
            return Some(steps);
        }

        for &dir in DIRECTIONS.iter() {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);

            if new_pos.0 >= 0
                && new_pos.0 < d
                && new_pos.1 >= 0
                && new_pos.1 < d
                && !map.contains(&new_pos)
            {
                map.insert(new_pos);
                queue.push_back((new_pos, steps + 1));
            }
        }
    }

    None  // No possible path!
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut map = read_data(input, 1024);
    Some(traverse(71, &mut map)?)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut i = 1025;
    while i < input.lines().count() {
        // Re-reading the map every time is inefficient, but don't want to refactor
        let mut map = read_data(input, i);

        if traverse(71, &mut map).is_none() {
            break;
        }

        // Horribly slow (20s, almost stopped it), because I am trying every
        // possible path. Prob better to try at half, then half of half, etc
        i += 1;
    }

    Some(input.lines().nth(i - 1).unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("6,1")));
    }
}

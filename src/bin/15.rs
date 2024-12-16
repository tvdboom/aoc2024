use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(15);

pub fn read_data(input: &str) -> (HashMap<(isize, isize), char>, (isize, isize), &str) {
    let (grid, movements) = input.split_once("\n\n").unwrap();

    let mut pos: (isize, isize) = (0, 0);
    let mut map: HashMap<(isize, isize), char> = HashMap::new();
    grid.lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            if c == '@' {
                pos = (i as isize, j as isize);
            }
            map.insert((i as isize, j as isize), c);
        })
    });

    (map, pos, movements)
}

pub fn read_data2(input: &str) -> (HashMap<(isize, isize), char>, (isize, isize), &str) {
    let (grid, movements) = input.split_once("\n\n").unwrap();

    let mut pos: (isize, isize) = (0, 0);
    let mut map: HashMap<(isize, isize), char> = HashMap::new();
    grid.lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| match c {
            '#' => {
                map.insert((i as isize, 2 * j as isize), '#');
                map.insert((i as isize, (2 * j + 1) as isize), '#');
            }
            'O' => {
                map.insert((i as isize, 2 * j as isize), '[');
                map.insert((i as isize, (2 * j + 1) as isize), ']');
            }
            '.' => {
                map.insert((i as isize, 2 * j as isize), '.');
                map.insert((i as isize, (2 * j + 1) as isize), '.');
            }
            '@' => {
                pos = (i as isize, 2 * j as isize);
                map.insert((i as isize, 2 * j as isize), '@');
                map.insert((i as isize, (2 * j + 1) as isize), '.');
            }
            _ => panic!("Invalid character"),
        })
    });

    (map, pos, movements)
}

pub fn do_move(
    pos: &(isize, isize),
    d: (isize, isize),
    map: &HashMap<(isize, isize), char>,
) -> isize {
    let mut s = 0;
    let mut next = pos.clone();
    loop {
        next = (next.0 + d.0, next.1 + d.1);
        match map[&next] {
            '#' => {
                s = 0;
                break;
            }
            '.' => {
                s += 1;
                break;
            }
            'O' => {
                s += 1;
            }
            _ => panic!("Invalid character"),
        }
    }
    s
}

pub fn part_one(input: &str) -> Option<isize> {
    let (mut map, mut pos, movements) = read_data(input);

    for c in movements.chars() {
        let d: (isize, isize) = match c {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => continue,
        };

        let mut s = 0;
        let mut next = pos.clone();
        loop {
            next = (next.0 + d.0, next.1 + d.1);
            match map[&next] {
                '#' => {
                    s = 0;
                    break;
                }
                '.' => {
                    s += 1;
                    break;
                }
                'O' => {
                    s += 1;
                }
                _ => panic!("Invalid character"),
            }
        }

        if s > 0 {
            for i in (1..=s).rev() {
                map.insert(
                    (pos.0 + (i * d.0), pos.1 + (i * d.1)),
                    map[&(pos.0 + ((i - 1) * d.0), pos.1 + ((i - 1) * d.1))],
                );
            }
            map.insert(pos, '.');
            pos = (pos.0 + d.0, pos.1 + d.1);
        }
    }

    let result = map
        .iter()
        .map(|(&p, &c)| if c == 'O' { 100 * p.0 + p.1 } else { 0 })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<isize> {
    let (mut map, mut pos, movements) = read_data2(input);

    for i in 0..10 {
        for j in 0..20 {
            print!("{}", map[&(i, j)]);
        }
        println!();
    }

    's: for c in movements.chars() {
        let d: (isize, isize) = match c {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => continue,
        };

        let mut queue = VecDeque::new();
        queue.push_front(pos);

        let mut visited = HashSet::new();
        while let Some(x) = queue.pop_front() {
            if !visited.contains(&x) {
                visited.insert(x);

                let next = (x.0 + d.0, x.1 + d.1);
                match map[&next] {
                    '.' => continue,
                    '[' => queue.extend([next, (next.0, next.1 + 1)]),
                    ']' => queue.extend([next, (next.0, next.1 - 1)]),
                    '#' => continue 's,
                    _ => panic!("Invalid character"),
                }
            }
        }

        while !visited.is_empty() {
            for p in visited.clone() {
                let next = (p.0 + d.0, p.1 + d.1);
                if !visited.contains(&next) {
                    map.insert(next, map[&p]);
                    map.insert(p, '.');
                    visited.remove(&p);
                }
            }
        }

        pos = (pos.0 + d.0, pos.1 + d.1);
    }

    let result = map
        .iter()
        .map(|(&p, &c)| if c == '[' { 100 * p.0 + p.1 } else { 0 })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}

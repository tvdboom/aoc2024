use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(16);

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
    d: char,
    points: usize,
    path: Vec<(isize, isize)>,
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn traverse(
    map: &HashMap<(isize, isize), char>,
    start: &(isize, isize),
    end: &(isize, isize),
) -> (HashMap<((isize, isize), char), usize>, Vec<(isize, isize)>) {
    let mut visited: HashMap<((isize, isize), char), usize> = HashMap::new();
    let mut min_points = usize::MAX;
    let mut paths = Vec::new();

    let mut queue: VecDeque<Loc> = VecDeque::from([Loc {
        pos: *start,
        d: '>',
        points: 0,
        path: vec![*start],
    }]);

    while let Some(loc) = queue.pop_front() {
        if visited
            .get(&(loc.pos, loc.d))
            .map_or(false, |&p| p < loc.points)
        {
            continue;
        }

        if loc.pos == *end {
            if loc.points < min_points {
                paths = loc.path.clone(); // Reset paths
                min_points = loc.points;
            } else if loc.points == min_points {
                paths.extend(loc.path.clone());
            }
        }

        visited.insert((loc.pos, loc.d), loc.points);
        for &dir in DIRECTIONS.iter() {
            let new_pos = (loc.pos.0 + dir.0, loc.pos.1 + dir.1);
            match map[&new_pos] {
                '#' => continue,
                '.' => {
                    let new_d = match dir {
                        (0, 1) => '>',
                        (1, 0) => 'v',
                        (0, -1) => '<',
                        (-1, 0) => '^',
                        _ => panic!("Invalid direction"),
                    };

                    let mut points = loc.points;
                    if loc.d == new_d {
                        points += 1;
                    } else {
                        points += 1001;
                    }

                    if points <= *visited.get(&(new_pos, new_d)).unwrap_or(&usize::MAX) {
                        let mut path = loc.path.clone();
                        path.push(new_pos);

                        queue.push_back(Loc {
                            pos: new_pos,
                            d: new_d,
                            points,
                            path,
                        });
                    }
                }
                _ => panic!("Invalid character"),
            }
        }
    }

    (visited, paths)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (map, start, end) = read_data(input);
    let (visited, _) = traverse(&map, &start, &end);

    let result = visited
        .iter()
        .filter_map(|((pos, _), &points)| if *pos == end { Some(points) } else { None })
        .min();
    Some(result?)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (map, start, end) = read_data(input);
    let (_, paths) = traverse(&map, &start, &end);

    Some(paths.into_iter().collect::<HashSet<(isize, isize)>>().len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}

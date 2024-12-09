use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn read_data(input: &str) -> HashMap<char, Vec<(isize, isize)>> {
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    input.lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            if c != '.' {
                antennas
                    .entry(c)
                    .or_default()
                    .push((i as isize, j as isize));
            }
        });
    });

    antennas
}

pub fn part_one(input: &str) -> Option<usize> {
    let antennas = read_data(input);
    let d = input.lines().count() as isize;
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    antennas.iter().for_each(|(_, v)| {
        for (x, (i, j)) in v.iter().enumerate() {
            for (ii, jj) in v.iter().skip(x + 1) {
                let xx = 2 * i - ii;
                let yy = 2 * j - jj;
                if xx >= 0 && xx < d && yy >= 0 && yy < d {
                    antinodes.insert((xx, yy));
                }

                let xx = 2 * ii - i;
                let yy = 2 * jj - j;
                if xx >= 0 && xx < d && yy >= 0 && yy < d {
                    antinodes.insert((xx, yy));
                }
            }
        }
    });

    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let antennas = read_data(input);
    let d = input.lines().count() as isize;
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    antennas.iter().for_each(|(_, v)| {
        for (x, (i, j)) in v.iter().enumerate() {
            for (ii, jj) in v.iter().skip(x + 1) {
                let xx = 2 * i - ii;
                let yy = 2 * j - jj;
                let step = (i - xx, j - yy);

                let mut pos = (*i, *j);
                while pos.0 >= 0 && pos.0 < d && pos.1 >= 0 && pos.1 < d {
                    antinodes.insert(pos);
                    pos = (pos.0 + step.0, pos.1 + step.1);
                }

                pos = (*i - step.0, *j - step.1);
                while pos.0 >= 0 && pos.0 < d && pos.1 >= 0 && pos.1 < d {
                    antinodes.insert(pos);
                    pos = (pos.0 - step.0, pos.1 - step.1);
                }
            }
        }
    });

    Some(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}

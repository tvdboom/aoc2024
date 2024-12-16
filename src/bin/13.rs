advent_of_code::solution!(13);

use regex::Regex;

#[derive(Debug)]
pub struct Machine {
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize),
}

pub fn read_data(input: &str) -> Vec<Machine> {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\s+Button B: X\+(\d+), Y\+(\d+)\s+Prize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    input
        .trim()
        .split("\n\n")
        .filter_map(|block| {
            if let Some(caps) = re.captures(block) {
                Some(Machine {
                    a: (caps[1].parse().unwrap(), caps[2].parse().unwrap()),
                    b: (caps[3].parse().unwrap(), caps[4].parse().unwrap()),
                    prize: (caps[5].parse().unwrap(), caps[6].parse().unwrap()),
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<isize> {
    let machines = read_data(input);

    let mut result = 0;
    for m in machines {
        let numerator1 = m.prize.1 * m.b.0 - m.prize.0 * m.b.1;
        let numerator2 = m.prize.1 * m.a.0 - m.prize.0 * m.a.1;
        let denominator = m.a.1 * m.b.0 - m.a.0 * m.b.1;
        if numerator1 % denominator == 0 && numerator2 % -denominator == 0 {
            result += 3 * (numerator1 / denominator) + numerator2 / -denominator;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<isize> {
    let machines = read_data(input);

    let mut result = 0;
    for mut m in machines {
        m.prize = (
            m.prize.0 + 10_000_000_000_000,
            m.prize.1 + 10_000_000_000_000,
        );

        let numerator1 = m.prize.1 * m.b.0 - m.prize.0 * m.b.1;
        let numerator2 = m.prize.1 * m.a.0 - m.prize.0 * m.a.1;
        let denominator = m.a.1 * m.b.0 - m.a.0 * m.b.1;
        if numerator1 % denominator == 0 && numerator2 % -denominator == 0 {
            result += 3 * (numerator1 / denominator) + numerator2 / -denominator;
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

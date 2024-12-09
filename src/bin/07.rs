advent_of_code::solution!(7);

use rayon::prelude::*;

pub fn read_data(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|l| {
            let parts = l.split_once(": ").unwrap();
            let result = parts.0.parse::<usize>().unwrap();
            let numbers = parts
                .1
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (result, numbers)
        })
        .collect()
}

pub fn is_valid(result: usize, numbers: &[usize], allow_concat: bool) -> Option<usize> {
    let mut entry = Vec::from([(result, numbers.len() - 1)]);

    // Solve from right to left using contrary operations
    while let Some((r, n)) = entry.pop() {
        if n == 0 && numbers[n] == r {
            return Some(result);
        }

        if n > 0 && numbers[n] <= r {
            // Try addition
            entry.push((r - numbers[n], n - 1));

            // Try division if we can divide exactly...
            if r % numbers[n] == 0 {
                entry.push((r / numbers[n], n - 1));
            }

            if allow_concat {
                // Try inverse concatenation
                let n1d = r.to_string().len() as u32;
                let n2d = numbers[n].to_string().len() as u32;
                if n1d >= n2d && r % 10usize.pow(n2d) == numbers[n] {
                    entry.push((r / 10usize.pow(n2d), n - 1));
                }
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let data = read_data(input);

    let result = data
        .into_par_iter()
        .filter_map(|(r, numbers)| is_valid(r, &numbers, false))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let data = read_data(input);

    let result = data
        .into_par_iter()
        .filter_map(|(r, numbers)| is_valid(r, &numbers, true))
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}

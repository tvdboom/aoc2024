use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn read_data(input: &str) -> Vec<usize> {
    input
        .trim()
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect()
}

pub fn blink(n: usize) -> Vec<usize> {
    let n_str = n.to_string();
    match n {
        0 => vec![1],
        _ if n_str.len() % 2 == 0 => {
            let n_str = n.to_string();
            let (left, right) = n_str.split_at(n_str.len() / 2);
            let left = left.parse::<usize>().unwrap();
            let right = right.parse::<usize>().unwrap();
            if left > 0 {
                vec![left, right]
            } else {
                vec![right]
            }
        }
        _ => vec![n * 2024],
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut data = read_data(input);

    for _ in 0..25 {
        let mut next = vec![];
        for n in data {
            next.extend(blink(n));
        }
        data = next;
    }

    Some(data.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let data = read_data(input);
    let mut data: HashMap<usize, usize> = data.iter().map(|n| (*n, 1)).collect();

    for _ in 0..75 {
        let mut memory = HashMap::new();
        for (n, c) in data {
            for s in blink(n) {
                memory.entry(s).and_modify(|count| *count += c).or_insert(c);
            }
        }
        data = memory;
    }

    Some(data.into_values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

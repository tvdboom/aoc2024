use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn parse_data(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            (nums[0], nums[1])
        })
        .unzip()
}

pub fn part_one(input: &str) -> Option<i32> {
    let (mut v1, mut v2): (Vec<i32>, Vec<i32>) = parse_data(input);

    v1.sort_unstable();
    v2.sort_unstable();

    let result = v1.iter().zip(v2.iter()).map(|(a, b)| (a - b).abs()).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (v1, v2): (Vec<i32>, Vec<i32>) = parse_data(input);

    let counts = v2.iter().copied().fold(HashMap::new(), |mut map, x| {
        *map.entry(x).or_insert(0) += 1;
        map
    });

    let result = v1.iter().map(|&n| n * *counts.get(&n).unwrap_or(&0)).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}

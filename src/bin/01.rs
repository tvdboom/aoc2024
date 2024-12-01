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

    let result = v1.iter()
        .zip(v2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (v1, v2): (Vec<i32>, Vec<i32>) = parse_data(input);

    let result = v1.iter()
        .map(|&n| n * v2.iter().filter(|&&x| x == n).count() as i32)
        .sum();

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

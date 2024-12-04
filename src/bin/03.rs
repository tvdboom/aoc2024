advent_of_code::solution!(3);

use regex::{Regex, RegexBuilder};


pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let result = re.captures_iter(input)
        .map(|x| x[1].parse::<u32>().unwrap() * x[2].parse::<u32>().unwrap())
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Search for don't() and stop at do()
    let re = RegexBuilder::new(r"don't\(\).*?(?:do\(\)|$)|mul\((\d{1,3}),(\d{1,3})\)")
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    // Filter out if no first digit group (don't followed by do)
    let result = re.captures_iter(input)
        .filter(|x| x.get(1).is_some())
        .map(|x| x[1].parse::<u32>().unwrap() * x[2].parse::<u32>().unwrap())
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}

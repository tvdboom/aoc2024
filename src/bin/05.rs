advent_of_code::solution!(5);

use std::collections::{HashMap, HashSet};

pub fn read_data(input: &str) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let parts = input.split_once("\n\n").expect("Invalid input format");

    let rules = parts
        .0
        .lines()
        .map(|l| l.split('|').map(|n| n.parse::<u32>().unwrap()).collect())
        .collect();

    let pages = parts
        .1
        .lines()
        .map(|l| l.split(',').map(|n| n.parse::<u32>().unwrap()).collect())
        .collect();

    (rules, pages)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, pages) = read_data(input);

    // Dict of number with all numbers that must come before it
    let mut all_rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    rules.iter().for_each(|v| {
        all_rules.entry(v[1]).or_default().insert(v[0]);
    });

    let result: u32 = pages
        .iter()
        .filter(|v| {
            v.is_sorted_by(|x, y| match all_rules.get(y) {
                Some(v) => v.contains(x),
                None => false,
            })
        })
        .map(|v| v[v.len() / 2])
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, pages) = read_data(input);

    // Dict of number with all numbers that must come before it
    let mut all_rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    rules.iter().for_each(|v| {
        all_rules.entry(v[1]).or_default().insert(v[0]);
    });

    let result: u32 = pages
        .iter()
        .map(|v| {
            if !v.is_sorted_by(|x, y| match all_rules.get(y) {
                Some(v) => v.contains(x),
                None => false,
            }) {
                let mut v2 = v.clone();
                v2.sort_by(|x, y| all_rules[y].contains(x).cmp(&true));
                v2[v2.len() / 2]
            } else {
                0
            }
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

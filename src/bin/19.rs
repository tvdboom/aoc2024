use dashmap::DashMap;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::collections::HashSet;

advent_of_code::solution!(19);

pub fn read_data(input: &str) -> (HashSet<&str>, Vec<&str>) {
    let data = input.split_once("\n\n").unwrap();

    let patterns = data.0.split(", ").collect::<HashSet<&str>>();
    let towels = data.1.lines().collect();

    (patterns, towels)
}

pub fn make_towel<'a>(
    towel: &'a str,
    patterns: &HashSet<&'a str>,
    cache: &DashMap<&'a str, usize>,
) -> usize {
    if let Some(visited) = cache.get(towel) {
        return *visited;
    }

    let result = patterns
        .iter()
        .filter_map(|p| {
            if towel.starts_with(*p) {
                let (_, remainder) = towel.split_at(p.len());
                if remainder.is_empty() {
                    Some(1)
                } else {
                    Some(make_towel(remainder, patterns, cache))
                }
            } else {
                None
            }
        })
        .sum();

    cache.insert(towel, result);

    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let (patterns, towels) = read_data(input);

    let cache: DashMap<&str, usize> = DashMap::new();
    Some(
        towels
            .par_iter()
            .filter(|t| make_towel(t, &patterns, &cache) > 0)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (patterns, towels) = read_data(input);
    
    let cache: DashMap<&str, usize> = DashMap::new();
    Some(
        towels
            .par_iter()
            .map(|t| make_towel(t, &patterns, &cache))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}

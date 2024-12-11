use dashmap::DashSet;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

advent_of_code::solution!(10);

pub fn read_data(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.chars().map(|x| x.to_digit(10).unwrap() as u8).collect())
        .collect()
}

pub fn traverse(i: usize, j: usize, data: &Vec<Vec<u8>>, count: &mut DashSet<(usize, usize)>) {
    if data[i][j] == 9 {
        count.insert((i, j));
    }

    // Improvements: Skip direction I came from
    for (dx, dy) in [(-1isize, 0), (0, 1), (1, 0), (0, -1)].iter() {
        let ii = i as isize + dx;
        let jj = j as isize + dy;
        let d = data.len() as isize;
        if ii >= 0
            && ii < d
            && jj >= 0
            && jj < d
            && data[ii as usize][jj as usize] == data[i][j] + 1
        {
            traverse(ii as usize, jj as usize, data, count);
        }
    }
}

pub fn traverse2(i: usize, j: usize, data: &Vec<Vec<u8>>, count: &AtomicUsize) {
    if data[i][j] == 9 {
        count.fetch_add(1, Ordering::Relaxed);
    }

    for (dx, dy) in [(-1isize, 0), (0, 1), (1, 0), (0, -1)].iter() {
        let ii = i as isize + dx;
        let jj = j as isize + dy;
        let d = data.len() as isize;
        if ii >= 0
            && ii < d
            && jj >= 0
            && jj < d
            && data[ii as usize][jj as usize] == data[i][j] + 1
        {
            traverse2(ii as usize, jj as usize, data, count);
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let data = read_data(input);

    let result = AtomicUsize::new(0);

    data.par_iter().enumerate().for_each(|(i, col)| {
        col.par_iter().enumerate().for_each(|(j, _)| {
            if data[i][j] == 0 {
                let mut count: DashSet<(usize, usize)> = DashSet::with_capacity(1024);
                traverse(i, j, &data, &mut count);
                result.fetch_add(count.len(), Ordering::Relaxed);
            }
        })
    });

    Some(result.load(Ordering::Relaxed))
}

pub fn part_two(input: &str) -> Option<usize> {
    let data = read_data(input);

    let result = AtomicUsize::new(0);

    data.par_iter().enumerate().for_each(|(i, col)| {
        col.par_iter().enumerate().for_each(|(j, _)| {
            if data[i][j] == 0 {
                traverse2(i, j, &data, &result);
            }
        })
    });

    Some(result.load(Ordering::Relaxed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}

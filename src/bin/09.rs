advent_of_code::solution!(9);

pub fn read_data(input: &str) -> Vec<isize> {
    let data: Vec<usize> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut disk: Vec<isize> = Vec::new();
    data.iter().enumerate().for_each(|(i, d)| {
        let x = if i % 2 == 0 {
            i as isize / 2
        } else {
            -1 // Use -1 instead of period
        };
        disk.extend(vec![x; *d]);
    });

    disk
}

pub fn part_one(input: &str) -> Option<usize> {
    let disk = read_data(input);

    let mut fs = disk.clone();
    for (i, d) in disk.iter().enumerate() {
        if *d < 0 {
            let n = fs.iter().rposition(|x| *x >= 0).unwrap();
            if i >= n {
                break;
            }
            fs.swap(i, n);
        }
    }

    let result = fs
        .iter()
        .enumerate()
        .filter(|n| *n.1 > 0)
        .map(|(i, n)| *n as usize * i)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let disk = read_data(input);

    let mut number: isize = -2;
    let mut space: usize = 0;
    let mut disk2: Vec<(isize, usize)> = Vec::new();

    disk.iter().for_each(|x| {
        if *x == number {
            space += 1;
        } else {
            if number >= -1 {
                disk2.push((number, space));
            }
            number = *x;
            space = 1;
        }
    });

    disk2.push((number, space));

    let mut fs = disk2.clone();
    for (d, s) in disk2.iter() {
        if *d < 0 {
            if let Some(idx) = fs.iter().rposition(|(dd, ss)| *dd >= 0 && s >= ss) {
                let elem = fs.remove(idx);

                fs.insert(idx, (-1, elem.1));

                // Correct the spaces of the empty element
                let ii = fs.iter().position(|xx| xx == &(*d, *s)).unwrap();
                if fs[ii].1 > elem.1 {
                    fs[ii].1 -= elem.1;
                } else {
                    fs.remove(ii);
                }

                fs.insert(ii, elem);
            }
        }
    }

    let mut count: isize = 0;
    let result = fs
        .iter()
        .map(|n| {
            let mut sum = 0;
            for _ in 0..n.1 {
                if n.0 > 0 {
                    sum += count * n.0;
                }
                count += 1;
            }
            sum as usize
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}

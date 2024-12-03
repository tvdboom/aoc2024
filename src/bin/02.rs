advent_of_code::solution!(2);

pub fn parse_data(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn is_valid(x: &Vec<i32>) -> bool {
    let increasing = x[1] > x[0];
    x.windows(2).all(|w| {
        let diff = w[1] - w[0];
        if increasing {
            (1..4).contains(&diff)
        } else {
            (-3..0).contains(&diff)
        }
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let v = parse_data(input);

    let result = v.iter().filter(|x| is_valid(x)).count();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let v = parse_data(input);

    let result = v
        .iter()
        .filter(|x| {
            if is_valid(x) {
                return true;
            }

            // Brute force...
            (0..x.len()).any(|i| {
                let mut new_v = (*x).clone();
                new_v.remove(i);
                is_valid(&new_v)
            })
        })
        .count();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}

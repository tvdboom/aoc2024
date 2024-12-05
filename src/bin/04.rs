advent_of_code::solution!(4);

pub fn read_data(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let xmas = Vec::from(['X', 'M', 'A', 'S']);

    let mut count: u32 = 0;
    let data: Vec<Vec<char>> = read_data(input);

    // Dimension of data is d x d
    let d: usize = data.len();

    for (i, v) in data.iter().enumerate() {
        for (j, c) in v.iter().enumerate() {
            if c == &'X' {
                // If-statements could be made more compact, but this is readable:)

                // East
                if j <= d - 4 && v[j..j + 4] == xmas {
                    count += 1;
                }

                // South-east
                if j <= d - 4 && i <= d - 4 && (1..4).all(|k| data[i + k][j + k] == xmas[k]) {
                    count += 1;
                }

                // South
                if i <= d - 4 && (1..4).all(|k| data[i + k][j] == xmas[k]) {
                    count += 1;
                }

                // South-west
                if j >= 3 && i <= d - 4 && (1..4).all(|k| data[i + k][j - k] == xmas[k]) {
                    count += 1;
                }

                // West
                if j >= 3 && (1..4).all(|k| data[i][j - k] == xmas[k]) {
                    count += 1;
                }

                // North-west
                if j >= 3 && i >= 3 && (1..4).all(|k| data[i - k][j - k] == xmas[k]) {
                    count += 1;
                }

                // North
                if i >= 3 && (1..4).all(|k| data[i - k][j] == xmas[k]) {
                    count += 1;
                }

                // North-east
                if j <= d - 4 && i >= 3 && (1..4).all(|k| data[i - k][j + k] == xmas[k]) {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count: u32 = 0;
    let data: Vec<Vec<char>> = read_data(input);

    // Dimension of data is d x d
    let d: usize = data.len();

    for (i, v) in data.iter().enumerate() {
        for (j, c) in v.iter().enumerate() {
            // Don't check outer borders
            if i > 0 && i < d - 1 && j > 0 && j < d - 1 && c == &'A' {
                let nw = data[i - 1][j - 1]; // North-west
                let se = data[i + 1][j + 1]; // South-east
                let ne = data[i - 1][j + 1]; // North-east
                let sw = data[i + 1][j - 1]; // South-west

                if ((nw == 'S' && se == 'M') || (nw == 'M' && se == 'S'))
                    && ((ne == 'S' && sw == 'M') || (ne == 'M' && sw == 'S'))
                {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}

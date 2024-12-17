advent_of_code::solution!(17);

pub fn read_data(input: &str) -> Computer {
    let numbers: Vec<usize> = input
        .split(|c: char| !c.is_numeric())
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    Computer {
        a: numbers[0],
        b: numbers[1],
        c: numbers[2],
        pointer: 0,
        program: numbers[3..].iter().map(|&n| n as u8).collect(),
    }
}

#[derive(Debug)]
pub struct Computer {
    a: usize,
    b: usize,
    c: usize,
    pointer: usize,
    program: Vec<u8>,
}

impl Computer {
    fn combo(&self, i: u8) -> usize {
        match i {
            i @ 0..=3 => i as usize,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }

    fn do_operation(&mut self) -> Option<u8> {
        while self.pointer < self.program.len() {
            let opcode = self.program[self.pointer];
            let operand = self.program[self.pointer + 1];

            match opcode {
                0 => self.a >>= self.combo(operand), // divide by 2**n by shifting n bits to the right
                1 => self.b ^= operand as usize,
                2 => self.b = self.combo(operand) % 8,
                3 => {
                    if self.a > 0 {
                        self.pointer = operand as usize;
                        continue;
                    }
                }
                4 => self.b ^= self.c,
                5 => {
                    self.pointer += 2;
                    return Some((self.combo(operand) % 8) as u8);
                }
                6 => self.b = self.a >> self.combo(operand),
                7 => self.c = self.a >> self.combo(operand),
                _ => unreachable!(),
            }

            self.pointer += 2;
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = read_data(input);
    println!("{:?}", computer);

    let mut result: Vec<char> = Vec::new();
    while let Some(n) = computer.do_operation() {
        result.extend([(n + b'0') as char, ',']);
    }
    result.pop(); // Drop trailing comma

    Some(result.iter().collect())
}

pub fn part_two(input: &str) -> Option<usize> {
    let start = read_data(input);

    let mut result = Vec::from([0]);

    // Reverse engineer
    for operand in start.program.iter().rev() {
        let mut result2 = Vec::new();

        for res in result {
            for n in 0..=7 {
                // Every value in the output only changes after every power of 8
                // First number changes every 1, second every 8, third every 64, etc...
                // So we can convert A to the target times 8 plus the nth position (shift 3 bits)
                let a = 8 * res + n;
                let mut computer = Computer {
                    a,
                    b: 0,
                    c: 0,
                    pointer: 0,
                    program: start.program.clone(),
                };

                match computer.do_operation() {
                    // If the result matches the last digit of the target, add it to the list
                    Some(r) if r == *operand => result2.push(a),
                    _ => (),
                }
            }
        }

        result = result2;
    }

    Some(*result.iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("4,6,3,5,6,3,5,2,1,0")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}

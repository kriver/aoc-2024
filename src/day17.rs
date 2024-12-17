use crate::util::load;

#[derive(Debug)]
pub struct Computer {
    ip: usize,
    reg: [u64; 3],
    input: Vec<u64>,
    output: Vec<u64>,
}

impl Computer {
    fn init(&mut self, a: u64) {
        self.ip = 0;
        self.reg = [a, 0, 0];
        self.output = vec![];
    }

    fn combo(&self, v: u64) -> u64 {
        match v {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg[0],
            5 => self.reg[1],
            6 => self.reg[2],
            _ => unreachable!("invalid value"),
        }
    }

    fn output(&self) -> String {
        self.output
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn run(&mut self) {
        loop {
            if self.ip >= self.input.len() {
                break;
            }
            let instr = self.input[self.ip];
            let operand = self.input[self.ip + 1];
            match instr {
                0 => self.reg[0] /= 2u64.pow(self.combo(operand) as u32),
                1 => self.reg[1] ^= operand,
                2 => self.reg[1] = self.combo(operand) % 8,
                3 => {
                    if self.reg[0] != 0 {
                        self.ip = operand as usize;
                        continue;
                    }
                }
                4 => self.reg[1] ^= self.reg[2],
                5 => self.output.push(self.combo(operand) % 8),
                6 => self.reg[1] = self.reg[0] / 2u64.pow(self.combo(operand) as u32),
                7 => self.reg[2] = self.reg[0] / 2u64.pow(self.combo(operand) as u32),
                _ => unreachable!("invalid instruction"),
            }
            self.ip += 2;
        }
    }
}

type Input = Computer;

pub fn input() -> Input {
    fn parse_reg(s: &str) -> u64 {
        s.split(" ").skip(2).next().unwrap().parse().unwrap()
    }
    let lines: Vec<String> = load("data/day17.txt");
    Computer {
        ip: 0,
        reg: [
            parse_reg(&lines[0]),
            parse_reg(&lines[1]),
            parse_reg(&lines[2]),
        ],
        input: lines[4]
            .split(" ")
            .skip(1)
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect(),
        output: vec![],
    }
}

pub fn part1(mut computer: Input) -> String {
    computer.run();
    computer.output()
}

/**
 *  0. (2,4) B = A % 8
 *  2. (1,5) B = B xor 5
 *  4. (7,5) C = A / 2^B
 *  6. (1,6) B = B xor 6
 *  8. (4,2) B = B xor C
 * 10. (5,5) output B % 8
 * 12. (0,3) A = A / 2^3
 * 14  (3,0) if A != 0 goto 0
 *
 * A = A / 8
 * B = (A % 8 xor 5 xor 6) xor (A / 2^(A % 8 xor 5))
 * C = A / 2^(A % 8 xor 5)
 */
pub fn part2(mut computer: Input) -> u64 {
    fn do_try(computer: &mut Computer, a: u64) {
        computer.init(a);
        computer.run();
    }
    let expected = [2, 4, 1, 5, 7, 5, 1, 6, 4, 2, 5, 5, 0, 3, 3, 0];
    let mut a = 0;
    for i in (0..expected.len()).rev() {
        loop {
            do_try(&mut computer, a);
            if computer.output == expected[i..] {
                a *= 8;
                break;
            }
            a += 1;
        }
    }
    a / 8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), "2,7,6,5,6,0,2,3,1");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 107416870455451);
    }
}

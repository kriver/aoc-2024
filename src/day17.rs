use crate::util::load;

#[derive(Debug)]
pub struct Computer {
    ip: usize,
    reg: [u32; 3],
    input: Vec<u32>,
    output: Vec<u32>,
}

impl Computer {
    fn combo(&self, v: u32) -> u32 {
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
    fn run(&mut self) {
        loop {
            if self.ip >= self.input.len() {
                break;
            }
            let instr = self.input[self.ip];
            let operand = self.input[self.ip + 1];
            match instr {
                0 => self.reg[0] /= 2u32.pow(self.combo(operand)),
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
                6 => self.reg[1] = self.reg[0] / 2u32.pow(self.combo(operand)),
                7 => self.reg[2] = self.reg[0] / 2u32.pow(self.combo(operand)),
                _ => unreachable!("invalid instruction"),
            }
            self.ip += 2;
        }
    }
}

type Input = Computer;

pub fn input() -> Input {
    fn parse_reg(s: &str) -> u32 {
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
    computer
        .output
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn part2(values: Input) -> u32 {
    0
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
        assert_eq!(part2(input()), 0);
    }
}

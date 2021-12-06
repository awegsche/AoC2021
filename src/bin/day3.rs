use aoc::day::Day;
use std::{fmt::Display, str::FromStr};

const NAME: &str = "day3";

pub struct Day3 {
    pub codes: Vec<ByteCode>,
    pub len: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct ByteCode {
    pub data: u32,
    pub len: u8,
}

impl FromStr for ByteCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = 0;
        let len = s.len();
        for (i, c) in s.chars().enumerate() {
            if c == '1' {
                data |= 1 << (len - i - 1);
            }
        }
        Ok(Self {
            data,
            len: len as u8,
        })
    }
}

impl Display for ByteCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#b}", self.data)
    }
}

impl Day<i32> for Day3 {
    fn part1(&mut self) -> Result<i32, aoc::AocError> {
        let mut epsilon = 0;
        let mut gamma = 0;
        let number = (self.codes.len() / 2) as u32;
        println!("number: {}", number);
        for index in 0..self.len {
            let count: u32 = self.codes.iter().map(|b| b.data >> index & 1).sum();
            if count >= number {
                epsilon |= 1 << index;
            } else {
                gamma |= 1 << index;
            }
        }

        println!("epsilon: {:#b}\ngamma: {:#b}", epsilon, gamma);
        Ok(epsilon * gamma)
    }

    fn part2(&mut self) -> Result<i32, aoc::AocError> {
        let mut index = self.len;

        let mut current = self.codes.to_vec();
        while current.len() > 1 {
            index -= 1;
            let mut ones = Vec::with_capacity(self.codes.len());
            let mut zeros = Vec::with_capacity(self.codes.len());
            for b in current.iter() {
                if b.data >> index & 1 == 1 {
                    ones.push(b.clone());
                } else {
                    zeros.push(b.clone());
                }
            }
            if zeros.len() > ones.len() {
                current = zeros;
            } else {
                current = ones
            }
            //print!("current: [");
            //for b in current.iter() {
            //    print!("{}, ", b);
            //}
            //println!("]");
            //println!("index: {}", index);
        }

        let oxygen = current.first().unwrap().data as i32;

        current = self.codes.to_vec();
        index = self.len;
        while current.len() > 1 {
            index -= 1;
            let mut ones = Vec::with_capacity(self.codes.len());
            let mut zeros = Vec::with_capacity(self.codes.len());
            for b in current.iter() {
                if b.data >> index & 1 == 1 {
                    ones.push(b.clone());
                } else {
                    zeros.push(b.clone());
                }
            }
            if zeros.len() <= ones.len() {
                current = zeros;
            } else {
                current = ones
            }
            //print!("current: [");
            //for b in current.iter() {
            //    print!("{}, ", b);
            //}
            //println!("]");
            //println!("index: {}", index);
        }

        let co2 = current.first().unwrap().data as i32;

        Ok(oxygen*co2)
    }

    fn parse_input(filename: &str) -> Result<Self, aoc::AocError> {
        let codes: Vec<ByteCode> = aoc::parse_lines(filename)?.collect();
        let len = codes.first().unwrap().len;
        Ok(Self { codes, len })
    }

    fn name() -> &'static str {
        NAME
    }
}

fn main() {
    Day3::run().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        assert_eq!(
            Day3 {
                codes: [
                    "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100",
                    "10000", "11001", "00010", "01010",
                ]
                .iter()
                .map(|s| s.parse().unwrap())
                .collect(),
                len: 5
            }
            .part1()
            .unwrap(),
            198
        )
    }

    #[test]
    fn part2() {
        assert_eq!(
            Day3 {
                codes: [
                    "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100",
                    "10000", "11001", "00010", "01010",
                ]
                .iter()
                .map(|s| s.parse().unwrap())
                .collect(),
                len: 5
            }
            .part2()
            .unwrap(),
            230
        )
    }
}

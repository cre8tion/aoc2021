use aoc_runner_derive::{aoc, aoc_generator};
use itertools::join;

#[derive(Debug)]
pub enum Binary {
    Zero,
    One,
}

impl std::fmt::Display for Binary {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match *self {
            Binary::Zero => write!(fmt, "0"),
            Binary::One => write!(fmt, "1"),
        }
    }
}

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Vec<Binary>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|val| {
                    let num = val.to_digit(2).unwrap();
                    match num {
                        0 => Binary::Zero,
                        _ => Binary::One,
                    }
                })
                .collect::<Vec<Binary>>()
        })
        .collect::<Vec<Vec<Binary>>>()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Vec<Binary>]) -> usize {
    let gamma_rate_vec = input
        .iter()
        .fold(vec![0 as i32; 12], |acc, vec| {
            acc.iter()
                .enumerate()
                .map(|(i, _val)| match vec[i] {
                    Binary::Zero => _val - 1,
                    Binary::One => _val + 1,
                })
                .collect::<Vec<i32>>()
        })
        .iter()
        .map(|val| if val > &0 { Binary::One } else { Binary::Zero })
        .collect::<Vec<Binary>>();

    let epsilon_rate_vec = gamma_rate_vec
        .iter()
        .map(|b| match b {
            Binary::Zero => Binary::One,
            Binary::One => Binary::Zero,
        })
        .collect::<Vec<Binary>>();

    let gamma_rate = usize::from_str_radix(&join(gamma_rate_vec, ""), 2).unwrap();
    let epsilon_rate = usize::from_str_radix(&join(epsilon_rate_vec, ""), 2).unwrap();

    gamma_rate * epsilon_rate
}

#[aoc(day3, part2)]
pub fn part2(input: &[Vec<Binary>]) -> u32 {
    let input_vec = input.iter().map(|b| join(b, "")).collect::<Vec<String>>();
    part2_solver(&input_vec, true) * part2_solver(&input_vec, false)
}

fn part2_solver(lines: &Vec<String>, most: bool) -> u32 {
    let mut pos = 0;
    let mut values = lines.clone();
    while values.len() > 1 {
        let ones_count = values
            .iter()
            .map(|x| x.chars().nth(pos).unwrap())
            .filter(|x| *x == '1')
            .count();
        if ones_count >= (values.len() - ones_count) {
            values.retain(|x| x.chars().nth(pos).unwrap() == if most { '1' } else { '0' })
        } else {
            values.retain(|x| x.chars().nth(pos).unwrap() == if most { '0' } else { '1' })
        }
        pos += 1
    }
    u32::from_str_radix(&values[0], 2).unwrap()
}

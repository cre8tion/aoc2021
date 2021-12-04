use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|l| {
            {
                l.trim()
                    .split_once(' ')
                    .map(|(c, val)| {
                        let val = val.parse::<u32>().unwrap();
                        match c {
                            "forward" => Ok(Command::Forward(val)),
                            "down" => Ok(Command::Down(val)),
                            "up" => Ok(Command::Up(val)),
                            _ => Err(()),
                        }
                    })
                    .unwrap()
            }
            .unwrap()
        })
        .collect::<Vec<Command>>()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Command]) -> u32 {
    let (fp, fd) = input
        .iter()
        .fold((0, 0), |(position, depth), command| match command {
            Command::Forward(amt) => (position + amt, depth),
            Command::Down(amt) => (position, depth + amt),
            Command::Up(amt) => (position, depth - amt),
        });

    fp * fd
}

#[aoc(day2, part2)]
pub fn part2(input: &[Command]) -> u32 {
    let (fp, fd, _fa) =
        input
            .iter()
            .fold((0, 0, 0), |(position, depth, aim), command| match command {
                Command::Forward(amt) => (position + *amt, depth + aim * *amt, aim),
                Command::Down(amt) => (position, depth, aim + *amt),
                Command::Up(amt) => (position, depth, aim - *amt),
            });

    fp * fd
}

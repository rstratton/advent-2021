use std::fs;

struct Submarine1 {
    horizontal: u32,
    depth: u32,
}

impl Submarine1 {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
        }
    }

    fn execute(&mut self, command: &Command) {
        match command {
            Command::Forward(amount) => self.horizontal += amount,
            Command::Down(amount) => self.depth += amount,
            Command::Up(amount) => self.depth -= amount,
        }
    }
}

struct Submarine2 {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

impl Submarine2 {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn execute(&mut self, command: &Command) {
        match command {
            Command::Forward(amount) => {
                self.horizontal += amount;
                self.depth += self.aim * amount;
            }
            Command::Down(amount) => self.aim += amount,
            Command::Up(amount) => self.aim -= amount,
        }
    }
}

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl From<&str> for Command {
    fn from(string: &str) -> Self {
        let parts: Vec<&str> = string.split(' ').collect();
        let command_type = parts[0].trim();
        let amount: u32 = parts[1].trim().parse().unwrap();
        match command_type {
            "forward" => Command::Forward(amount),
            "down" => Command::Down(amount),
            "up" => Command::Up(amount),
            _ => panic!("Unknown command"),
        }
    }
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn read_input() -> Vec<Command> {
    let commands = fs::read_to_string("data/day_02.txt").expect("File missing");
    commands.split('\n').map(|s| s.into()).collect()
}

fn part1() -> u32 {
    let commands = read_input();
    let mut sub = Submarine1::new();
    for command in commands {
        sub.execute(&command);
    }
    sub.horizontal * sub.depth
}

fn part2() -> u32 {
    let commands = read_input();
    let mut sub = Submarine2::new();
    for command in commands {
        sub.execute(&command);
    }
    sub.horizontal * sub.depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part1(), 1_938_402);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(), 1_947_878_632);
    }
}

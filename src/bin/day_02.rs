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
    let commands = fs::read_to_string("data/day_02.txt").expect("File missing");
    let commands: Vec<Command> = commands.split('\n').map(|s| s.into()).collect();
    println!("{}", part1(&commands));
    println!("{}", part2(&commands));
}

fn part1(commands: &[Command]) -> u32 {
    let mut sub = Submarine1::new();
    for command in commands {
        sub.execute(command);
    }
    sub.horizontal * sub.depth
}

fn part2(commands: &[Command]) -> u32 {
    let mut sub = Submarine2::new();
    for command in commands {
        sub.execute(command);
    }
    sub.horizontal * sub.depth
}

use std::collections::HashSet;
use std::fs;

struct EnergyLevels {
    width: usize,
    height: usize,
    energy: Vec<u32>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn neighbors(&self) -> Vec<Position> {
        vec![
            Position::new(self.x - 1, self.y),
            Position::new(self.x + 1, self.y),
            Position::new(self.x, self.y - 1),
            Position::new(self.x, self.y + 1),
            Position::new(self.x - 1, self.y - 1),
            Position::new(self.x - 1, self.y + 1),
            Position::new(self.x + 1, self.y - 1),
            Position::new(self.x + 1, self.y + 1),
        ]
    }
}

impl EnergyLevels {
    fn positions(&self) -> Vec<Position> {
        let mut result = Vec::new();
        for i in 0..self.width {
            for j in 0..self.height {
                result.push(Position::new(i as i32, j as i32));
            }
        }
        result
    }

    fn position_to_index(&self, p: &Position) -> Option<usize> {
        if p.x < 0 || p.y < 0 || p.x >= self.width as i32 || p.y >= self.height as i32 {
            None
        } else {
            Some((p.x + (self.width as i32 * p.y)) as usize)
        }
    }

    fn energy(&self, p: &Position) -> Option<u32> {
        self.position_to_index(p).map(|idx| self.energy[idx])
    }

    fn incr_energy(&mut self, p: &Position) {
        if let Some(idx) = self.position_to_index(p) {
            self.energy[idx] += 1
        }
    }

    fn step(&mut self) -> u32 {
        for energy in self.energy.iter_mut() {
            *energy += 1;
        }

        let mut flashed = HashSet::new();
        loop {
            let mut flash_occurred = false;

            for position in self.positions() {
                if self.energy(&position).unwrap() > 9 && !flashed.contains(&position) {
                    flash_occurred = true;
                    flashed.insert(position);
                    for neighbor in position.neighbors() {
                        self.incr_energy(&neighbor)
                    }
                }
            }

            if !flash_occurred {
                break;
            }
        }

        for energy in self.energy.iter_mut() {
            if *energy > 9 {
                *energy = 0;
            }
        }

        flashed.len() as u32
    }
}

impl From<&str> for EnergyLevels {
    fn from(s: &str) -> Self {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        EnergyLevels {
            width,
            height,
            energy: s.chars().flat_map(|c| c.to_digit(10)).collect(),
        }
    }
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn read_input() -> EnergyLevels {
    EnergyLevels::from(&fs::read_to_string("data/day_11.txt").expect("File missing")[..])
}

fn part1() -> u32 {
    let mut energy_levels = read_input();
    let mut flash_count = 0;
    for _ in 0..100 {
        flash_count += energy_levels.step();
    }
    flash_count
}

fn part2() -> u32 {
    let mut energy_levels = read_input();

    for step_number in 1.. {
        let flash_count = energy_levels.step();
        if flash_count == (energy_levels.width * energy_levels.height) as u32 {
            return step_number;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1546);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 471);
    }
}

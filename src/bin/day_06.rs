use cached::proc_macro::cached;
use std::fs;

struct Fish {
    days_until_spawn: u8,
}

impl Fish {
    fn new(days_until_spawn: u8) -> Self {
        Self { days_until_spawn }
    }

    fn advance_day(&mut self) -> Option<Fish> {
        if self.days_until_spawn == 0 {
            self.days_until_spawn = 6;
            Some(Fish::new(8))
        } else {
            self.days_until_spawn -= 1;
            None
        }
    }
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn read_input() -> Vec<Fish> {
    let input = fs::read_to_string("data/day_06.txt").expect("File missing");
    input
        .split(',')
        .map(|num| Fish::new(num.trim().parse().unwrap()))
        .collect()
}

fn part1() -> usize {
    let mut fishes = read_input();
    for _ in 0..80 {
        let mut new_fishes = Vec::new();
        for fish in fishes.iter_mut() {
            if let Some(new_fish) = fish.advance_day() {
                new_fishes.push(new_fish);
            }
        }
        fishes.append(&mut new_fishes);
    }
    fishes.len()
}

fn part2() -> u64 {
    let fishes = read_input();
    let mut num_fish_after_256_days = 0u64;
    for fish in fishes {
        num_fish_after_256_days += family_size_after_days(fish.days_until_spawn as u64, 256);
    }
    num_fish_after_256_days
}

#[cached]
fn family_size_after_days(spawn_timer: u64, days: u64) -> u64 {
    if days == 0 {
        1
    } else if spawn_timer == 0 {
        family_size_after_days(6, days - 1) + family_size_after_days(8, days - 1)
    } else {
        family_size_after_days(spawn_timer - 1, days - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 362_740);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1_644_874_076_764);
    }
}

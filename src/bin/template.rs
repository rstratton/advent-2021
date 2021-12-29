use std::fs;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn read_input() -> () {
    let input = fs::read_to_string("data/day_01.txt").expect("File missing");
}

fn part1() -> u32 {
    let input = read_input();
    0
}

fn part2() -> u32 {
    let input = read_input();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0);
    }
}

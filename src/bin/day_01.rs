use std::fs;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn read_input() -> Vec<u32> {
    let depths = fs::read_to_string("data/day_01.txt").expect("File missing");
    depths
        .split('\n')
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}

fn part1() -> usize {
    let depths = read_input();
    depths
        .windows(2)
        .filter(|&window| window[0] < window[1])
        .count()
}

fn part2() -> usize {
    let depths = read_input();
    let summed_depths: Vec<u32> = depths
        .windows(3)
        .map(|window| window[0] + window[1] + window[2])
        .collect();

    summed_depths
        .windows(2)
        .filter(|&window| window[0] < window[1])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part1(), 1482);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(), 1518);
    }
}

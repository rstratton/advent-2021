use std::fs;

fn main() {
    let depths = fs::read_to_string("data/day_01.txt").expect("File missing");
    let depths: Vec<u32> = depths
        .split('\n')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    println!("{}", part1(&depths));
    println!("{}", part2(&depths));
}

fn part1(depths: &[u32]) -> usize {
    depths
        .windows(2)
        .filter(|&window| window[0] < window[1])
        .count()
}

fn part2(depths: &[u32]) -> usize {
    let summed_depths: Vec<u32> = depths
        .windows(3)
        .map(|window| window[0] + window[1] + window[2])
        .collect();

    summed_depths
        .windows(2)
        .filter(|&window| window[0] < window[1])
        .count()
}

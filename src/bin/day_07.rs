use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;
use std::fs;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn read_input() -> Vec<u32> {
    let input = fs::read_to_string("data/day_07.txt").expect("File missing");
    input
        .split(',')
        .map(|num| num.trim().parse().unwrap())
        .collect()
}

fn part1() -> u32 {
    let positions = read_input();
    let alignment_target = median(&positions);
    fuel_cost_1(&positions, alignment_target)
}

fn part2() -> u32 {
    let positions = read_input();
    // I don't fully understand WHY the average of the positions yields the
    // lowest cost (whereas I have a decent understanding as to why this is
    // the case for the median in part 1).  My intuition told me this might
    // work and it turns out it does.
    let alignment_target = average(&positions);
    fuel_cost_2(&positions, alignment_target)
}

fn fuel_cost_1(positions: &[u32], alignment_target: u32) -> u32 {
    positions
        .iter()
        .map(|p| {
            if *p < alignment_target {
                alignment_target - *p
            } else {
                *p - alignment_target
            }
        })
        .sum()
}

fn fuel_cost_2(positions: &[u32], alignment_target: u32) -> u32 {
    positions
        .iter()
        .map(|p| {
            if *p < alignment_target {
                alignment_target - *p
            } else {
                *p - alignment_target
            }
        })
        .map(|distance| (distance * (distance + 1)) / 2)
        .sum()
}

fn average(values: &[u32]) -> u32 {
    values.iter().sum::<u32>() / values.len() as u32
}

// Linear time median finding algo taken from https://rcoh.me/posts/linear-time-median-finding/
fn median(values: &[u32]) -> u32 {
    if values.len() % 2 == 1 {
        quickselect(values, values.len() / 2)
    } else {
        (quickselect(values, (values.len() / 2) - 1) + quickselect(values, values.len() / 2)) / 2
    }
}

fn quickselect(values: &[u32], idx: usize) -> u32 {
    if values.len() == 1 {
        return values[0];
    }

    let pivot = pivot(values);

    let mut lows = Vec::new();
    let mut pivots = Vec::new();
    let mut highs = Vec::new();
    for v in values {
        match v.cmp(&pivot) {
            Ordering::Less => lows.push(*v),
            Ordering::Equal => pivots.push(*v),
            Ordering::Greater => highs.push(*v),
        }
    }

    if idx < lows.len() {
        quickselect(&lows, idx)
    } else if idx < lows.len() + pivots.len() {
        pivots[0]
    } else {
        quickselect(&highs, idx - (lows.len() + pivots.len()))
    }
}

fn pivot(values: &[u32]) -> u32 {
    *values.choose(&mut thread_rng()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 356958);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 105461913);
    }
}

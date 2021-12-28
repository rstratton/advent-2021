use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

struct HeightMap {
    width: usize,
    height: usize,
    values: Vec<u32>,
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
        ]
    }
}

impl HeightMap {
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

    fn height(&self, p: &Position) -> Option<u32> {
        self.position_to_index(p).map(|idx| self.values[idx])
    }

    fn is_low_point(&self, p: &Position) -> bool {
        let p_height = self.height(p);
        match p_height {
            Some(p_height) => p
                .neighbors()
                .into_iter()
                .flat_map(|neighbor| self.height(&neighbor))
                .all(|neighbor_height| p_height < neighbor_height),
            None => false,
        }
    }
}

impl From<&str> for HeightMap {
    fn from(s: &str) -> Self {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        HeightMap {
            width,
            height,
            values: s.chars().flat_map(|c| c.to_digit(10)).collect(),
        }
    }
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn read_input() -> HeightMap {
    let input = fs::read_to_string("data/day_09.txt").expect("File missing");
    HeightMap::from(&input[..])
}

fn part1() -> u32 {
    let height_map = read_input();
    let mut sum_of_risk_levels = 0;
    for position in height_map.positions() {
        if height_map.is_low_point(&position) {
            sum_of_risk_levels += height_map.height(&position).unwrap() + 1;
        }
    }
    sum_of_risk_levels
}

fn part2() -> u32 {
    let height_map = read_input();
    let mut position_to_basin_idx: HashMap<Position, usize> = HashMap::new();
    let mut basin_sizes: Vec<u32> = Vec::new();
    for position in height_map.positions() {
        if position_to_basin_idx.contains_key(&position)
            || height_map.height(&position).unwrap() == 9
        {
            continue;
        }

        let basin_idx = basin_sizes.len();
        basin_sizes.push(0);
        let mut to_visit: VecDeque<Position> = VecDeque::from([position]);

        while let Some(position) = to_visit.pop_front() {
            if position_to_basin_idx.contains_key(&position)
                || height_map.height(&position).unwrap_or(9) == 9
            {
                continue;
            }

            position_to_basin_idx.insert(position, basin_idx);
            basin_sizes[basin_idx] += 1;

            for neighbor in position.neighbors() {
                to_visit.push_back(neighbor);
            }
        }
    }

    basin_sizes.sort();

    basin_sizes.iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 514);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1103130);
    }
}

use itertools::Itertools;
use std::fs;

#[derive(Debug)]
struct Board {
    values: [u32; 25],
    markings: [Marking; 25],
}

#[derive(Debug, Copy, Clone)]
enum Marking {
    Unmarked,
    Marked,
}

impl Marking {
    fn is_marked(&self) -> bool {
        match *self {
            Marking::Marked => true,
            Marking::Unmarked => false,
        }
    }
}

struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        assert!(row < 5);
        assert!(col < 5);
        Self { row, col }
    }

    fn to_idx(&self) -> usize {
        self.row * 5 + self.col
    }
}

impl Board {
    fn new(values: &[u32]) -> Self {
        let mut result = Board {
            values: Default::default(),
            markings: [Marking::Unmarked; 25],
        };
        result.values.copy_from_slice(values);
        result
    }

    fn mark(&mut self, value: u32) {
        for (idx, v) in self.values.iter().enumerate() {
            if *v == value {
                self.markings[idx] = Marking::Marked
            }
        }
    }

    fn has_marked_all(&self, positions: &[Position]) -> bool {
        positions
            .iter()
            .all(|p| self.markings[p.to_idx()].is_marked())
    }

    fn is_winner(&self) -> bool {
        for i in 0..5 {
            let horizontal_positions: Vec<Position> = (0..5).map(|j| Position::new(i, j)).collect();
            if self.has_marked_all(&horizontal_positions) {
                return true;
            }
            let vertical_positions: Vec<Position> = (0..5).map(|j| Position::new(j, i)).collect();
            if self.has_marked_all(&vertical_positions) {
                return true;
            }
        }
        false
    }

    fn sum_of_unmarked(&self) -> u32 {
        let mut sum = 0;
        for idx in 0..25 {
            if let Marking::Unmarked = self.markings[idx] {
                sum += self.values[idx]
            }
        }
        sum
    }
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn read_input() -> (Vec<u32>, Vec<Board>) {
    let input = fs::read_to_string("data/day_04.txt").expect("File missing");
    let mut lines = input.lines();

    let numbers = lines.next().unwrap();
    let numbers: Vec<u32> = numbers
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let board_numbers: Vec<u32> = lines
        .filter(|&s| !s.trim().is_empty())
        .join(" ")
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let boards = board_numbers
        .chunks(25)
        .map(|chunk| Board::new(chunk))
        .collect();

    (numbers, boards)
}

fn part1() -> u32 {
    let (numbers, mut boards) = read_input();
    for number in numbers {
        for board in boards.iter_mut() {
            board.mark(number);
            if board.is_winner() {
                return board.sum_of_unmarked() * number;
            }
        }
    }
    panic!("No solution found");
}

fn part2() -> u32 {
    let (numbers, mut boards) = read_input();
    for number in numbers {
        if boards.len() == 1 {
            let board = boards.first_mut().unwrap();
            board.mark(number);
            if board.is_winner() {
                return board.sum_of_unmarked() * number;
            }
        } else {
            for board in boards.iter_mut() {
                board.mark(number);
            }
            boards.retain(|board| !board.is_winner());
        }
    }
    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 8136);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 12738);
    }
}

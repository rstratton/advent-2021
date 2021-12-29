use std::fs;

use itertools::Chunk;
#[derive(Clone, Copy, PartialEq, Eq)]
enum ChunkType {
    Paren,
    Square,
    Curly,
    Angle,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ChunkBound {
    Begin,
    End,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct ChunkDelim {
    chunk_type: ChunkType,
    chunk_bound: ChunkBound,
}

impl ChunkDelim {
    fn new(chunk_type: ChunkType, chunk_bound: ChunkBound) -> Self {
        Self {
            chunk_type,
            chunk_bound,
        }
    }

    fn complement(&self) -> Self {
        Self {
            chunk_type: self.chunk_type,
            chunk_bound: match self.chunk_bound {
                ChunkBound::Begin => ChunkBound::End,
                ChunkBound::End => ChunkBound::Begin,
            },
        }
    }
}

impl From<char> for ChunkDelim {
    fn from(c: char) -> Self {
        match c {
            '(' => ChunkDelim::new(ChunkType::Paren, ChunkBound::Begin),
            '[' => ChunkDelim::new(ChunkType::Square, ChunkBound::Begin),
            '{' => ChunkDelim::new(ChunkType::Curly, ChunkBound::Begin),
            '<' => ChunkDelim::new(ChunkType::Angle, ChunkBound::Begin),
            ')' => ChunkDelim::new(ChunkType::Paren, ChunkBound::End),
            ']' => ChunkDelim::new(ChunkType::Square, ChunkBound::End),
            '}' => ChunkDelim::new(ChunkType::Curly, ChunkBound::End),
            '>' => ChunkDelim::new(ChunkType::Angle, ChunkBound::End),
            _ => panic!("Illegal char {}", c),
        }
    }
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn read_input() -> Vec<Vec<ChunkDelim>> {
    fs::read_to_string("data/day_10.txt")
        .expect("File missing")
        .lines()
        .map(|s| s.chars().map(|c| c.into()).collect())
        .collect()
}

fn part1() -> u32 {
    let input = read_input();

    input
        .iter()
        .flat_map(|line| first_illegal_chunk_delim(line).map(score1))
        .sum()
}

fn first_illegal_chunk_delim(delims: &[ChunkDelim]) -> Option<ChunkDelim> {
    let mut delim_stack: Vec<ChunkDelim> = Vec::new();

    for delim in delims {
        match delim.chunk_bound {
            ChunkBound::Begin => delim_stack.push(*delim),
            ChunkBound::End => {
                let last_begin = delim_stack.pop().unwrap();
                if last_begin.complement() != *delim {
                    return Some(*delim);
                }
            }
        }
    }

    None
}

fn score1(d: ChunkDelim) -> u32 {
    match d.chunk_type {
        ChunkType::Paren => 3,
        ChunkType::Square => 57,
        ChunkType::Curly => 1197,
        ChunkType::Angle => 25137,
    }
}

fn part2() -> u64 {
    let input = read_input();
    let mut scores: Vec<u64> = input
        .iter()
        .flat_map(|line| close_chunks(line))
        .map(|closing_delims| score2(&closing_delims))
        .collect();
    scores.sort();
    scores[scores.len() / 2]
}

fn close_chunks(delims: &[ChunkDelim]) -> Option<Vec<ChunkDelim>> {
    let mut delim_stack = Vec::new();

    for delim in delims {
        match delim.chunk_bound {
            ChunkBound::Begin => delim_stack.push(*delim),
            ChunkBound::End => {
                let last_begin = delim_stack.pop().unwrap();
                if last_begin.complement() != *delim {
                    return None;
                }
            }
        }
    }

    Some(
        delim_stack
            .iter()
            .rev()
            .map(|delim| delim.complement())
            .collect(),
    )
}

fn score2(delims: &[ChunkDelim]) -> u64 {
    let mut score = 0;

    for delim in delims {
        score *= 5;
        score += match delim.chunk_type {
            ChunkType::Paren => 1,
            ChunkType::Square => 2,
            ChunkType::Curly => 3,
            ChunkType::Angle => 4,
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 392043);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1605968119);
    }
}

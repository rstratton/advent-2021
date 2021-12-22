use std::fs;
use std::ops::{BitAnd, BitOr, Not, Sub};

struct Input {
    signal_bundles: Vec<SignalBundle>,
    output_value: Vec<SignalBundle>,
}

enum Signal {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl From<char> for Signal {
    fn from(c: char) -> Self {
        match c {
            'a' => Signal::A,
            'b' => Signal::B,
            'c' => Signal::C,
            'd' => Signal::D,
            'e' => Signal::E,
            'f' => Signal::F,
            'g' => Signal::G,
            c => panic!("Received unparseable signal char {}", c),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct SignalBundle {
    bits: u8,
}

impl SignalBundle {
    fn new() -> Self {
        Self { bits: 0 }
    }

    fn bit(s: Signal) -> u8 {
        match s {
            Signal::A => 1,
            Signal::B => 1 << 1,
            Signal::C => 1 << 2,
            Signal::D => 1 << 3,
            Signal::E => 1 << 4,
            Signal::F => 1 << 5,
            Signal::G => 1 << 6,
        }
    }

    fn bits_set(&self) -> u32 {
        self.bits.count_ones()
    }
}

impl BitAnd for SignalBundle {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        SignalBundle {
            bits: self.bits & rhs.bits,
        }
    }
}

impl BitOr for SignalBundle {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        SignalBundle {
            bits: self.bits | rhs.bits,
        }
    }
}

impl Not for SignalBundle {
    type Output = Self;

    fn not(self) -> Self::Output {
        SignalBundle {
            bits: (!self.bits) & 0b0111_1111,
        }
    }
}

impl Sub for SignalBundle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self & !rhs
    }
}

impl From<&str> for SignalBundle {
    fn from(s: &str) -> Self {
        let mut result: u8 = 0;

        for signal in s.chars().map(|c| c.into()) {
            result |= SignalBundle::bit(signal)
        }

        SignalBundle { bits: result }
    }
}

impl From<&str> for Input {
    fn from(s: &str) -> Input {
        let sections: Vec<&str> = s.split('|').collect();
        Input {
            signal_bundles: sections[0].split(' ').map(|s| s.trim().into()).collect(),
            output_value: sections[1].split(' ').map(|s| s.trim().into()).collect(),
        }
    }
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn read_input() -> Vec<Input> {
    let input = fs::read_to_string("data/day_08.txt").expect("File missing");
    input.lines().map(|line| line.into()).collect()
}

fn part1() -> u32 {
    let input = read_input();
    let mut total = 0;
    for i in input {
        for signals in i.output_value {
            match signals.bits_set() {
                2 | 3 | 4 | 7 => total += 1,
                _ => {}
            }
        }
    }
    total
}

fn part2() -> u32 {
    let input = read_input();
    let mut sum_of_output_values = 0;

    for i in input {
        let mut mapping = [SignalBundle::new(); 10];
        infer_mapping(&i.signal_bundles, &mut mapping);
        sum_of_output_values += interpret_output_value(&i.output_value, &mapping);
    }

    sum_of_output_values
}

fn infer_mapping(signal_bundles: &[SignalBundle], mapping: &mut [SignalBundle; 10]) {
    // As stated in the problem, we know which signal bundles map to digits 1, 4, 7, and 8
    // due to each having a unique number of bits set.
    mapping[1] = *signal_bundles.iter().find(|&s| s.bits_set() == 2).unwrap();
    mapping[4] = *signal_bundles.iter().find(|&s| s.bits_set() == 4).unwrap();
    mapping[7] = *signal_bundles.iter().find(|&s| s.bits_set() == 3).unwrap();
    mapping[8] = *signal_bundles.iter().find(|&s| s.bits_set() == 7).unwrap();
    let intersection235 = signal_bundles
        .iter()
        .cloned()
        .filter(|s| s.bits_set() == 5)
        .reduce(|a, b| a & b)
        .unwrap();
    let intersection069 = signal_bundles
        .iter()
        .cloned()
        .filter(|s| s.bits_set() == 6)
        .reduce(|a, b| a & b)
        .unwrap();
    // These single-letter variables each represet a single segment of the display.  Each SignalBundle
    // should have exactly 1 bit set.  We deduce which signals correspond to which display segments
    // here.
    let a = mapping[7] - mapping[1];
    let g = intersection235 - (mapping[4] | mapping[7]);
    let b = (mapping[4] - mapping[1]) & intersection069;
    let f = mapping[1] & intersection069;
    let d = mapping[4] - mapping[1] - b;
    let c = mapping[1] - f;
    let e = mapping[8] - (intersection235 | intersection069) - c;
    // Use our knowledge of segment mappings to construct the remaining digit <> bundle mappings.
    mapping[0] = mapping[8] - d;
    mapping[2] = mapping[8] - b - f;
    mapping[3] = mapping[8] - b - e;
    mapping[5] = mapping[8] - c - e;
    mapping[6] = mapping[8] - c;
    mapping[9] = mapping[8] - e;
}

fn interpret_output_value(output_value: &[SignalBundle], mapping: &[SignalBundle; 10]) -> u32 {
    let mut result = 0;

    for (idx, output_digit) in output_value.iter().rev().enumerate() {
        for (value, signal_bundle) in mapping.iter().enumerate() {
            if output_digit == signal_bundle {
                result += value as u32 * 10u32.pow(idx as u32);
                continue;
            }
        }
    }

    result
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
        assert_eq!(part2(), 1012272);
    }
}

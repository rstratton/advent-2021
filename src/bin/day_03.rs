use std::{fs, ops::Index};

#[derive(PartialEq, Debug)]
enum Bit {
    Zero,
    One,
}

#[derive(Clone)]
struct BinNum(u32);

impl BinNum {
    fn bits(&self) -> Bits {
        Bits {
            value: self,
            next_bit_idx: 0,
        }
    }

    fn value(&self) -> u32 {
        self.0
    }

    fn bit(&self, bit_idx: usize) -> Bit {
        if self.value() & ((1 << 11) >> bit_idx) > 0 {
            Bit::One
        } else {
            Bit::Zero
        }
    }
}

impl From<&str> for BinNum {
    fn from(s: &str) -> Self {
        BinNum(u32::from_str_radix(s, 2).unwrap())
    }
}

struct Bits<'a> {
    value: &'a BinNum,
    next_bit_idx: usize,
}

impl<'a> Iterator for Bits<'a> {
    type Item = Bit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_bit_idx >= 12 {
            return None;
        }
        let result = self.value.bit(self.next_bit_idx);
        self.next_bit_idx += 1;
        Some(result)
    }
}

#[derive(Clone, Copy)]
struct DigitFrequency {
    zeroes: u32,
    ones: u32,
}

impl DigitFrequency {
    fn new() -> Self {
        Self { zeroes: 0, ones: 0 }
    }

    fn incr(&mut self, value: Bit) {
        match value {
            Bit::Zero => self.zeroes += 1,
            Bit::One => self.ones += 1,
        }
    }
}

impl Index<Bit> for DigitFrequency {
    type Output = u32;

    fn index(&self, value: Bit) -> &Self::Output {
        match value {
            Bit::Zero => &self.zeroes,
            Bit::One => &self.ones,
        }
    }
}

struct DigitCounter {
    frequencies: [DigitFrequency; 12],
    total: u32,
}

impl DigitCounter {
    fn count(bin_nums: &[BinNum]) -> Self {
        let mut counter = Self {
            frequencies: [DigitFrequency::new(); 12],
            total: 0,
        };

        for bin_num in bin_nums {
            for (idx, value) in bin_num.bits().enumerate() {
                counter.frequencies[idx].incr(value);
                counter.total += 1;
            }
        }

        counter
    }
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn read_input() -> Vec<BinNum> {
    let bit_strings = fs::read_to_string("data/day_03.txt").expect("File missing");
    bit_strings.lines().map(|line| line.into()).collect()
}

fn part1() -> u32 {
    let numbers = read_input();
    let counter = DigitCounter::count(&numbers);
    let mut gamma = 0u32;
    let mut epsilon = 0u32;
    for (idx, freq) in counter.frequencies.iter().rev().enumerate() {
        let digit_value = 2u32.pow(idx as u32);
        if freq[Bit::One] > freq[Bit::Zero] {
            gamma += digit_value;
        } else {
            epsilon += digit_value;
        }
    }
    gamma * epsilon
}

fn part2() -> u32 {
    let numbers = read_input();
    let o2_rating = rating(&numbers, |freq| {
        if freq[Bit::One] >= freq[Bit::Zero] {
            Bit::One
        } else {
            Bit::Zero
        }
    });
    let co2_rating = rating(&numbers, |freq| {
        if freq[Bit::One] < freq[Bit::Zero] {
            Bit::One
        } else {
            Bit::Zero
        }
    });
    o2_rating * co2_rating
}

fn rating(numbers: &[BinNum], bit_selector: fn(DigitFrequency) -> Bit) -> u32 {
    let mut numbers = numbers.to_vec();
    for bit_idx in 0..12 {
        if numbers.len() == 1 {
            break;
        }

        let counter = DigitCounter::count(&numbers);
        let digit_frequency = counter.frequencies[bit_idx];
        let bit_value_to_keep = bit_selector(digit_frequency);

        numbers.retain(|n| n.bit(bit_idx) == bit_value_to_keep);
    }
    numbers.first().unwrap().value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin_num_bits() {
        let bin_num = BinNum::from("111100001111");
        let bits: Vec<Bit> = bin_num.bits().collect();
        assert_eq!(bits.len(), 12);
        assert_eq!(bits[0], Bit::One);
        assert_eq!(bits[1], Bit::One);
        assert_eq!(bits[2], Bit::One);
        assert_eq!(bits[3], Bit::One);
        assert_eq!(bits[4], Bit::Zero);
        assert_eq!(bits[5], Bit::Zero);
        assert_eq!(bits[6], Bit::Zero);
        assert_eq!(bits[7], Bit::Zero);
        assert_eq!(bits[8], Bit::One);
        assert_eq!(bits[9], Bit::One);
        assert_eq!(bits[10], Bit::One);
        assert_eq!(bits[11], Bit::One);
    }

    #[test]
    fn test_counter() {
        let bin_nums = vec![
            BinNum::from("000000000000"),
            BinNum::from("111100000000"),
            BinNum::from("110011001100"),
        ];
        let counter = DigitCounter::count(&bin_nums);
        assert_eq!(counter.frequencies[0][Bit::One], 2);
        assert_eq!(counter.frequencies[0][Bit::Zero], 1);
        assert_eq!(counter.frequencies[2][Bit::One], 1);
        assert_eq!(counter.frequencies[2][Bit::Zero], 2);
        assert_eq!(counter.frequencies[6][Bit::One], 0);
        assert_eq!(counter.frequencies[6][Bit::Zero], 3);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part1(), 2_640_986);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(), 6_822_109);
    }
}

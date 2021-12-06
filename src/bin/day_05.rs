use std::{cmp::Ordering, collections::HashMap, fs};

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Point {
        let mut coordinates = s.split(',');
        Point::new(
            coordinates.next().unwrap().trim().parse().unwrap(),
            coordinates.next().unwrap().trim().parse().unwrap(),
        )
    }
}

#[derive(Debug)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }

    fn is_diagonal(&self) -> bool {
        self.a.x != self.b.x && self.a.y != self.b.y
    }

    fn points(&self) -> Vec<Point> {
        let dx = match self.a.x.cmp(&self.b.x) {
            Ordering::Equal => 0,
            Ordering::Greater => -1,
            Ordering::Less => 1,
        };

        let dy = match self.a.y.cmp(&self.b.y) {
            Ordering::Equal => 0,
            Ordering::Greater => -1,
            Ordering::Less => 1,
        };

        let num_points = (self.a.x - self.b.x).abs().max((self.a.y - self.b.y).abs()) + 1;

        (0..num_points)
            .map(|n| Point::new(self.a.x + n * dx, self.a.y + n * dy))
            .collect()
    }
}

impl From<&str> for Line {
    fn from(s: &str) -> Line {
        let mut endpoints = s.split("->");
        Line::new(
            endpoints.next().unwrap().into(),
            endpoints.next().unwrap().into(),
        )
    }
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn read_input() -> Vec<Line> {
    let input = fs::read_to_string("data/day_05.txt").expect("File missing");
    input.lines().map(|l| l.into()).collect()
}

fn part1() -> usize {
    let lines = read_input();
    let mut counter = HashMap::new();

    for line in lines.into_iter().filter(|line| !line.is_diagonal()) {
        for point in line.points() {
            let count = counter.entry(point).or_insert(0u32);
            *count += 1
        }
    }

    counter.into_values().filter(|v| *v > 1).count()
}

fn part2() -> usize {
    let lines = read_input();
    let mut counter = HashMap::new();

    for line in lines {
        for point in line.points() {
            let count = counter.entry(point).or_insert(0u32);
            *count += 1
        }
    }

    counter.into_values().filter(|v| *v > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 5774);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 18423);
    }

    #[test]
    fn test_line_points() {
        let line = Line::from("10,20 -> 13,20");
        let points = line.points();
        assert_eq!(points.len(), 4);
        assert_eq!(points[0], Point::new(10, 20));
        assert_eq!(points[1], Point::new(11, 20));
        assert_eq!(points[2], Point::new(12, 20));
        assert_eq!(points[3], Point::new(13, 20));

        let line = Line::from("10,20 -> 7,17");
        let points = line.points();
        assert_eq!(points.len(), 4);
        assert_eq!(points[0], Point::new(10, 20));
        assert_eq!(points[1], Point::new(9, 19));
        assert_eq!(points[2], Point::new(8, 18));
        assert_eq!(points[3], Point::new(7, 17));
    }
}

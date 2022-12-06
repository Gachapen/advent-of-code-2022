use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = File::open("input.txt").unwrap();
    let reader = BufReader::new(input);

    let answer_one = solve_part_one(reader);
    println!("Answer 1: {}", answer_one);

    let input = File::open("input.txt").unwrap();
    let reader = BufReader::new(input);

    let answer_two = solve_part_two(reader);
    println!("Answer 2: {}", answer_two);
}

fn solve_part_one<R: BufRead>(input: R) -> usize {
    input
        .lines()
        .map(|line| AssignmentPair::parse_text(&line.unwrap()))
        .filter(|pair| pair.completely_overlaps())
        .count()
}

fn solve_part_two<R: BufRead>(input: R) -> usize {
    input
        .lines()
        .map(|line| AssignmentPair::parse_text(&line.unwrap()))
        .filter(|pair| pair.overlaps())
        .count()
}

#[derive(Debug)]
struct AssignmentPair {
    a: SectionAssignment,
    b: SectionAssignment,
}

impl AssignmentPair {
    fn parse_text(text: &str) -> Self {
        let pairs = text.split(',').collect::<Vec<_>>();

        Self {
            a: SectionAssignment::parse_text(pairs[0]),
            b: SectionAssignment::parse_text(pairs[1]),
        }
    }

    fn completely_overlaps(&self) -> bool {
        self.a.completely_overlaps(&self.b)
    }

    fn overlaps(&self) -> bool {
        self.a.overlaps(&self.b)
    }
}

#[derive(Debug)]
struct SectionAssignment {
    from: usize,
    to: usize,
}

lazy_static! {
    pub static ref ASSIGNMENT_REGEX: Regex = Regex::new(r"(\d+)-(\d+)").unwrap();
}

impl SectionAssignment {
    fn parse_text(text: &str) -> Self {
        let captures = ASSIGNMENT_REGEX.captures(text).unwrap();

        Self {
            from: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            to: captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        }
    }

    fn completely_overlaps(&self, other: &SectionAssignment) -> bool {
        (self.from >= other.from && self.to <= other.to)
            || (other.from >= self.from && other.to <= self.to)
    }

    fn overlaps(&self, other: &SectionAssignment) -> bool {
        (self.from >= other.from && self.from <= other.to)
            || (self.to >= other.from && self.to <= other.to)
            || (other.from >= self.from && other.from <= self.to)
            || (other.from >= self.from && other.to <= self.to)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    fn open_example() -> BufReader<File> {
        let file = File::open("example.txt").unwrap();
        BufReader::new(file)
    }

    #[test]
    fn test_part_one() {
        let input = open_example();
        assert_eq!(solve_part_one(input), 2);
    }

    #[test]
    fn test_part_two() {
        let input = open_example();
        assert_eq!(solve_part_two(input), 4);
    }
}

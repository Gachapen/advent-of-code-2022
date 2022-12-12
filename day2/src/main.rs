use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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

fn solve_part_one<R: BufRead>(input: R) -> u32 {
    input
        .lines()
        .map(|line| {
            let (opponent, me) = parse_line_shapes(&line.unwrap());
            get_round_score(&me, &opponent)
        })
        .sum()
}

fn solve_part_two<R: BufRead>(input: R) -> u32 {
    input
        .lines()
        .map(|line| {
            let (opponent_shape, outcome) = parse_line_shape_and_outcome(&line.unwrap());
            let my_shape = get_shape_required_for_outcome(&opponent_shape, &outcome);
            get_round_score(&my_shape, &opponent_shape)
        })
        .sum()
}

fn parse_line_shapes(line: &str) -> (Shape, Shape) {
    let mut chars = line.chars();

    let opponent = chars.next().unwrap();
    chars.next().unwrap();
    let me = chars.next().unwrap();

    (parse_shape(opponent), parse_shape(me))
}

fn parse_line_shape_and_outcome(line: &str) -> (Shape, Outcome) {
    let mut chars = line.chars();

    let opponent_shape = chars.next().unwrap();
    chars.next().unwrap();
    let outcome = chars.next().unwrap();

    (parse_shape(opponent_shape), parse_outcome(outcome))
}

fn parse_shape(input: char) -> Shape {
    match input {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        'X' => Shape::Rock,
        'Y' => Shape::Paper,
        'Z' => Shape::Scissors,
        _ => panic!("Unknown input"),
    }
}

fn parse_outcome(input: char) -> Outcome {
    match input {
        'X' => Outcome::Loss,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("Unknown input"),
    }
}

fn get_shape_required_for_outcome(opponent_shape: &Shape, outcome: &Outcome) -> Shape {
    if *outcome == Outcome::Draw {
        *opponent_shape
    } else {
        let losing_shape = get_losing_shape(opponent_shape);

        if *outcome == Outcome::Loss {
            losing_shape
        } else {
            get_losing_shape(&losing_shape)
        }
    }
}

fn get_round_score(my_shape: &Shape, others_shape: &Shape) -> u32 {
    let outcome = get_outcome(my_shape, others_shape);

    get_shape_score(my_shape) + get_outcome_score(&outcome)
}

fn get_shape_score(shape: &Shape) -> u32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn get_outcome_score(outcome: &Outcome) -> u32 {
    match outcome {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

fn get_outcome(my_shape: &Shape, others_shape: &Shape) -> Outcome {
    if *my_shape == *others_shape {
        Outcome::Draw
    } else {
        let losing_shape = get_losing_shape(my_shape);

        if *others_shape == losing_shape {
            Outcome::Win
        } else {
            Outcome::Loss
        }
    }
}

fn get_losing_shape(shape: &Shape) -> Shape {
    match shape {
        Shape::Rock => Shape::Scissors,
        Shape::Paper => Shape::Rock,
        Shape::Scissors => Shape::Paper,
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq)]
enum Outcome {
    Win,
    Draw,
    Loss,
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
        assert_eq!(solve_part_one(input), 15);
    }

    #[test]
    fn test_part_two() {
        let input = open_example();
        assert_eq!(solve_part_two(input), 12);
    }
}

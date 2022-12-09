use std::{
    cmp::{max, min},
    collections::HashSet,
    fmt::{self, Display},
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

fn solve_part_one<R: BufRead>(input: R) -> usize {
    solve::<R, 2>(input)
}

fn solve_part_two<R: BufRead>(input: R) -> usize {
    solve::<R, 10>(input)
}

fn solve<R: BufRead, const N: usize>(input: R) -> usize {
    let movements = input.lines().map(|line| read_line(&line.unwrap()));

    let mut knots = [Position::zero(); N];

    let mut visited = HashSet::<Position>::new();

    for movement in movements {
        for _ in 0..movement.repeats {
            let head_index = 0;
            let tail_index = N - 1;

            knots[head_index].translate(movement.x, movement.y);

            for i in (head_index + 1)..tail_index {
                let movement = find_movement_to_next_knot(&knots[i], &knots[i - 1]);
                knots[i].translate(movement.0, movement.1);
            }

            let movement = find_movement_to_next_knot(&knots[tail_index], &knots[tail_index - 1]);
            knots[N - 1].translate(movement.0, movement.1);

            visited.insert(knots[tail_index]);
        }
    }

    visited.len()
}

fn read_line(line: &str) -> Movement {
    let mut split = line.split(' ');
    let direction = split.next().unwrap().chars().next().unwrap();
    let repeats = split.next().unwrap().parse::<u32>().unwrap();

    let movement = match direction {
        'R' => (1, 0),
        'L' => (-1, 0),
        'U' => (0, 1),
        'D' => (0, -1),
        _ => panic!("Unknown direction"),
    };

    Movement {
        x: movement.0,
        y: movement.1,
        repeats,
    }
}

fn find_movement_to_next_knot(current: &Position, next: &Position) -> (i32, i32) {
    let distance = next.distance_from(current);

    if distance.0.abs() > 1 || distance.1.abs() > 1 {
        (max(min(distance.0, 1), -1), max(min(distance.1, 1), -1))
    } else {
        (0, 0)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn translate(&mut self, mov_x: i32, mov_y: i32) {
        self.x += mov_x;
        self.y += mov_y;
    }

    fn distance_from(&self, other: &Position) -> (i32, i32) {
        (self.x - other.x, self.y - other.y)
    }

    fn zero() -> Position {
        Position { x: 0, y: 0 }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:04}, {:04})", self.x, self.y)
    }
}

struct Movement {
    x: i32,
    y: i32,
    repeats: u32,
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    fn open_example_one() -> BufReader<File> {
        let file = File::open("example1.txt").unwrap();
        BufReader::new(file)
    }

    fn open_example_two() -> BufReader<File> {
        let file = File::open("example2.txt").unwrap();
        BufReader::new(file)
    }

    #[test]
    fn test_part_one() {
        let input = open_example_one();
        assert_eq!(solve_part_one(input), 13);
    }

    #[test]
    fn test_part_two() {
        let input = open_example_two();
        assert_eq!(solve_part_two(input), 36);
    }
}

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

fn solve_part_one<R: BufRead>(input: R) -> usize {
    *input
        .lines()
        .fold(vec![0], |mut sums, line| {
            let line = line.unwrap();

            if line.is_empty() {
                sums.push(0);
            } else {
                let last_index = sums.len() - 1;
                let updated_sum = sums[last_index] + line.parse::<usize>().unwrap();
                sums[last_index] = updated_sum;
            }

            sums
        })
        .iter()
        .max()
        .unwrap()
}

fn solve_part_two<R: BufRead>(input: R) -> usize {
    let mut cals = input.lines().fold(vec![0], |mut sums, line| {
        let line = line.unwrap();

        if line.is_empty() {
            sums.push(0);
        } else {
            let last_index = sums.len() - 1;
            sums[last_index] = sums.last().unwrap() + line.parse::<usize>().unwrap();
        }

        sums
    });

    cals.sort_by(|cal1, cal2| cal2.cmp(cal1));

    cals.iter().take(3).sum::<usize>()
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
        assert_eq!(solve_part_one(input), 24000);
    }

    #[test]
    fn test_part_two() {
        let input = open_example();
        assert_eq!(solve_part_two(input), 45000);
    }
}

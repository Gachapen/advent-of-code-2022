use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use regex::Regex;

type CrateStack = Vec<char>;
type CraneMovement = (usize, usize, usize);

fn main() {
    let (mut stacks, movements) = load_input().unwrap();

    let crate_mover_9001 = true;

    for (move_count, from_stack_index, to_stack_index) in movements {
        if crate_mover_9001 {
            let stack_size = stacks[from_stack_index].len();
            let mut popped = stacks[from_stack_index].split_off(stack_size - move_count);
            stacks[to_stack_index].append(&mut popped);
        } else {
            for _ in 0..move_count {
                let popped = stacks[from_stack_index].pop().unwrap();
                stacks[to_stack_index].push(popped);
            }
        }
    }

    println!("{:?}", stacks);

    let top_crates = stacks.iter().map(|s| s.last().unwrap()).collect::<Vec<_>>();
    let answer = String::from_iter(top_crates);
    print!("{}", answer);
}

fn load_input() -> Result<(Vec<CrateStack>, Vec<CraneMovement>), io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut stacks = Vec::new();
    let mut movements = Vec::new();
    let movement_regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    for line in reader.lines() {
        let line = line?;

        if line.starts_with('[') {
            load_stack_row(&line, &mut stacks)
        } else if line.starts_with('m') {
            let captures = movement_regex.captures(&line).unwrap();
            let count = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let from_idx = captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
            let to_idx = captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

            movements.push((count, from_idx, to_idx));
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    Ok((stacks, movements))
}

fn load_stack_row(row: &str, stacks: &mut Vec<Vec<char>>) {
    let mut stack_index = 0;
    let mut char_index = 0;

    while char_index < row.len() {
        let cell = &row[char_index..char_index + 3];

        if cell != "   " {
            let label = cell.chars().nth(1).unwrap();

            while stack_index >= stacks.len() {
                stacks.push(Vec::new());
            }

            stacks[stack_index].push(label);
        }

        char_index += 4;
        stack_index += 1;
    }
}

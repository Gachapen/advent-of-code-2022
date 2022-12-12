use std::{
    collections::{HashMap, HashSet},
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
        .map(|line| find_shared_priorities_from_line(&line.unwrap()))
        .sum()
}

fn solve_part_two<R: BufRead>(input: R) -> u32 {
    let lines = input.lines().map(|line| line.unwrap());

    let mut group = vec![String::new(); 3];
    let mut priority_sum = 0_u32;

    for (i, line) in lines.enumerate() {
        let group_index = i % 3;

        group[group_index] = line;

        if group_index == 2 {
            let shared_item = find_shared_item_in_group(&group);
            priority_sum += get_priority(shared_item) as u32;
        }
    }

    priority_sum
}

fn find_shared_item_in_group(group: &[String]) -> u8 {
    let first_list = &group[0];
    let other_lists = group[1..]
        .iter()
        .map(|list| -> HashSet<u8> { HashSet::from_iter(list.bytes()) })
        .collect::<Vec<_>>();

    for byte in first_list.bytes() {
        if other_lists.iter().all(|list| list.contains(&byte)) {
            return byte;
        }
    }

    panic!("Found no shared item");
}

fn find_shared_priorities_from_line(line: &str) -> u32 {
    let compartment_len = line.len() / 2;

    let compartment_1 = &line[..compartment_len];
    let compartment_2 = &line[compartment_len..];

    let compartment_1_set = HashSet::<u8>::from_iter(compartment_1.as_bytes().iter().cloned());

    let mut shared_bytes = compartment_2
        .bytes()
        .filter(|b| compartment_1_set.contains(b))
        .collect::<Vec<_>>();
    shared_bytes.sort();
    shared_bytes.dedup();

    shared_bytes
        .into_iter()
        .map(|b| get_priority(b) as u32)
        .sum()
}

fn get_priority(byte: u8) -> u8 {
    const LOWERCASE_A: u8 = 97;
    const UPPERCASE_A: u8 = 65;

    if byte >= LOWERCASE_A {
        byte - LOWERCASE_A + 1
    } else {
        byte - UPPERCASE_A + 27
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
    fn test_get_priority() {
        assert_eq!(get_priority(b'b'), 2);
        assert_eq!(get_priority(b'B'), 28);
    }

    #[test]
    fn test_part_one() {
        let input = open_example();
        assert_eq!(solve_part_one(input), 157);
    }

    #[test]
    fn test_part_two() {
        let input = open_example();
        assert_eq!(solve_part_two(input), 70);
    }
}

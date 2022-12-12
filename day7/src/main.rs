use std::{
    fs::File,
    io::{BufRead, Read},
};

fn main() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let answer_one = solve_part_one(input.as_bytes());
    println!("Answer 1: {}", answer_one);

    let answer_two = solve_part_two(input.as_bytes());
    println!("Answer 2: {}", answer_two);
}

fn solve_part_one<R: BufRead>(input: R) -> usize {
    let directories = parse_input(input);

    directories
        .iter()
        .map(|d| d.size)
        .filter(|size| *size <= 100_000)
        .sum()
}

fn solve_part_two<R: BufRead>(input: R) -> usize {
    const TOTAL_SPACE: usize = 70_000_000;
    const DESIRED_UNUSED_SPACE: usize = 30_000_000;

    let directories = parse_input(input);

    let used_space = directories[0].size;
    let available_space = TOTAL_SPACE - used_space;
    let needed_extra_space = DESIRED_UNUSED_SPACE - available_space;

    let dir_to_delete = find_smallest_dir_with_minimum_size(&directories, needed_extra_space);

    dir_to_delete.size
}

fn find_smallest_dir_with_minimum_size(directories: &[Dir], minimum_size: usize) -> &Dir {
    let mut candidate_dir = &directories[0];

    for dir in directories.iter().filter(|d| d.size >= minimum_size) {
        if dir.size < candidate_dir.size {
            candidate_dir = dir;
        }
    }

    candidate_dir
}

fn parse_input<R: BufRead>(input: R) -> Vec<Dir> {
    let mut dirs = Vec::<Dir>::new();
    let mut dir_index = 0_usize;

    for line in input.lines() {
        let line = line.unwrap();

        if line.starts_with('$') {
            let command = &line[2..4];

            if command == "cd" {
                let cd_input = &line[5..];

                if cd_input == ".." {
                    dir_index = dirs[dir_index].parent_index;
                } else {
                    let dir_name = cd_input.to_string();

                    dirs.push(Dir {
                        name: dir_name,
                        parent_index: dir_index,
                        size: 0,
                    });
                    dir_index = dirs.len() - 1;
                }
            }
        } else if !line.starts_with('d') {
            let parts = line.split(' ').collect::<Vec<_>>();
            let size = parts[0].parse::<usize>().unwrap();

            // Could probably sum sizes for dir, then update
            update_parent_sizes(&mut dirs, dir_index, size);
        }
    }

    dirs
}

fn update_parent_sizes(directories: &mut [Dir], dir_index: usize, file_size: usize) {
    directories[dir_index].size += file_size;

    let mut child_index = dir_index;

    while child_index != 0 {
        let parent_index = directories[child_index].parent_index;
        let parent_dir = &mut directories[parent_index];

        parent_dir.size += file_size;

        child_index = parent_index;
    }
}

#[derive(Debug)]
struct Dir {
    name: String,
    size: usize,
    parent_index: usize,
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    fn get_example() -> BufReader<File> {
        BufReader::new(File::open("example.txt").unwrap())
    }

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(get_example()), 95437);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(get_example()), 24933642);
    }
}

use std::{
    collections::HashSet,
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
    let mut visible = HashSet::<(usize, usize)>::new();

    let mut col_maxes = Vec::<u8>::new();
    let mut bottom_visibilities = Vec::<Vec<(u8, usize, usize)>>::new();

    for (row, line) in input.lines().enumerate() {
        let line = line.unwrap();
        let bytes = line.as_bytes();

        col_maxes.resize(bytes.len(), 0);
        bottom_visibilities.resize_with(bytes.len(), Vec::new);

        let mut row_max = 0_u8;
        let mut right_visibility = Vec::<(u8, usize, usize)>::new();

        for (col, byte) in bytes.iter().enumerate() {
            let byte = *byte;

            if byte > row_max {
                visible.insert((row, col));
                row_max = byte;
            }

            while !right_visibility.is_empty() && byte >= right_visibility.last().unwrap().0 {
                right_visibility.pop();
            }
            right_visibility.push((byte, row, col));

            if byte > col_maxes[col] {
                visible.insert((row, col));
                col_maxes[col] = byte;
            }

            while !bottom_visibilities[col].is_empty()
                && byte >= bottom_visibilities[col].last().unwrap().0
            {
                bottom_visibilities[col].pop();
            }
            bottom_visibilities[col].push((byte, row, col));
        }

        for (_, row, col) in right_visibility {
            visible.insert((row, col));
        }
    }

    for bottom_visibility in bottom_visibilities {
        for (_, row, col) in bottom_visibility {
            visible.insert((row, col));
        }
    }

    visible.len()
}

fn solve_part_two<R: BufRead>(input: R) -> usize {
    let mut rows = Vec::new();
    let mut cols = Vec::new();

    for line in input.lines().map(|l| l.unwrap()) {
        let bytes = line.as_bytes();

        let row = bytes.to_vec();
        rows.push(row);

        cols.resize_with(bytes.len(), Vec::new);

        for (i, byte) in bytes.iter().enumerate() {
            cols[i].push(*byte);
        }
    }

    let mut highest_score = 0_usize;

    (0..rows.len()).for_each(|y| {
        (0..cols.len()).for_each(|x| {
            let row = &rows[y];
            let col = &cols[x];

            let vis_right = find_view_distance_forward(row, x);
            let vis_left = find_view_distance_backward(row, x);
            let vis_down = find_view_distance_forward(col, y);
            let vis_up = find_view_distance_backward(col, y);

            let score = vis_left * vis_right * vis_down * vis_up;

            if score > highest_score {
                highest_score = score;
            }
        })
    });

    highest_score
}

fn find_view_distance_forward(trees: &[u8], position: usize) -> usize {
    let current_height = trees[position];

    let blocking_tree_index = trees[(position + 1)..]
        .iter()
        .position(|height| *height >= current_height);

    if let Some(blocking_tree_index) = blocking_tree_index {
        blocking_tree_index + 1
    } else {
        trees.len() - position - 1
    }
}

fn find_view_distance_backward(trees: &[u8], position: usize) -> usize {
    let current_height = trees[position];

    let blocking_tree_index = trees[..position]
        .iter()
        .rev()
        .position(|height| *height >= current_height);

    if let Some(blocking_tree_index) = blocking_tree_index {
        blocking_tree_index + 1
    } else {
        position
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
        assert_eq!(solve_part_one(input), 21);
    }

    #[test]
    fn test_part_two() {
        let input = open_example();
        assert_eq!(solve_part_two(input), 8);
    }
}

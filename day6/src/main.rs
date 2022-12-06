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
    find_unique_sequence::<R, 4>(input)
}

fn solve_part_two<R: BufRead>(input: R) -> usize {
    find_unique_sequence::<R, 14>(input)
}

fn find_unique_sequence<R: BufRead, const N: usize>(mut input: R) -> usize {
    let mut marker_buffer = [0; N];
    input.read_exact(&mut marker_buffer).unwrap();

    let mut marker_buf_index = 0;
    let mut input_index = marker_buffer.len();
    let mut input_buffer: [u8; 1] = [0; 1];

    while !is_unique_sequence(&marker_buffer) && input.read(&mut input_buffer).unwrap() != 0 {
        let character = input_buffer[0];
        marker_buffer[marker_buf_index] = character;

        marker_buf_index = if marker_buf_index == marker_buffer.len() - 1 {
            0
        } else {
            marker_buf_index + 1
        };

        input_index += 1;
    }

    input_index
}

fn is_unique_sequence(marker: &[u8]) -> bool {
    for i in 0..marker.len() {
        for j in i + 1..marker.len() {
            if marker[i] == marker[j] {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_one() -> Vec<(String, usize)> {
        vec![
            ("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 5),
            ("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 11),
        ]
    }

    fn get_example_two() -> Vec<(String, usize)> {
        vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 23),
            ("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 26),
        ]
    }

    #[test]
    fn test_part_one() {
        for (input, expected_result) in get_example_one() {
            assert_eq!(solve_part_one(input.as_bytes()), expected_result);
        }
    }

    #[test]
    fn test_part_two() {
        for (input, expected_result) in get_example_two() {
            assert_eq!(solve_part_two(input.as_bytes()), expected_result);
        }
    }
}

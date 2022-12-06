#![feature(iter_array_chunks)]
#![feature(test)]
extern crate test;

use std::{collections::HashSet, ops::Deref};

fn main() {
    const DAY: &str = env!("CARGO_PKG_NAME");
    println!("++{DAY}++");
    println!("Part 1: {}", solve_part_one(include_str!("../input.txt")));
    println!("Part 2: {}", solve_part_two(include_str!("../input.txt")));
}

// Open ai solution
fn find_marker(data: &str, marker_len: usize) -> usize {
    let mut chars = vec![];

    for (i, c) in data.chars().enumerate() {
        chars.push(c);
        if chars.len() > marker_len {
            chars.remove(0);
        }

        if chars.len() == marker_len
            && chars
                .iter()
                .all(|c| chars.iter().filter(|d| d.deref() == c).count() == 1)
        {
            return i + 1;
        }
    }

    data.len()
}
#[allow(dead_code)]
fn find_first_n_distict_char_offset(input: &str, n: usize) -> i32 {
    let binding = input.trim().chars().collect::<Vec<_>>();
    let a = binding
        .windows(n)
        .take_while(|x| x.iter().collect::<HashSet<_>>().len() != n)
        .count();
    (a + n).try_into().unwrap()
}

fn solve_part_one(input: &str) -> usize {
    find_marker(input, 4)
}

fn solve_part_two(input: &str) -> usize {
    find_marker(input, 14)
}

#[cfg(test)]
mod tests {
    mod bench {
        use crate::{find_first_n_distict_char_offset, find_marker};
        use test::Bencher;

        #[bench]
        fn bench_human_solution(b: &mut Bencher) {
            b.iter(|| {
                let result = find_first_n_distict_char_offset(include_str!("../input.txt"), 4);
                assert_eq!(result, 1779);
                let result = find_first_n_distict_char_offset(include_str!("../input.txt"), 14);
                assert_eq!(result, 2635);
            });
        }
        #[bench]
        fn bench_machine_solution(b: &mut Bencher) {
            b.iter(|| {
                let result = find_marker(include_str!("../input.txt"), 4);
                assert_eq!(result, 1779);
                let result = find_marker(include_str!("../input.txt"), 14);
                assert_eq!(result, 2635);
            });
        }
    }

    mod pt1 {
        use crate::solve_part_one;

        #[test]
        fn example_0() {
            let result = solve_part_one("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
            assert_eq!(result, 7);
        }
        #[test]
        fn example_1() {
            let result = solve_part_one("bvwbjplbgvbhsrlpgdmjqwftvncz");
            assert_eq!(result, 5);
        }

        #[test]
        fn example_2() {
            let result = solve_part_one("nppdvjthqldpwncqszvftbrmjlhg");
            assert_eq!(result, 6);
        }

        #[test]
        fn example_3() {
            let result = solve_part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
            assert_eq!(result, 10);
        }

        #[test]
        fn example_4() {
            let result = solve_part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
            assert_eq!(result, 11);
        }

        #[test]
        fn result() {
            let result = solve_part_one(include_str!("../input.txt"));
            assert_eq!(result, 1779);
        }
    }
    mod pt2 {
        use crate::solve_part_two;

        #[test]
        fn example_0() {
            let result = solve_part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
            assert_eq!(result, 19);
        }

        #[test]
        fn example_1() {
            let result = solve_part_two("bvwbjplbgvbhsrlpgdmjqwftvncz");
            assert_eq!(result, 23);
        }

        #[test]
        fn example_2() {
            let result = solve_part_two("nppdvjthqldpwncqszvftbrmjlhg");
            assert_eq!(result, 23);
        }

        #[test]
        fn example_3() {
            let result = solve_part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
            assert_eq!(result, 29);
        }

        #[test]
        fn example_4() {
            let result = solve_part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
            assert_eq!(result, 26);
        }

        #[test]
        fn result() {
            let result = solve_part_two(include_str!("../input.txt"));
            assert_eq!(result, 2635);
        }
    }
}

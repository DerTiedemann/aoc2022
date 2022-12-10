use std::ops::Deref;

use day06;

fn main() {
    const DAY: &str = env!("CARGO_PKG_NAME");
    println!("++{DAY}++");
    println!("Part 1: {}", solve_part_one(include_str!("../input.txt")));
    println!("Part 2: {}", solve_part_two(include_str!("../input.txt")));
}

pub fn solve_part_one(input: &str) -> usize {
    day06::find_first_n_distict_char_offset(input, 4)
}

pub fn solve_part_two(input: &str) -> usize {
    day06::find_first_n_distict_char_offset(input, 14)
}

#[cfg(test)]
mod tests {

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

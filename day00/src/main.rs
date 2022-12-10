fn main() {
    const DAY: &str = env!("CARGO_PKG_NAME");
    println!("++{DAY}++");
    println!("Part 1: {}", solve_part_one(include_str!("../input.txt")));
    println!("Part 2: {}", solve_part_two(include_str!("../input.txt")));
}

pub fn solve_part_one(input: &str) -> usize {
    0
}

pub fn solve_part_two(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    mod pt1 {
        use crate::solve_part_one;

        #[test]
        fn example_0() {
            let result = solve_part_one(include_str!("../test.txt"));
            assert_eq!(result, 1);
        }

        #[test]
        fn result() {
            let result = solve_part_one(include_str!("../input.txt"));
            assert_eq!(result, 1);
        }
    }
    mod pt2 {
        use crate::solve_part_two;

        #[test]
        fn example_0() {
            let result = solve_part_two(include_str!("../test.txt"));
            assert_eq!(result, 1);
        }

        #[test]
        fn result() {
            let result = solve_part_two(include_str!("../input.txt"));
            assert_eq!(result, 1);
        }
    }
}

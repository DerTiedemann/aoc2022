use std::collections::BinaryHeap;

fn main() {
    println!("{}", solve_part_one(include_str!("../input.txt")));
    println!("{}", solve_part_two(include_str!("../input.txt")));
}

fn top_n_elves(input: &str, n: usize) -> i32 {
    let mut res: Vec<i32> = input
        .lines()
        .fold(Vec::new(), |mut acc, x| {
            if acc.is_empty() || x.is_empty() {
                acc.push(Vec::new())
            }
            if !x.is_empty() {
                // Not sure if this can be cleaner, e.g. how expensive is x.is_empty()
                acc.last_mut().unwrap().push(x.parse::<i32>().unwrap());
            }
            acc
        })
        .iter()
        .map(|x| x.iter().sum())
        .collect();
    res.sort(); // This is not optimal, but i cant be bothered to write it uwu O(n) vs O(n logn)

    res.iter().rev().take(n).sum()
}

#[allow(dead_code)]
fn solve_part_one_simple(input: &str) -> i32 {
    let mut max = i32::MIN;
    let mut current = 0;
    for l in input.lines() {
        if let Ok(a) = l.parse::<i32>() {
            current += a;
        } else {
            max = max.max(current);
            current = 0;
        }
    }
    max.max(current)
}

fn solve_part_one(input: &str) -> i32 {
    top_n_elves(input, 1)
}

fn solve_part_two(input: &str) -> i32 {
    top_n_elves(input, 3)
}

#[cfg(test)]
mod tests {
    mod pt1 {
        use crate::solve_part_one;
        #[test]
        fn example() {
            let result = solve_part_one(include_str!("../test.txt"));
            assert_eq!(result, 24000);
        }

        #[test]
        fn result() {
            let result = solve_part_one(include_str!("../input.txt"));
            assert_eq!(result, 69281);
        }
    }
    mod pt2 {
        use crate::solve_part_two;

        #[test]
        fn example() {
            let result = solve_part_two(include_str!("../test.txt"));
            assert_eq!(result, 45000);
        }

        #[test]
        fn result() {
            let result = solve_part_two(include_str!("../input.txt"));
            assert_eq!(result, 201524);
        }

        #[test]
        fn custom_test_1() {
            let result = solve_part_two(include_str!("../custom_test_1.txt"));
            assert_eq!(result, 24);
        }
        #[test]
        fn custom_test_2() {
            let result = solve_part_two(include_str!("../custom_test_2.txt"));
            assert_eq!(result, 24);
        }
    }
}

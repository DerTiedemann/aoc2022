#![feature(iter_array_chunks)]
use std::collections::HashSet;

fn main() {
    const DAY: &str = env!("CARGO_PKG_NAME");
    println!("++{DAY}++");
    println!("Part 1: {}", solve_part_one(include_str!("../input.txt")));
    println!("Part 2: {}", solve_part_two(include_str!("../input.txt")));
}

fn score(c: char) -> i32 {
    if c.is_ascii_lowercase() {
        c as i32 - 'a' as i32 + 1
    } else {
        c as i32 - 'A' as i32 + 1 + 26
    }
}

fn solve_part_one(input: &str) -> i32 {
    input
        .lines()
        .map(|x| {
            let mut a = HashSet::new();
            let mut b = HashSet::new();
            let (c, d) = x.split_at(x.len() / 2);

            for i in c.chars() {
                a.insert(i);
            }
            for i in d.chars() {
                b.insert(i);
            }

            score(*a.intersection(&b).next().unwrap())
        })
        .sum()
}

fn solve_part_two(input: &str) -> i32 {
    input
        .lines()
        .array_chunks()
        .map(|x: [&str; 3]| {
            let mut a = vec![HashSet::new(), HashSet::new(), HashSet::new()];
            for (i, b) in x.iter().enumerate() {
                for c in b.chars() {
                    a[i].insert(c);
                }
            }

            let d = a.iter().skip(1).fold(a[0].clone(), |acc, hs| {
                acc.intersection(hs).cloned().into_iter().collect()
            });

            score(*d.iter().next().unwrap())
        })
        .sum()
}

#[cfg(test)]
mod tests {
    mod pt1 {
        use crate::solve_part_one;
        #[test]
        fn example() {
            let result = solve_part_one(include_str!("../test.txt"));
            assert_eq!(result, 157);
        }

        #[test]
        fn result() {
            let result = solve_part_one(include_str!("../input.txt"));
            assert_eq!(result, 8233);
        }
    }
    mod pt2 {
        use crate::solve_part_two;

        #[test]
        fn example() {
            let result = solve_part_two(include_str!("../test.txt"));
            assert_eq!(result, 70);
        }

        #[test]
        fn result() {
            let result = solve_part_two(include_str!("../input.txt"));
            assert_eq!(result, 2821);
        }
    }
}

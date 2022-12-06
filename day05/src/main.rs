#![feature(iter_array_chunks)]
#![feature(generic_arg_infer)]
#![feature(map_many_mut)]

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref CRATE_REGEX: regex::Regex = regex::Regex::new(r"\[([A-Z])\]").unwrap();
}

fn main() {
    const DAY: &str = env!("CARGO_PKG_NAME");
    println!("++{DAY}++");
    println!("Part 1: {}", solve_part_one(include_str!("../input.txt")));
    println!("Part 2: {}", solve_part_two(include_str!("../input.txt")));
}

type ParsedInput = (HashMap<usize, Vec<char>>, Vec<(usize, usize, usize)>);

fn parse_input(input: &str) -> ParsedInput {
    let mut parts = input.split("\n\n");
    let crates = parts
        .next()
        .unwrap()
        .lines()
        .rev()
        .skip(1)
        .flat_map(|x| {
            CRATE_REGEX.captures_iter(x).map(|m| {
                let a = m.get(1).unwrap();
                ((a.start() - 1) / 4, a.as_str().chars().next().unwrap())
            })
        })
        .fold(
            HashMap::new(),
            |mut acc: HashMap<usize, Vec<char>>, (i, letter)| {
                match acc.get_mut(&i) {
                    Some(v) => v.push(letter),
                    None => {
                        let v = vec![letter];
                        acc.insert(i, v);
                    }
                }

                acc
            },
        );

    let commands = parts
        .next()
        .unwrap()
        .lines()
        .map(|x| {
            let mut i = x.split_ascii_whitespace();
            (
                i.nth(1).unwrap().parse::<usize>().unwrap(),
                i.nth(1).unwrap().parse::<usize>().unwrap() - 1,
                i.nth(1).unwrap().parse::<usize>().unwrap() - 1,
            )
        })
        .collect::<Vec<_>>();
    (crates, commands)
}

fn build_solution_string(crates: HashMap<usize, Vec<char>>) -> String {
    let mut res = String::new();

    for i in 0..crates.len() {
        res.push(crates.get(&i).unwrap().last().unwrap().to_owned())
    }

    res
}

fn solve_part_one(input: &str) -> String {
    let (mut crates, commands) = parse_input(input);
    for (amount, from_id, to_id) in commands {
        let [from, to] = crates.get_many_mut([&from_id, &to_id]).unwrap();

        for _ in 0..amount {
            to.push(from.pop().unwrap())
        }
    }
    build_solution_string(crates)
}

fn solve_part_two(input: &str) -> String {
    let (mut crates, commands) = parse_input(input);
    for (amount, from_id, to_id) in commands {
        let [from, to] = crates.get_many_mut([&from_id, &to_id]).unwrap();

        to.append(&mut from.drain((from.len() - amount)..).collect::<Vec<_>>());
    }

    build_solution_string(crates)
}

#[cfg(test)]
mod tests {
    mod pt1 {
        use crate::solve_part_one;
        #[test]
        fn example() {
            let result = solve_part_one(include_str!("../test.txt"));
            assert_eq!(result, "CMZ".to_string());
        }

        #[test]
        fn result() {
            let result = solve_part_one(include_str!("../input.txt"));
            assert_eq!(result, "GFTNRBZPF".to_string());
        }
    }
    mod pt2 {
        use crate::solve_part_two;

        #[test]
        fn example() {
            let result = solve_part_two(include_str!("../test.txt"));
            assert_eq!(result, "MCD".to_string());
        }

        #[test]
        fn result() {
            let result = solve_part_two(include_str!("../input.txt"));
            assert_eq!(result, "VRQWPDSGP".to_string());
        }
    }
}

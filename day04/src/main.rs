#![feature(iter_array_chunks)]

fn main() {
    const DAY: &str = env!("CARGO_PKG_NAME");
    println!("++{DAY}++");
    println!("Part 1: {}", solve_part_one(include_str!("../input.txt")));
    println!("Part 2: {}", solve_part_two(include_str!("../input.txt")));
}

fn fully_overlaps(mut a: (i32, i32), mut b: (i32, i32)) -> bool {
    if a.0 > b.0 || (a.0 == b.0 && a.1 < b.1) {
        std::mem::swap(&mut a, &mut b);
    }

    b.0 >= a.0 && b.1 <= a.1
}

fn overlaps(a: (i32, i32), b: (i32, i32)) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}

#[allow(dead_code)]
fn print_sections(a: (i32, i32), b: (i32, i32), intersects: bool) {
    let max = a.0.max(a.1.max(b.0.max(b.1)));
    let mut l1 = String::new();
    let mut l2 = String::new();
    for i in 1..=max {
        let j = i.to_string();
        if i >= a.0 && i <= a.1 {
            l1.push_str(&j);
        } else {
            for _ in 0..j.len() {
                l1.push('#');
            }
        }

        if i >= b.0 && i <= b.1 {
            l2.push_str(&j);
        } else {
            for _ in 0..j.len() {
                l2.push('#');
            }
        }
    }

    println!("{l1} {a:?} {intersects}");
    println!("{l2} {b:?} {intersects}");
}

fn into_tuple_pairs(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .flat_map(|x| {
            x.split(',').map(|y| {
                let mut a = y.split('-');
                (
                    a.next().unwrap().parse::<i32>().unwrap(),
                    a.next().unwrap().parse::<i32>().unwrap(),
                )
            })
        })
        .collect()
}

fn solve_part_one(input: &str) -> i32 {
    into_tuple_pairs(input)
        .chunks_exact(2)
        .into_iter()
        .filter(|a| fully_overlaps(a[0], a[1]))
        .count()
        .try_into()
        .unwrap()
}

fn solve_part_two(input: &str) -> i32 {
    into_tuple_pairs(input)
        .chunks_exact(2)
        .into_iter()
        .filter(|a| overlaps(a[0], a[1]))
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    mod pt1 {
        use crate::solve_part_one;
        #[test]
        fn example() {
            let result = solve_part_one(include_str!("../test.txt"));
            assert_eq!(result, 2);
        }

        #[test]
        fn custom_test_1() {
            let result = solve_part_one(include_str!("../custom_test_1.txt"));
            assert_eq!(result, 0);
        }

        #[test]
        fn custom_test_2() {
            let result = solve_part_one(include_str!("../custom_test_2.txt"));
            assert_eq!(result, 2);
        }

        #[test]
        fn result() {
            let result = solve_part_one(include_str!("../input.txt"));
            assert_eq!(result, 494);
        }
    }
    mod pt2 {
        use crate::solve_part_two;

        #[test]
        fn example() {
            let result = solve_part_two(include_str!("../test.txt"));
            assert_eq!(result, 4);
        }

        #[test]
        fn result() {
            let result = solve_part_two(include_str!("../input.txt"));
            assert_eq!(result, 833);
        }
    }
}

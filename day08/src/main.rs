use std::collections::HashSet;

fn main() {
    const DAY: &str = env!("CARGO_PKG_NAME");
    println!("++{DAY}++");
    println!("Part 1: {}", solve_part_one(include_str!("../input.txt")));
    println!("Part 2: {}", solve_part_two(include_str!("../input.txt")));
}

fn get_max(
    x: usize,
    y: usize,
    max: i32,
    tree_height: &usize,
    visible_trees: &mut HashSet<(usize, usize)>,
) -> i32 {
    let tree_height: i32 = *tree_height as i32;
    if tree_height > max {
        visible_trees.insert((x, y));
        tree_height
    } else {
        max
    }
}

fn parse_input(input: &str) -> vecgrid::Vecgrid<usize> {
    let forest: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    vecgrid::Vecgrid::from_rows(forest).unwrap()
}

pub fn solve_part_one(input: &str) -> u64 {
    let grid = parse_input(input);
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
    for (y, row) in grid.as_rows().iter().enumerate() {
        let mut max = -1;
        for (x, tree_height) in row.iter().enumerate() {
            if max == 9 {
                break;
            }
            max = get_max(x, y, max, tree_height, &mut visible_trees);
        }

        max = -1;
        for (x, tree_height) in row.iter().enumerate().rev() {
            if max == 9 {
                break;
            }
            max = get_max(x, y, max, tree_height, &mut visible_trees);
        }
    }

    for (x, col) in grid.as_columns().iter().enumerate() {
        let mut max = -1;
        for (y, tree_height) in col.iter().enumerate() {
            if max == 9 {
                break;
            }
            max = get_max(x, y, max, tree_height, &mut visible_trees);
        }

        max = -1;
        for (y, tree_height) in col.iter().enumerate().rev() {
            if max == 9 {
                break;
            }
            max = get_max(x, y, max, tree_height, &mut visible_trees);
        }
    }

    visible_trees.len() as u64
}

fn calculate_viewing_distance<'a, I: Iterator<Item = &'a usize>>(mut vals: I) -> usize {
    let height = vals.next().unwrap();
    let mut viewing_disance = 0_usize;
    for val in vals {
        viewing_disance += 1;
        if height <= val {
            break;
        }
    }
    viewing_disance
}

pub fn solve_part_two(input: &str) -> u64 {
    let grid = parse_input(input);

    grid.indices_row_major()
        .map(|(tree_y, tree_x)| {
            let left_score = calculate_viewing_distance(
                grid.row_iter(tree_y)
                    .unwrap()
                    .rev()
                    .skip(grid.row_len() - tree_x - 1),
            );
            let right_score =
                calculate_viewing_distance(grid.row_iter(tree_y).unwrap().skip(tree_x));

            let top_score = calculate_viewing_distance(
                grid.column_iter(tree_x)
                    .unwrap()
                    .rev()
                    .skip(grid.column_len() - tree_y - 1),
            );

            let bottom_score =
                calculate_viewing_distance(grid.column_iter(tree_x).unwrap().skip(tree_y));
            right_score * left_score * top_score * bottom_score
        })
        .max()
        .unwrap() as u64
}

#[cfg(test)]
mod tests {
    mod pt1 {
        use crate::solve_part_one;

        #[test]
        fn example_0() {
            let result = solve_part_one(include_str!("../test.txt"));
            assert_eq!(result, 21);
        }

        #[test]
        fn custom_1() {
            let result = solve_part_one(include_str!("../custom_1.txt"));
            assert_eq!(result, 16);
        }

        #[test]
        fn result() {
            let result = solve_part_one(include_str!("../input.txt"));
            assert_eq!(result, 1647);
        }
    }
    mod pt2 {
        use crate::calculate_viewing_distance;
        #[test]
        fn general_viewing_distance_test() {
            let a = vec![3, 2, 1];
            let result = calculate_viewing_distance(a.iter());
            assert_eq!(result, 2);

            let a = vec![3, 2, 3];
            let result = calculate_viewing_distance(a.iter());
            assert_eq!(result, 2);

            let a = vec![3, 2, 4];
            let result = calculate_viewing_distance(a.iter());
            assert_eq!(result, 2);

            let a = vec![3, 3, 3];
            let result = calculate_viewing_distance(a.iter());
            assert_eq!(result, 1);

            let a = vec![3, 4];
            let result = calculate_viewing_distance(a.iter());
            assert_eq!(result, 1);

            let a = vec![3];
            let result = calculate_viewing_distance(a.iter());
            assert_eq!(result, 0);
        }

        use crate::solve_part_two;

        #[test]
        fn example_0() {
            let result = solve_part_two(include_str!("../test.txt"));
            assert_eq!(result, 8);
        }
        #[test]
        fn custom_1() {
            let result = solve_part_two(include_str!("../custom_1.txt"));
            assert_eq!(result, 1);
        }

        #[test]
        fn result() {
            let result = solve_part_two(include_str!("../input.txt"));
            assert_eq!(result, 392080);
        }
    }
}

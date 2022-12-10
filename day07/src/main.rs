use std::{
    collections::HashMap,
    fmt::{self, Write},
    ops::Deref,
};

// i didnt solve this one today and this effectively just yoinked this code: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=2d4c298f177b67438066b275a5a8e152

fn main() {
    const DAY: &str = env!("CARGO_PKG_NAME");
    println!("++{DAY}++");
    println!("Part 1: {}", solve_part_one(include_str!("../input.txt")));
    println!("Part 2: {}", solve_part_two(include_str!("../input.txt")));
}

#[derive(Clone, Debug)]
enum Node {
    Dir {
        children: HashMap<String, Node>,
        total_size: u64,
    },
    File {
        size: u64,
    },
}

impl Node {
    fn new_dir() -> Self {
        Node::Dir {
            children: HashMap::new(),
            total_size: 0,
        }
    }

    fn children(&mut self) -> &mut HashMap<String, Node> {
        match self {
            Node::Dir { children, .. } => children,
            Node::File { .. } => panic!("files have no children"),
        }
    }

    fn children_at(&mut self, path: &[&str]) -> &mut HashMap<String, Node> {
        if let Some((&first, rest)) = path.split_first() {
            self.children()
                .entry(first.to_owned())
                .or_insert_with(Node::new_dir)
                .children_at(rest)
        } else {
            self.children()
        }
    }

    fn compute_total_size(&mut self) -> u64 {
        match *self {
            Node::Dir {
                ref mut children,
                ref mut total_size,
                ..
            } => {
                *total_size = children.values_mut().map(Node::compute_total_size).sum();
                *total_size
            }
            Node::File { size, .. } => size,
        }
    }

    fn sum_total_dir_sizes(&self, max_size: u64) -> u64 {
        match *self {
            Node::File { .. } => 0,
            Node::Dir {
                ref children,
                total_size,
                ..
            } => {
                let new_size = if total_size <= max_size {
                    total_size
                } else {
                    0
                };

                let children_total = children
                    .values()
                    .map(|node| node.sum_total_dir_sizes(max_size))
                    .sum::<u64>();

                children_total + new_size
            }
        }
    }

    fn get_all_dirs<'a>(&'a self, dirs: &mut Vec<&'a Node>) {
        match *self {
            Node::Dir { ref children, .. } => {
                dirs.push(self);
                children
                    .values()
                    .filter(|node| matches!(node, Node::Dir { .. }))
                    .for_each(|node| node.get_all_dirs(dirs));
            }
            Node::File { .. } => {}
        }
    }

    fn dir_size(&self) -> u64 {
        match *self {
            Node::Dir { total_size, .. } => total_size,
            Node::File { .. } => panic!("file doesn't have a directory size"),
        }
    }
}

fn parse(input: &str) -> Node {
    let output_lines = input.lines();

    let mut path = Vec::new();
    let mut root = Node::new_dir();

    for line in output_lines {
        let line = line.trim();

        if line.starts_with('$') {
            let line = line.trim_start_matches('$').trim();
            if line.starts_with("cd") {
                match line.split_ascii_whitespace().nth(1).unwrap() {
                    "/" => path.clear(),
                    ".." => _ = path.pop(),
                    a => path.push(a),
                }
            }
        } else {
            let (first, name) = line.split_once(' ').unwrap();

            let node = match first {
                "dir" => Node::new_dir(),
                size => Node::File {
                    size: size.parse().unwrap(),
                },
            };

            root.children_at(&path)
                .entry(name.to_string())
                .or_insert(node);
        }
    }
    root.compute_total_size();
    root
}

pub fn solve_part_one(input: &str) -> u64 {
    parse(input).sum_total_dir_sizes(100_000)
}
pub fn solve_part_two(input: &str) -> u64 {
    let root = parse(input);
    const DISK_SPACE: u64 = 70_000_000;
    const REQUIRED_SPACE: u64 = 30_000_000;
    let current_free_space = DISK_SPACE - root.dir_size();
    let min_size = REQUIRED_SPACE - current_free_space;

    let mut dirs = Vec::new();
    root.get_all_dirs(&mut dirs);

    dirs.iter()
        .map(|x| x.dir_size())
        .filter(|x| x.deref() >= &min_size)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    mod pt1 {
        use crate::solve_part_one;

        #[test]
        fn example_0() {
            let result = solve_part_one(include_str!("../test.txt"));
            assert_eq!(result, 95437);
        }

        #[test]
        fn result() {
            let result = solve_part_one(include_str!("../input.txt"));
            assert_eq!(result, 1306611);
        }
    }
    mod pt2 {
        use crate::solve_part_two;

        #[test]
        fn example_0() {
            let result = solve_part_two(include_str!("../test.txt"));
            assert_eq!(result, 24933642);
        }

        #[test]
        fn result() {
            let result = solve_part_two(include_str!("../input.txt"));
            assert_eq!(result, 1);
        }
    }
}

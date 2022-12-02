use std::cmp::Ordering;

fn main() {
    const DAY: &str = env!("CARGO_PKG_NAME");
    println!("++{DAY}++");
    println!("Part 1: {}", solve_part_one(include_str!("../input.txt")));
    println!("Part 2: {}", solve_part_two(include_str!("../input.txt")));
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Choice {
    Rock,
    Paper,
    Sissors,
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Choice {
    fn win(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Sissors,
            Choice::Sissors => Choice::Rock,
        }
    }

    fn lose(&self) -> Choice {
        match self {
            Choice::Rock => Self::Sissors,
            Choice::Paper => Self::Rock,
            Choice::Sissors => Self::Paper,
        }
    }
}

// Rock < Paper
// Rock > Sissors

// Paper < Sissor
// Paper > Rock

// Sissor < Rock
// Sissor > Paper

impl Ord for Choice {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Choice::Rock => match other {
                Choice::Rock => Ordering::Equal,
                Choice::Paper => Ordering::Less,
                Choice::Sissors => Ordering::Greater,
            },
            Choice::Paper => match other {
                Choice::Rock => Ordering::Greater,
                Choice::Paper => Ordering::Equal,
                Choice::Sissors => Ordering::Less,
            },
            Choice::Sissors => match other {
                Choice::Rock => Ordering::Less,
                Choice::Paper => Ordering::Greater,
                Choice::Sissors => Ordering::Equal,
            },
        }
    }
}

fn get_choice(resp: &str) -> Choice {
    match resp {
        "A" | "X" => Choice::Rock,
        "B" | "Y" => Choice::Paper,
        "C" | "Z" => Choice::Sissors,
        a => panic!("unexpected input: {a}"),
    }
}

fn get_outcome(opponent: Choice, resp: &str) -> Choice {
    match resp {
        "X" => opponent.lose(),
        "Y" => opponent,
        "Z" => opponent.win(),
        a => panic!("unexpected input: {a}"),
    }
}

fn score(opponent: Choice, response: Choice) -> i32 {
    let mut score = match opponent.cmp(&response) {
        Ordering::Less => 6,
        Ordering::Equal => 3,
        Ordering::Greater => 0,
    };
    score += match response {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Sissors => 3,
    };
    score
}

fn solve_part_one(input: &str) -> i32 {
    input
        .lines()
        .map(|x| {
            let mut b = x.split(' ');
            (get_choice(b.next().unwrap()), get_choice(b.next().unwrap()))
        })
        .map(|(opponent, response)| score(opponent, response))
        .sum::<i32>()
}

fn solve_part_two(input: &str) -> i32 {
    input
        .lines()
        .map(|x| {
            let mut b = x.split(' ');
            let opponent = get_choice(b.next().unwrap());
            (opponent.clone(), get_outcome(opponent, b.next().unwrap()))
        })
        .map(|(opponent, response)| score(opponent, response))
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    mod pt1 {
        use crate::solve_part_one;
        #[test]
        fn example() {
            let result = solve_part_one(include_str!("../test.txt"));
            assert_eq!(result, 15);
        }

        #[test]
        fn result() {
            let result = solve_part_one(include_str!("../input.txt"));
            assert_eq!(result, 11906);
        }
    }
    mod pt2 {
        use crate::solve_part_two;

        #[test]
        fn example() {
            let result = solve_part_two(include_str!("../test.txt"));
            assert_eq!(result, 12);
        }

        #[test]
        fn result() {
            let result = solve_part_two(include_str!("../input.txt"));
            assert_eq!(result, 11186);
        }
    }
}

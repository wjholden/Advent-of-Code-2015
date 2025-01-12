use std::sync::mpsc::{self, Sender};

fn main() {
    let puzzle = include_str!("../../puzzles/day17.txt").trim();
    let (part1, part2) = solve(150, puzzle);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

/// I'm not exactly proud of this code. This problem looked like a Knapsack problem,
/// but it's slightly different because we aren't looking to maximize anything,
/// but rather count combinations.
fn combinations(
    total: u8,
    solution: &mut Vec<u8>,
    tx: &Sender<usize>,
    containers: &[u8],
) -> u16 {
    if total == 0 {
        tx.send(solution.len()).unwrap();
        1
    } else if containers.is_empty() {
        0
    } else {
        let head = containers[0];
        if head <= total {
            solution.push(head);
            let left = combinations(total - head, solution, tx, &containers[1..]);
            solution.pop();
            let right = combinations(total, solution, tx, &containers[1..]);
            left + right
        } else {
            combinations(total, solution, tx, &containers[1..])
        }
    }
}

/// I just wanted to use a channel. Slightly less clean than Go, but it works.
/// I was surprised to learn that you can use channels within a single thread.
/// Probably not a great queue implementation but meh.
fn solve(total: u8, input: &str) -> (u16, usize) {
    let containers: Vec<u8> = input.lines().map(|l| l.parse().unwrap()).collect();
    let (tx, rx) = mpsc::channel();
    let part1 = combinations(total, &mut Vec::with_capacity(32), &tx, &containers);
    drop(tx);

    // I had thought that this would be faster than the more functional approach
    // of collecting the values, finding the minimum, and counting those instances,
    // but this surprisingly isn't much better or worse.
    let mut min = usize::MAX;
    let mut part2 = 0;
    for solution in rx.into_iter() {
        match solution.cmp(&min) {
            std::cmp::Ordering::Less => {
                min = solution;
                part2 = 1   
            },
            std::cmp::Ordering::Equal => part2 += 1,
            std::cmp::Ordering::Greater => (),
        }
    }

    (part1, part2)
}

#[cfg(test)]
mod day17 {
    use super::*;

    const SAMPLE: &str = "20
15
10
5
5";

    #[test]
    fn test1() {
        let w = 25;
        assert_eq!(solve(w, SAMPLE).0, 4)
    }

    #[test]
    fn test2() {
        assert_eq!(solve(25, SAMPLE).1, 3)
    }
}

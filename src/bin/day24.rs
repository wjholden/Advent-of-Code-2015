use itertools::Itertools;

fn main() {
    let puzzle = include_str!("../../puzzles/day24.txt").trim();
    println!("Part 1: {}", part1(puzzle).unwrap()); // 29728298883 too high
    // 201927595423 also too high.
                                                    // println!("Part 2: {}", part2(puzzle));
}

fn part1(input: &str) -> Option<u64> {
    // https://stackoverflow.com/a/36020284/5459668
    let mut packages: Vec<u64> = input.lines().flat_map(str::parse).collect();
    packages.sort();
    packages.reverse();
    let third = packages.iter().sum::<u64>() / 3;

    // Maybe the greedy approach works?
    let n = packages.len();
    for i in 0..n {
        let mut solution = Vec::new();
        if let Some(qe) = find(third, &packages[i..], &mut solution) {
            return Some(qe);
        }
    }
    None
}

/// This function works on the sample input but not the puzzle because it fails
/// disqualify solutions where the other two thirds of the packages cannot make
/// the desired total. We need to somehow track the de-selected packages and
/// determine if we can half them.
fn find(total: u64, packages: &[u64], solution: &mut Vec<u64>) -> Option<u64> {
    if total == 0 {
        // The sum of the candidate solution is 1/3 of the original input,
        // but we need to test if the other 2/3s can be cleanly split in half.
        let rest: Vec<u64> = packages
            .iter()
            .filter_map(|i| {
                if !solution.contains(i) {
                    Some(*i)
                } else {
                    None
                }
            })
            .collect();
        if is_bisectable(&rest) {
            Some(1)
        } else {
            None
        }
    } else if packages.is_empty() {
        None
    } else {
        for i in 0..packages.len() {
            let candidate = packages[i];
            if candidate <= total {
                solution.push(candidate);
                if let Some(qe) = find(total - candidate, &packages[i + 1..], solution) {
                    return Some(qe * candidate);
                }
                solution.pop();
            }
        }
        None
    }
}

fn is_bisectable(x: &[u64]) -> bool {
    let whole = x.iter().sum::<u64>();
    let half = whole / 2;
    if half * 2 != whole {
        return false;
    }
    let n = x.len();
    for size in 1..=(n/2) {
        // There's no need to go beyond n/2 because the other partition will
        // have sizes (n-1), (n-2), and so on.
        for partition in x.iter().combinations(size) {
            if partition.into_iter().sum::<u64>() == half {
                
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod day24 {
    use super::*;

    const SAMPLE: &str = "1
2
3
4
5
7
8
9
10
11";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE), Some(99))
    }
}

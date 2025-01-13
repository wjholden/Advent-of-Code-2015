fn main() {
    let puzzle = include_str!("../../puzzles/day24.txt").trim();
    println!("Part 1: {}", part1(puzzle).unwrap()); // 29728298883 too high
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
        if let Some(qe) = find(third, &packages[i..]) {
            return Some(qe)
        }
    }
    None
}

/// This function works on the sample input but not the puzzle because it fails
/// disqualify solutions where the other two thirds of the packages cannot make
/// the desired total. We need to somehow track the de-selected packages and
/// determine if we can half them.
fn find(total: u64, packages: &[u64]) -> Option<u64> {
    if total == 0 {
        Some(1)
    } else if packages.is_empty() {
        None
    } else {
        for i in 0..packages.len() {
            let candidate = packages[i];
            if candidate <= total {
                if let Some(qe) = find(total - candidate, &packages[i+1..]) {
                    return Some(qe * candidate)
                }
            }
        }
        None
    }
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

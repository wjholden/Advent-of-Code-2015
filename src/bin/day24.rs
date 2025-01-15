use std::{fmt::{Debug, Display}, ops::Add};

use itertools::Itertools;

fn main() {
    let puzzle = include_str!("../../puzzles/day24.txt").trim();
    // https://stackoverflow.com/a/36020284/5459668
    let mut packages: Vec<u64> = puzzle.lines().flat_map(str::parse).collect();
    packages.sort();
    packages.reverse();

    println!("Part 1: {}", balance(&packages, 3).unwrap());
    println!("Part 2: {}", balance(&packages, 4).unwrap());
}

fn smallest_candidate_solution(packages: &[u64], sum: u64) -> usize {
    assert!(packages.is_sorted_by(|a,b| a >= b));
    let mut lower_bound = 1;
    while lower_bound < packages.len() && packages[..=lower_bound].iter().sum::<u64>() < sum {
        lower_bound += 1;
    }
    lower_bound
}

#[allow(dead_code)]
fn part1(packages: &[u64]) -> Option<u64> {
    let whole = packages.iter().sum::<u64>();
    let third = whole / 3;
    assert_eq!(third * 3, whole);
    //println!("We are looking for {third} (1/3 of {whole}).");

    // Maybe the greedy approach works?
    //let mut solution = Vec::new();
    //let _qe = greedy_find(third, &packages, &packages, &mut solution);
    //println!("The greedy approach gives solution {solution:?} (QE = {}).", _qe.unwrap());
    
    // Unfortunately this apparently does *not* produce a valid solution.
    // We need to iterate towards the optimum. Let's use the size of our greedy
    // solution as an upper bound.
    //let upper_bound = solution.len();
    
    // For the lower bound, we can find the smallest slice of packages that
    // could possibly make the target size.
    let lower_bound = smallest_candidate_solution(packages, third);

    for k in lower_bound.. {
        let solutions = packages.iter().copied().combinations(k).filter(|combo| combo.into_iter().sum::<u64>() == third);
        if let Some(min) = solutions.map(|s| s.into_iter().product()).min() {
            return Some(min)
        }
    }
    
    None
}

fn balance(packages: &[u64], partitions: u64) -> Option<u64> {
    let total: u64 = packages.iter().sum();
    let quarter = total / partitions;
    assert_eq!(quarter * partitions, total);

    let lower_bound = smallest_candidate_solution(packages, quarter);
    for k in lower_bound.. {
        let candidates = packages.iter().copied().combinations(k).filter(|combo| combo.iter().sum::<u64>() == quarter);
        // It's pretty wild to me that we don't need to check the other elements
        // for whether they can fit into 2 or 3 equal-sized partitions. I guess
        // the puzzle is designed so they balance without having to go into a
        // 3-partition problem search.
        if let Some(qe) = candidates.map(|v| v.into_iter().product()).min() {
            return Some(qe)
        }
    }

    None
}

fn slice_diff<T: std::cmp::PartialEq + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    a
    .iter()
    .filter_map(|i| {
        if !b.contains(i) {
            Some(i.clone())
        } else {
            None
        }
    })
    .collect()
}

/// This function works on the sample input but not the puzzle because it fails
/// disqualify solutions where the other two thirds of the packages cannot make
/// the desired total. We need to somehow track the de-selected packages and
/// determine if we can half them.
#[allow(dead_code)]
fn greedy_find(total: u64, candidates: &[u64], packages: &[u64], solution: &mut Vec<u64>) -> Option<u64> {
    if total == 0 {
        // The sum of the candidate solution is 1/3 of the original input,
        // but we need to test if the other 2/3s can be cleanly split in half.
        let rest = slice_diff(packages, &solution);
        if is_bisectable(&rest) {
            Some(1)
        } else {
            //println!("Candidate {solution:?} not accepted because other partitions aren't even.");
            None
        }
    } else if candidates.is_empty() {
        None
    } else {
        for i in 0..candidates.len() {
            let candidate = candidates[i];
            if candidate <= total {
                solution.push(candidate);
                //println!("Consider candidate {candidate} towards solution in {solution:?}.");
                if let Some(qe) = greedy_find(total - candidate, &candidates[i + 1..], packages, solution) {
                    return Some(qe * candidate);
                }
                solution.pop();
            } else {
                //println!("Candidate {} skipped ({} would exceed target with {solution:?}).", candidate, solution.iter().sum::<u64>() + candidate)
            }
        }
        None
    }
}

fn is_bisectable(x: &[u64]) -> bool {
    //println!("Check others in {x:?}");
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
            let total = partition.iter().fold(0, |acc,x| acc + **x);
            if total == half {
                //let product = partition.iter().fold(1, |acc, x| acc * **x);
                //println!("Partition 2: {:?} (sum = {}, product = {}).",
                //    partition,
                //    total,
                //    product);

                //let p3: Vec<_> = x.iter().filter(|x| !partition.contains(x)).collect();
                //let total = p3.iter().fold(0, |acc, x| acc + **x);
                //let product = p3.iter().fold(1, |acc, x| acc * **x);
                //println!("Partition 3: {:?} (sum = {}, product = {}).",
                //    p3,
                //    total,
                //    "huge");

                return true;
            }
        }
    }
    false
}

/// Idk why we didn't consider the goal value before. It's an obvious optimization:
/// don't continue the depth-first search down a path where x, y, or z has exceeded
/// the target value.
/// 
/// Anything you can do to prune these trees can help!
/// 
/// Update: sad face, we don't even need this! Apparently **all** of the slices
/// are somehow 3-partitionable.
#[allow(dead_code)]
fn is_3_partitionable<T: std::cmp::PartialEq + Add<Output = T> + Copy + Display + Debug + PartialOrd>(a: &[T], x: T, y: T, z: T, goal: T) -> bool {
    if a.is_empty() {
        return x == y && y == z
    } else {
        (x + a[0] <= goal && is_3_partitionable(&a[1..], x+a[0], y, z, goal)) ||
        (y + a[0] <= goal && is_3_partitionable(&a[1..], x, y+a[0], z, goal)) ||
        (z + a[0] <= goal && is_3_partitionable(&a[1..], x, y, z+a[0], goal))
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
        let mut packages: Vec<u64> = SAMPLE.lines().flat_map(str::parse).collect();
        packages.sort();
        packages.reverse();
        
        //assert_eq!(part1(&packages), Some(99));
        assert_eq!(balance(&packages, 3), Some(99))
    }

    
    #[test]
    fn test2() {
        let mut packages: Vec<u64> = SAMPLE.lines().flat_map(str::parse).collect();
        packages.sort();
        packages.reverse();
        
        assert_eq!(balance(&packages, 4), Some(44))
    }
}

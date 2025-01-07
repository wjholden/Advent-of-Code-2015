fn main() {
    let puzzle = include_str!("../../puzzles/day17.txt").trim();
    let (part1, part2) = solve(150, &puzzle);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn combinations(total: u8, solution: &mut Vec<u8>, solutions: &mut Vec<Vec<u8>>, containers: &[u8]) -> u64 {
    if total == 0 {
        solutions.push(solution.clone());
        1
    } else if containers.len() == 0 {
        0
    } else {
        let head = containers[0];
        if head <= total {
            solution.push(head);
            let left = combinations(total - head, solution, solutions,&containers[1..]);
            solution.pop();
            let right = combinations(total, solution, solutions,&containers[1..]);
            left + right
        } else {
            combinations(total, solution, solutions,&containers[1..])
        }
    }
}

fn solve(total: u8, input: &str) -> (u64, usize) {
    let containers: Vec<u8> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut solutions = vec![];
    let part1 = combinations(total, &mut vec![], &mut solutions, &containers);
    let min = solutions.iter().map(Vec::len).min().unwrap();
    let part2 = solutions.into_iter().filter(|v| v.len() == min).count();
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
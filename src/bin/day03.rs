use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let puzzle = fs::read_to_string("puzzles/day03.txt").unwrap();

    println!("Part 1: {}", part1(&puzzle));
    println!("Part 2: {}", part2(&puzzle));
}

fn part1(puzzle: &str) -> usize {
    let mut locations = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    locations.insert((0,0), 1);

    for c in puzzle.chars() {
        match c {
            'v' => y -= 1,
            '^' => y += 1,
            '<' => x -= 1,
            '>' => x += 1,
            _ => panic!("part1 wrong"),
        }

        let point = (x, y);
        let count = locations.entry(point).or_insert(0);
        *count += 1;
    }
    locations.len()
}

fn part2(puzzle: &str) -> usize {
    let mut locations = HashSet::new();
    let mut sx = 0;
    let mut sy = 0;
    let mut rx = 0;
    let mut ry = 0;

    locations.insert((0, 0));

    for (i, c) in puzzle.chars().enumerate() {
        let x = if i % 2 == 0 { &mut sx } else { &mut rx };
        let y = if i % 2 == 0 { &mut sy } else { &mut ry };
        match c {
            'v' => *y -= 1,
            '^' => *y += 1,
            '<' => *x -= 1,
            '>' => *x += 1,
            _ => panic!("part2 wrong"),
        }
        let (x, y) = (*x, *y);
        locations.insert((x, y));
    }
    locations.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(">"), 2);
        assert_eq!(part1("^>v<"), 4);
        assert_eq!(part1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("^v"), 3);
        assert_eq!(part2("^>v<"), 3);
        assert_eq!(part2("^v^v^v^v^v"), 11);
    }
}

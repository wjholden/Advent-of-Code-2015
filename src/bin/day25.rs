use regex::Regex;

fn main() {
    let puzzle = include_str!("../../puzzles/day25.txt").trim();
    let (a, b) = bounds(puzzle);
    println!("Part 1: {}", part1(a, b));
}

fn part1(a: u64, b: u64) -> u64 {
    let mut row = 1;
    let mut col = 1;
    let mut x = 20151125;

    while !(row == a && col == b) {
        x = (x * 252533) % 33554393;
        if row == 1 {
            row = col + 1;
            col = 1;
        } else {
            col += 1;
            row -= 1;
        }
    }

    x
}

fn bounds(input: &str) -> (u64, u64) {
    let re = Regex::new(r"\d+").unwrap();
    let mut it = re.captures_iter(input);
    // Sigh. Sometimes Rust reminds me of Java...
    let a = it.next().unwrap().get(0).unwrap().as_str().parse().unwrap();
    let b = it.next().unwrap().get(0).unwrap().as_str().parse().unwrap();
    (a, b)
}

#[cfg(test)]
mod day25 {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(6, 6), 27995004)
    }
}

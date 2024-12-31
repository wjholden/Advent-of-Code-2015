use std::process::Command;

fn main() {
    let puzzle = include_str!("../../puzzles/day12.json").trim();
    println!("Part 1: {}", part1(&puzzle));
    println!("Part 2: {}", part2("puzzles/day12.json"));
}

fn part1(input: &str) -> i64 {
    let re = regex::Regex::new(r"-?\d+").unwrap();
    re.captures_iter(input).map(|captures| {
        (&captures[0]).parse::<i64>().unwrap()
    }).sum()
}

fn part2(input: &str) -> String {
    let out = Command::new("julia")
        .arg("day12.jl")
        .arg(input)
        .output().unwrap();
    String::from_utf8(out.stdout).unwrap()
}

#[cfg(test)]
mod day12 {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1("[1,2,3]"), 6);
        assert_eq!(part1("{\"a\":2,\"b\":4}"), 6);
        assert_eq!(part1("[[[3]]]"), 3);
        assert_eq!(part1("{\"a\":{\"b\":4},\"c\":-1}"), 3);
        assert_eq!(part1("{\"a\":[-1,1]}"), 0);
        assert_eq!(part1("[-1,{\"a\":1}]"), 0);
        assert_eq!(part1("[]"), 0);
        assert_eq!(part1("{}"), 0);
    }
 
    #[test]
    fn test2() {
        assert_eq!(part2("[1,2,3]"), "6");
        assert_eq!(part2("[1,{\"c\":\"red\",\"b\":2},3]"), "4");
        assert_eq!(part2("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"), "0");
        assert_eq!(part2("[1,\"red\",5]"), "6");
    }   
}
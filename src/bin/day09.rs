use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let puzzle = include_str!("../../puzzles/day09.txt");
    println!("Part 1: {}", min_cost(&puzzle));
    println!("Part 2: {}", max_cost(&puzzle));
}

fn vertices(s: &str) -> HashSet<&str> {
    let mut v = HashSet::new();
    for line in s.lines() {
        let words = line.split(" ").collect::<Vec<&str>>();
        v.insert(words[0]);
        v.insert(words[2]);
    }
    v
}

fn edges(s: &str) -> HashMap<(&str, &str), u64> {
    let mut e = HashMap::new();
    for line in s.lines() {
        let words = line.split(" ").collect::<Vec<&str>>();
        let u = words[0];
        let v = words[2];
        let w = words[4].parse::<u64>().unwrap();
        e.insert((u, v), w);
        e.insert((v, u), w);
    }
    e
}

fn min_cost(s: &str) -> u64 {
    let v = vertices(s);
    let e = edges(s);
    let n = v.len();
    v.into_iter()
        .permutations(n)
        .fold(u64::MAX, |acc, path| cmp::min(acc, cost(path, &e)))
}

fn max_cost(s: &str) -> u64 {
    let v = vertices(s);
    let e = edges(s);
    let n = v.len();
    v.into_iter()
        .permutations(n)
        .fold(u64::MIN, |acc, path| cmp::max(acc, cost(path, &e)))
}

fn cost(path: Vec<&str>, e: &HashMap<(&str, &str), u64>) -> u64 {
    // OK, I wanted to use a clever std:iter::zip here, but sometimes a manual
    // loop is easier to understand.
    let mut length = 0;
    for i in 1..path.len() {
        length += e.get(&(path[i - 1], path[i])).unwrap();
    }
    length
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn count_inputs() {
        assert_eq!(3, vertices(EXAMPLE).len())
    }

    #[test]
    fn count_edges() {
        assert_eq!(2 * 3, edges(EXAMPLE).len())
    }

    #[test]
    fn london_dublin_belfast() {
        assert_eq!(605, min_cost(EXAMPLE))
    }

    #[test]
    fn dublin_london_belfast() {
        assert_eq!(982, max_cost(EXAMPLE))
    }
}

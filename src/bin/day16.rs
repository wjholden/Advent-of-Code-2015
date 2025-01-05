use lazy_static::lazy_static;
use regex::{CaptureMatches, Regex};
use std::{collections::HashMap, error::Error};

enum Part {
    One,
    Two,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1: {}", solve(Part::One)?);
    println!("Part 2: {}", solve(Part::Two)?);
    Ok(())
}

fn sues() -> CaptureMatches<'static, 'static> {
    lazy_static! {
        static ref re: Regex =
            Regex::new(r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)").unwrap();
    }
    re.captures_iter(include_str!("../../puzzles/day16.txt"))
}

fn solve(part: Part) -> Result<&'static str, Box<dyn Error>> {
    let constraints: HashMap<&str, u8> = parse_constraints();
    'outer: for cap in sues() {
        let n = cap.get(1).unwrap().as_str();
        let k1 = cap.get(2).unwrap().as_str();
        let v1 = cap.get(3).unwrap().as_str().parse()?;
        let k2 = cap.get(4).unwrap().as_str();
        let v2 = cap.get(5).unwrap().as_str().parse()?;
        let k3 = cap.get(6).unwrap().as_str();
        let v3 = cap.get(7).unwrap().as_str().parse()?;

        for (k, v) in [(k1, v1), (k2, v2), (k3, v3)] {
            let constraint_v = *constraints.get(k).unwrap();
            if !match (&part, k) {
                (Part::Two, "cats" | "trees") => constraint_v < v,
                (Part::Two, "pomeranians" | "goldfish") => constraint_v > v,
                _ => constraint_v == v,
            } {
                continue 'outer;
            }
        }

        // If we made it here then we found the right Sue.
        return Ok(n);
    }
    // https://stackoverflow.com/a/55125216/5459668
    // Could also do Err("message".into())
    Err("No Sue found matching constraints")?
}

fn parse_constraints() -> HashMap<&'static str, u8> {
    CONSTRAINTS
        .lines()
        .map(|line| {
            let mut s = line.split(": ");
            (s.next().unwrap(), s.next().unwrap().parse().unwrap())
        })
        .collect()
}

const CONSTRAINTS: &str = "children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1";

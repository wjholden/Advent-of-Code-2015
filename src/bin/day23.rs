use std::{collections::BTreeMap, str::SplitAsciiWhitespace};

fn main() {
    let puzzle = include_str!("../../puzzles/day23.txt").trim();
    let registers = run(puzzle, Part::One);
    println!("Part 1: {}", registers.get("b").unwrap());
    let registers = run(puzzle, Part::Two);
    println!("Part 2: {}", registers.get("b").unwrap());
}

enum Part {
    One,
    Two
}

fn run(input: &str, part: Part) -> BTreeMap<&str, usize> {
    let input = input.replace(",", "");
    let instructions: Vec<Vec<&str>> = input
        .lines()
        .map(str::split_ascii_whitespace)
        .map(SplitAsciiWhitespace::collect)
        .collect();
    let mut registers = BTreeMap::new();
    registers.insert("a", match part {
        Part::One => 0,
        Part::Two => 1,    
    });
    registers.insert("b", 0);
    let mut i = 0i32;
    while i < instructions.len() as i32 {
        i += match instructions[i as usize].as_slice() {
            ["hlf", r] => {
                *registers.get_mut(*r).unwrap() /= 2;
                1
            },
            ["tpl", r] => {
                *registers.get_mut(*r).unwrap() *= 3;
                1
            },
            ["inc", r] => {
                *registers.get_mut(*r).unwrap() += 1;
                1
            },
            ["jmp", offset] => {
                offset.parse().unwrap()
            },
            ["jie", r, offset] => {
                if *registers.get(*r).unwrap() % 2 == 0 {
                    offset.parse().unwrap()
                } else {
                    1
                }
            },
            ["jio", r, offset] => {
                if *registers.get(*r).unwrap() == 1 {
                    offset.parse().unwrap()
                } else {
                    1
                }
            },
            _ => unreachable!()
        }
    }
    registers
}

#[cfg(test)]
mod day23 {
    use super::*;

    const SAMPLE: &str = "inc a
jio a, +2
tpl a
inc a";

    #[test]
    fn skip_tpl() {
        let registers = run(SAMPLE, Part::One);
        assert_eq!(*registers.get("a").unwrap(), 2);
        assert_eq!(*registers.get("b").unwrap(), 0);
    }
}

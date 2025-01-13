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
        let instruction = instructions[i as usize][0];
        let param = instructions[i as usize][1];

        match instruction {
            "hlf" => {
                *registers.get_mut(param).unwrap() /= 2;
                i += 1;
            }
            "tpl" => {
                *registers.get_mut(param).unwrap() *= 3;
                i += 1;
            },
            "inc" => {
                *registers.get_mut(param).unwrap() += 1;
                i += 1;
            },
            "jmp" => {
                i += param.parse::<i32>().unwrap();
            },
            "jie" => {
                i += if *registers.get(param).unwrap() % 2 == 0 {
                    instructions[i as usize][2].parse().unwrap()
                } else {
                    1
                };
            },
            "jio" => {
                i += if *registers.get(param).unwrap() == 1 {
                    instructions[i as usize][2].parse().unwrap()
                } else {
                    1
                };
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

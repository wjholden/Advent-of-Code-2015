use std::collections::HashSet;
use futures::executor::block_on;
use itertools::Itertools;

fn main() {
    let puzzle = include_str!("../../puzzles/day18.txt").trim();
    let part1 = solve(puzzle, 100, Part::One);
    let part2 = solve(puzzle, 100, Part::Two);
    println!("{}", block_on(part1));
    println!("{}", block_on(part2));
}

#[derive(PartialEq)]
enum Part{
    One,
    Two
}

async fn solve(input: &str, steps: u8, part: Part) -> usize {
    let size = input.trim().lines().count() as i8;
    let mut state = input.lines().enumerate().fold(HashSet::new(), 
    |mut acc, (i,line)| {
        for j in 0..line.len() {
            if line.chars().nth(j).unwrap() == '#' {
                acc.insert((i as i8, j as i8));
            }
        }
        acc
    });
    if part == Part::Two {
        state.insert((0,0));
        state.insert((0,size-1));
        state.insert((size-1,0));
        state.insert((size-1,size-1));
    }
    //println!("Initial");
    //show(&state, size);
    for _iterations in 1..=steps {
        state = next_state(&state, size);
        if part == Part::Two {
            state.insert((0,0));
            state.insert((0,size-1));
            state.insert((size-1,0));
            state.insert((size-1,size-1));
        }
        //println!("After {_iterations} steps:");
        //show(&state, size);
    }
    state.len()
}

fn next_state(state: &HashSet<(i8, i8)>, size: i8) -> HashSet<(i8, i8)> {
    (0..size).cartesian_product(0..size).filter(|&(i,j)| {
        let mut neighbors = 0;
            for (dr,dc) in [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)] {
                if state.contains(&(i+dr,j+dc)) {
                    neighbors += 1;
                }
            }
            (state.contains(&(i,j)) && (neighbors==2||neighbors==3)) ||
            (!state.contains(&(i,j)) && neighbors==3)
    }).collect()
}

#[allow(dead_code)]
fn show(state: &HashSet<(i8, i8)>, size: i8) {
    for i in 0..size {
        for j in 0..size {
            if state.contains(&(i,j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }    
}

#[cfg(test)]
mod day18 {
    use super::*;

    const SAMPLE: &str = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";

    #[test]
    fn test1() {
        assert_eq!(block_on(solve(SAMPLE, 4, Part::One)), 4)
    }
 
    #[test]
    fn test2() {
        assert_eq!(block_on(solve(SAMPLE, 5, Part::Two)), 17)
    }   
}

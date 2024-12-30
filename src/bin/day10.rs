// https://oeis.org/A005150
fn main() {
    let puzzle = include_str!("../../puzzles/day10.txt");
    println!("Part 1: {}", iterate(&puzzle, 40));
    println!("Part 2: {}", iterate(&puzzle, 50));
}

fn iterate(input: &str, iterations: u8) -> usize {
    let mut x: Vec<u32> = input.trim().chars().map(|c| c.to_digit(10).unwrap()).collect();
    for _ in 1..=iterations {
        x = look_and_say(x);
        //println!("{x:?}");
    }
    x.len()
}

fn look_and_say(x: Vec<u32>) -> Vec<u32> {
    let mut y = vec![];
    let mut iter = x.iter().peekable();
    let mut count = 0;
    while let Some(&i) = iter.next() {
        match iter.peek() {
            Some(&&j) if j == i => {
                count += 1
            },
            _ => {
                count += 1;
                y.push(count);
                y.push(i);
                count = 0;
            }
        }
    }
    y
}

#[cfg(test)]
mod day10 {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test() {
        let mut x = vec![1];
        
        x = look_and_say(x);
        assert_eq!(x, [1,1]);
        
        x = look_and_say(x);
        assert_eq!(x, [2,1]);

        x = look_and_say(x);
        assert_eq!(x, [1,2,1,1]);

        x = look_and_say(x);
        assert_eq!(x, [1,1,1,2,2,1]);

        x = look_and_say(x);
        assert_eq!(x, [3,1,2,2,1,1]);
    }  
}
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nom::{branch::alt, bytes::complete::{tag, take_until, take_while}, multi::many0, AsChar, IResult};

fn main() {
    let puzzle = include_str!("../../puzzles/day13.txt").trim();
    println!("Part 1: {}", part1(&puzzle));
    println!("Part 2: {}", part2(&puzzle));
}

fn part1(input: &str) -> i32 {
    let relations = parse(input);
    let names: HashSet<&str> = relations.iter().map(|((u,_),_)| *u).collect();
    let n = names.len();
    names.into_iter().permutations(n).map(|names| happiness(&names, &relations)).max().unwrap()
}

fn part2(input: &str) -> i32 {
    let relations = parse(input);
    let names: HashSet<&str> = relations.iter().map(|((u,_),_)| *u).collect();
    let n = names.len();
    names.into_iter().permutations(n).map(|mut name_permutation| {
        // We can make this substantially faster by only inserting ourselves at
        // the head of the table. Our original set of permutations was actually
        // very wasteful. The *circular* order of a,b,c = b,c,a = c,a,b. The
        // basic permutations function doesn't make any effort to de-duplicate.
        // The optimization doesn't go very far: we can only fix one element.
        // For four elements, we can fix the first element and still get
        // 1) a,b,c,d
        // 2) a,b,d,c
        // 3) a,c,b,d
        // 4) a,c,d,b
        // 5) a,d,b,c
        // 6) a,d,c,b
        // In general, if the number of permutations is n!, then the number of
        // *circuluar permutations* is (n-1)!.
        // https://mathworld.wolfram.com/CircularPermutation.html
        // https://www.reddit.com/r/adventofcode/comments/3wm0oy/comment/cxx80pp/
        name_permutation.push("me");    
        happiness(&name_permutation, &relations)
    }).max().unwrap()
}

fn happiness(names: &[&str], relations: &HashMap<(&str, &str), i32>) -> i32 {
    let mut h = 0;
    let n = names.len();
    for i in 0..n {
        let j = (i + 1) % n;
        h += relations.get(&(names[i], names[j])).cloned().unwrap_or(0);
        h += relations.get(&(names[j], names[i])).cloned().unwrap_or(0);
    }
    h
}

fn parse(input: &str) -> HashMap<(&str, &str), i32> {
    let (_, v) = many0(parse_relation)(input).unwrap();
    v.into_iter().map(|(u,v,dh)| ((u,v),dh)).collect()
}

fn parse_relation(input: &str) -> IResult<&str, (&str, &str, i32)> {
    let (input, name1) = take_until(" ")(input)?;
    let (input, _) = tag(" would ")(input)?;
    let (input, direction) = alt((gain, lose))(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, amount) = take_while(|c: char| c.is_dec_digit())(input)?;
    let amount = amount.parse::<i32>().unwrap();
    let (input, _) = tag(" happiness units by sitting next to ")(input)?;
    let (input, name2) = take_until(".")(input)?;
    let (input, _) = alt((tag(".\n"), tag(".")))(input)?;
    Ok((input, (name1, name2, direction * amount)))
}

fn gain(input: &str) -> IResult<&str, i32> {
    let (input, _) = tag("gain")(input)?;
    Ok((input, 1))
}

fn lose(input: &str) -> IResult<&str, i32> {
    let (input, _) = tag("lose")(input)?;
    Ok((input, -1))
}

#[cfg(test)]
mod day13 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE), 330)
    } 
}
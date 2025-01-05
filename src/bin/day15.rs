use itertools::{any, Itertools};
use nalgebra::{DMatrix, DVector};
use regex::Regex;
use advent_of_code_2015::ConstantSumSeq;

fn main() {
    let puzzle = include_str!("../../puzzles/day15.txt").trim();
    println!("Part 1: {}", part1(&puzzle));
    println!("Part 2: {}", part2(&puzzle));
}

fn part1(input: &str) -> i32 {
    let a = parse_properties(input);
    let ingredients = a.ncols();
    let mut x = DVector::repeat(ingredients, 100/(ingredients as i32));
    print!("{a}");

    loop {
        let score: i32 = (&a * &x).into_iter().filter(|&&p| p >= 0).product();

        let mut max_move = DVector::repeat(ingredients, 0);
        let mut max_ds = 0;

        // Ok...sorry.
        // This sets up a list of zeros chained to +1 and -1. We take the permutation of this
        // to get a series of moves (gradients?) from our current position.
        // We're going to move in the direction of the largest increase.
        // Skip anything that goes into a negative value.
        // Since we are always adding 1 and subtracting 1, there is no possibility that
        // the total will go below 100.
        let moves = [-1,1].into_iter().chain(vec![0; ingredients-2]).permutations(ingredients);
        for dx in moves {
            let dx = DVector::from_iterator(ingredients, dx);
            let new_x = &x + dx;
            if any(new_x.iter(), |&i| i < 0) {
                continue
            }
            let ds = (&a * &new_x).into_iter().filter(|&&p| p >= 0).product();
            if ds > max_ds {
                max_move = new_x;
                max_ds = ds;
            }
        }
        if max_ds > score {
            x = max_move;
        } else {
            print!("{x}");
            return score
        }
    }
}

fn part2(input: &str) -> i32 {
    // Ok, I've run out of cleverness for this one. I don't know if there is
    // any efficient way to waltz up to the solution quickly. Our gradient ascent
    // method we used before might walk right past an optimial solution without
    // us knowing it.
    let a = parse_properties(input);
    let ingredients = a.ncols();
    let c = parse_calories(input);

    let mut max_score = 0;
    let mut best_recipe = DVector::zeros(ingredients);
    
    // You would think that Rayon's par_bridge would make this faster by
    // parallelization, but experimentally no -- our solution went from ~250ms
    // single-threaded to ~350ms multi-threaded.
    for x in ConstantSumSeq::new(ingredients, 100) {
        assert_eq!(x.iter().sum::<i32>(), 100); // it's supposed to be...
        assert_eq!(x.len(), ingredients);

        let x = DVector::from_vec(x);
        if x.dot(&c) != 500 {
            continue
        }

        let score = (&a * &x).into_iter().filter(|&&p| p >= 0).product();
        if score > max_score {
            max_score = score;
            best_recipe = x;
        }
    }

    print!("{best_recipe}");
    max_score
}

fn parse_properties(input: &str) -> DMatrix<i32> {
    let re = Regex::new(r"capacity (?P<a>-?\d+), durability (?P<b>-?\d+), flavor (?P<c>-?\d+), texture (?P<d>-?\d+),").unwrap();
    let properties: Vec<i32> = re.captures_iter(input).map(|cap| {
        [cap["a"].parse().unwrap(), cap["b"].parse().unwrap(), cap["c"].parse().unwrap(), cap["d"].parse().unwrap()]
    }).flatten().collect_vec();
    let ningredients = properties.len()/4;
    let a = DMatrix::from_iterator(4, ningredients, properties.into_iter());
    a
}

fn parse_calories(input: &str) -> DVector<i32> {
    let re = Regex::new(r"calories (?P<calories>\d+)").unwrap();
    let mut nrows = 0;
    let c = re.captures_iter(input).map(|cap| {
        nrows += 1;
        cap["calories"].parse().unwrap()
    }).collect();
    DVector::from_vec( c)
}

#[cfg(test)]
mod day15 {
    use super::*;

    const SAMPLE: &str = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE), 62842880)
    }
 
    #[test]
    fn test2() {
        assert_eq!(part2(SAMPLE), 57600000)
    }   
}
use std::cmp;
use std::fs;

fn main() {
    let puzzle = fs::read_to_string("day02.txt").unwrap();

    println!("Part 1: {}", part1(&puzzle));

    // Part 2 endeavors for a more functional approach. We'll take the lines
    // (which is delightfully easy in Rust, thanks to the `lines()` function!),
    // then convert the line to an integer vector. We'll use a custom function
    // get the smallest and next-to-smallest values from the vector.
    // Finally, we'll use a one-liner with `fold` to take the product (which
    // would have been better with `Vec.iter().product()`, but too late).
    println!("Part 2: {}", part2(&puzzle));

    // Remember the magical `dbg!` macro for easy debugging. Remember also
    // that you can sprinkle `assert!` tests wherever. Finally, borrowship rules
    // apply to both `dbg!` and `assert!`, so in general use a pointer (`&`) to
    // maintain ownership.
}

fn part1(input: &str) -> u64 {
    let mut area = 0;
    for line in input.trim().lines() {
        let xyz: Vec<&str> = line.split('x').collect();
        assert_eq!(xyz.len(), 3);
        let (x, y, z) = (xyz[0], xyz[1], xyz[2]);
        let (x, y, z): (u64, u64, u64) = (
            str::parse(x).unwrap(),
            str::parse(y).unwrap(),
            str::parse(z).unwrap(),
        );
        let slack = cmp::min(x * y, cmp::min(y * z, x * z));
        area += 2 * (x * y + x * z + y * z) + slack;
    }
    area
}

fn part2(input: &str) -> u64 {
    let mut ribbon = 0;
    let lines = input.trim().lines();
    for line in lines {
        let sides: Vec<u64> = line.split('x').map(|s| s.parse().unwrap()).collect();
        //dbg!(&sides);
        assert!(sides.len() == 3);
        let (a, b) = min2(&sides);
        let perimeter = 2 * (a + b);
        let volume: u64 = sides.iter().product();
        ribbon += perimeter + volume;
    }
    ribbon
}

fn min2(vec: &Vec<u64>) -> (u64, u64) {
    let mut a = std::u64::MAX;
    let mut b = std::u64::MAX;
    for i in vec {
        let i = *i;
        if i < a {
            (a, b) = (i, a);
        } else if i < b {
            b = i;
        }
    }
    return (a, b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("2x3x4"), 58);
        assert_eq!(part1("1x1x10"), 43);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("2x3x4"), 34);
        assert_eq!(part2("1x1x10"), 14);
    }
}
use std::fs;

fn main() {
    let puzzle = fs::read_to_string("puzzles/day01.txt").unwrap();

    let (part1, part2) = floor(&puzzle);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn floor(instructions: &str) -> (i32, i32) {
    let mut position = 0;
    let mut i = 0;
    let mut basement = -1;

    for c in instructions.chars() {
        i += 1;

        if c == '(' {
            position += 1;
        } else if c == ')' {
            position -= 1;
        }

        if position == -1 && basement < 0 {
            basement = i;
        }
    }

    (position, basement)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let expect = 0;
        for s in ["(())", "()()"] {
            assert_eq!(floor(s).0, expect, "{s}");
        }
    }

    #[test]
    fn test2() {
        let expect = 3;
        for s in ["(((", "(()(()(", "))((((("] {
            assert_eq!(floor(s).0, expect, "{s}");
        }
    }

    #[test]
    fn test3() {
        let expect = -1;
        for s in ["())", "))("] {
            assert_eq!(floor(s).0, expect, "{s}");
        }
    }

    #[test]
    fn test4() {
        let expect = -3;
        for s in [")))", ")())())"] {
            assert_eq!(floor(s).0, expect, "{s}");
        }
    }

    #[test]
    fn test5() {
        let input = ")";
        assert_eq!(floor(input).1, 1, "{input}");
    }

    #[test]
    fn test6() {
        let input = "()())";
        assert_eq!(floor(input).1, 5, "{input}");
    }
}
fn main() {
    let puzzle = include_str!("../../puzzles/day11.txt").trim();
    let part1 = next_password(puzzle);
    let part2 = next_password(&part1);
    println!("Part 1: {}", part1); // cqjxppqrr wrong
    println!("Part 2: {}", part2);
}

fn next_password(input: &str) -> String {
    let mut s = String::from(input);
    loop {
        s = increment(&s);
        if is_secure(&s) {
            break
        }
    }
    s
}

fn increment(input: &str) -> String {
    let mut s = input.chars().collect::<Vec<_>>();
    let mut overflow = true;
    for j in (0..s.len()).rev() {
        match (overflow, s[j]) {
            (true, 'z') => s[j] = 'a',
            (true, notz) => {
                s[j] = char::from_u32(notz as u32 + 1).unwrap();
                overflow = false;
            },
            (false, _) => (),
        }
    }
    s.iter().collect()
}

fn is_secure(input: &str) -> bool {
    increasing_straight(input) &&
    unambiguous(input) &&
    two_pairs(input)
}

fn increasing_straight(input: &str) -> bool {
    let c: Vec<_> = input.bytes().collect();
    for i in 2..c.len() {
        if c[i-2] + 2 == c[i-1] + 1 && c[i-1] + 1 == c[i] {
            return true
        }
    }
    false
}

fn unambiguous(input: &str) -> bool {
    !input.contains("i") &&
    !input.contains("o") &&
    !input.contains("l")
}

fn two_pairs(input: &str) -> bool {
    let mut pairs = 0;
    let mut s = input.chars().peekable();

    while let (Some(i), Some(&j)) = (s.next(), s.peek()) {
        if i == j {
            pairs += 1;
            _ = s.next(); // feels like a trap, can this greedy approach fail?
        };
    }

    pairs >= 2
}

#[cfg(test)]
mod day11 {
    use super::*;

    #[test]
    fn increments() {
        assert_eq!(increment("xx"), "xy");
        assert_eq!(increment("xy"), "xz");
        assert_eq!(increment("xz"), "ya");
        assert_eq!(increment("ya"), "yb");
    }

    #[test]
    fn hijklmmn() {
        assert_eq!(is_secure("hijklmmn"), false)
    }

    #[test]
    fn abbceffg() {
        assert_eq!(is_secure("abbceffg"), false)
    }

    #[test]
    fn abbcegjk() {
        assert_eq!(is_secure("abbcegjk"), false)
    }
 
    #[test]
    fn abcdefgh() {
        assert_eq!(next_password("abcdefgh"), "abcdffaa")
    }   

    #[test]
    fn ghijklmn() {
        assert_eq!(next_password("ghijklmn"), "ghjaabcc")
    }

    #[test]
    fn abcdefyz() {
        assert_eq!(increment("abcdefyz"), "abcdefza")
    }

    #[test]
    fn ayyyyz() {
        assert_eq!(increment("ayyyyz"), "ayyyza")
    }
}
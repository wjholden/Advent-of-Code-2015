fn main() {
    let puzzle = std::fs::read_to_string("day08.txt").unwrap();
    let string_length: usize = puzzle.lines().map(count).sum();
    let memory_length: usize = puzzle.lines().map(strlen).sum();
    let encode_length: usize = puzzle.lines().map(encoded_length).sum();
    println!("Part 1: {}", memory_length - string_length);
    println!("Part 2: {}", encode_length - memory_length);
}

fn strlen(s: &str) -> usize {
    s.chars().count()
}

fn count(s: &str) -> usize {
    let mut n = 0;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        n += match c {
            '"' => 0,
            '\\' => {
                match chars.peek() {
                    Some('\\') | Some('"') => {
                        chars.next();
                    },
                    Some('x') => {
                        chars.next(); // x
                        chars.next(); // 1st hex symbol
                        chars.next(); // 2nd hex symbol
                    },
                    Some(unexpected) => {
                        eprint!("Unexpected escape sequence: {c}{unexpected}");
                        panic!("Unexpected escape sequnce")
                    },
                    None => panic!("Unexpected end of input after escape.")
                };
                1
            },
            _ => 1
        }
    }
    n
}

fn encoded_length(s: &str) -> usize {
    // Isn't this just 2 + length + count(") + count(\)?
    let base_length = s.chars().count();
    let backslashes = s.matches('\\').count();
    let quotes = s.matches('"').count();
    2 + base_length + backslashes + quotes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let s = r#""""#;
        let n = 0;
        assert_eq!(n, count(s))
    }

    #[test]
    fn abc() {
        let s = r#""abc""#;
        let n = 3;
        assert_eq!(n, count(s))
    }

    #[test]
    fn aaa_aaa() {
        let s = r#""aaa\"aaa""#;
        let n = 7;
        assert_eq!(n, count(s))
    }

    #[test]
    fn x27() {
        let s = r#""\x27""#;
        let n = 1;
        assert_eq!(n, count(s))
    }

    #[test]
    fn empty_encoded() {
        let s = r#""""#;
        let n = 6;
        assert_eq!(n, encoded_length(s));
    }

    #[test]
    fn abc_encoded() {
        let s = r#""abc""#;
        let n = 9;
        assert_eq!(n, encoded_length(s));
    }

    #[test]
    fn aaa_aaa_encoded() {
        let s = r#""aaa\"aaa""#;
        let n = 16;
        assert_eq!(n, encoded_length(s));
    }

    #[test]
    fn x27_encoded() {
        let s = r#""\x27""#;
        let n = 11;
        assert_eq!(n, encoded_length(s));
    }
}
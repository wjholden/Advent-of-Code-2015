use regex::Regex;

fn main() {
    let lines = read_input();
    let n1: u64 = lines
        .iter()
        .map(|line| if nice1(line) { 1 } else { 0 })
        .sum();
    println!("Part 1: {n1}");
    let n2: u64 = lines
        .iter()
        .map(|line| if nice2(line) { 1 } else { 0 })
        .sum();
    println!("Part 2: {n2}");
}

fn nice1(s: &str) -> bool {
    contains_three_vowels(s) && contains_double(s) && contains_no_disallowed(s)
}

fn nice2(s: &str) -> bool {
    contains_non_overlapping_double(s) && contains_repeat_with_middle(s)
}

fn contains_three_vowels(s: &str) -> bool {
    let mut count = 0;
    for letter in s.chars() {
        match letter {
            'a' | 'e' | 'i' | 'o' | 'u' => count += 1,
            _ => continue,
        }
    }
    count >= 3
}

fn contains_double(s: &str) -> bool {
    let re = Regex::new(
        r"aa|bb|cc|dd|ee|ff|gg|hh|ii|jj|kk|ll|mm|nn|oo|pp|qq|rr|ss|tt|uu|vv|ww|xx|yy|zz",
    )
    .unwrap();
    re.is_match(s)
}

fn contains_no_disallowed(s: &str) -> bool {
    let re = Regex::new(r"ab|cd|pq|xy").unwrap();
    !re.is_match(s)
}

fn contains_non_overlapping_double(s: &str) -> bool {
    // Julia and R ruined me. Now, these zero-based indices
    // and half-closed intervals are really tricky for me.
    let n = s.len();
    for i in 0..=(n - 2) {
        let left = &s[i..=(i + 1)];
        for j in (i + 2)..=(n - 2) {
            let right = &s[j..=(j + 1)];
            assert_eq!(left.len(), 2);
            assert_eq!(right.len(), 2);
            if left == right {
                return true;
            }
        }
    }

    false
}

fn contains_repeat_with_middle(s: &str) -> bool {
    // Rust makes strings really difficult. I suppose it's all for the best, though.
    // After all, with UTF-8, strings are actually much harder than people think.
    let b = s.as_bytes();
    let n = b.len();
    for i in 0..=(n - 3) {
        if b[i] == b[i + 2] {
            return true;
        }
    }
    false
}

// Apparently this is bad:
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_input() -> Vec<String> {
    std::fs::read_to_string("day05.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ugknbfddgicrmopn() {
        assert_eq!(nice1("ugknbfddgicrmopn"), true);
    }

    #[test]
    fn aaa() {
        assert_eq!(nice1("aaa"), true);
    }

    #[test]
    fn jchzalrnumimnmhp() {
        assert_eq!(nice1("jchzalrnumimnmhp"), false);
    }

    #[test]
    fn haegwjzuvuyypxyu() {
        assert_eq!(nice1("haegwjzuvuyypxyu"), false);
    }

    #[test]
    fn dvszwmarrgswjxmb() {
        assert_eq!(nice1("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn qjhvhtzxzqqjkmpb() {
        assert_eq!(nice2("qjhvhtzxzqqjkmpb"), true);
    }

    #[test]
    fn xxyxx() {
        assert_eq!(nice2("xxyxx"), true);
    }

    #[test]
    fn uurcxstgmygtbstg() {
        assert_eq!(nice2("uurcxstgmygtbstg"), false);
    }

    #[test]
    fn ieodomkazucvgmuy() {
        assert_eq!(nice2("ieodomkazucvgmuy"), false);
    }
}

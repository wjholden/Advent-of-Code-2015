use md5::{Digest, Md5};

fn main() {
    let puzzle = include_str!("../../puzzles/day04.txt");
    println!("Part 1: {}", search(puzzle, 5));
    println!("Part 2: {}", search(puzzle, 6));
}

fn search(key: &str, zeros: u8) -> u64 {
    if !(zeros == 5 || zeros == 6) {
        panic!("Can only search for 5 or 6 zeros.");
    }
    let mut i = 0;
    loop {
        if (zeros == 5 && find_5_0s(key, i)) ||
            (zeros == 6 && find_6_0s(key, i)) {
            return i
        } else {
            i += 1;
        }
    }
}

fn find_5_0s(key: &str, suffix: u64) -> bool {
    let mut hasher = Md5::new();
    hasher.update(format!("{key}{suffix}"));
    let result = hasher.finalize();
    result[0..=1] == [0, 0] && result[2] & 0xf0 == 0
}

fn find_6_0s(key: &str, suffix: u64) -> bool {
    let mut hasher = Md5::new();
    hasher.update(format!("{key}{suffix}"));
    let result = hasher.finalize();
    result[0..=2] == [0, 0, 0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abcdef() {
        let input = String::from("abcdef");
        let result = search(&input, 5);
        let expect = 609043;
        assert_eq!(result, expect);
    }

    #[test]
    fn pqrstuv() {
        let input = String::from("pqrstuv");
        let result = search(&input, 5);
        let expect = 1048970;
        assert_eq!(result, expect);
    }
}

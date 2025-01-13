use std::collections::BTreeMap;

fn main() {
    let puzzle = include_str!("../../puzzles/day19.txt").trim();
    println!("Part 1: {}", part1(puzzle));
    println!("Part 2: {}", part2(puzzle)); // 61 is not the right answer.
}

#[derive(Default, Debug)]
struct TrieNode {
    children: BTreeMap<char, TrieNode>,
    terminus: bool,
}

impl TrieNode {
    fn count(&self) -> usize {
        self.children.values().map(|v| v.count()).sum::<usize>() + 
        if self.terminus { 1 } else { 0 }
    }
}

/// You can absolutely call this a premature optimization. I was
/// thinking that we would need to count billions of substitutions
/// in part 2, but my answer was just 509 for part 1 and I don't
/// even need the trie for part 2. Oh well, I did learn how to use
/// `default` for the first time.
///
/// I use a separate Trie structure for the root after reading
/// someone else's code at https://dev.to/timclicks/two-trie-implementations-in-rust-ones-super-fast-2f3m,
/// but honestly I think this isn't necessary.
#[derive(Debug)]
struct Trie {
    root: TrieNode
}

impl Trie {
    fn new() -> Trie {
        Trie {
            root: TrieNode::default(),
        }
    }
    
    fn insert(&mut self, word: &str) {
        let mut position = &mut self.root;
        for c in word.chars() {
            position = position.children.entry(c).or_default();
        }
        position.terminus = true;
    }
    
    fn count(&self) -> usize {
        self.root.count()
    }
}

fn parse(input: &str) -> (BTreeMap<&str, Vec<&str>>, &str) {
    let mut s = input.split("\n\n");
    let rules = s.next().unwrap().lines().map(|line| {
        let mut l = line.split(" => ");
        (l.next().unwrap(), l.next().unwrap())
    }).fold(BTreeMap::new(), |mut acc: BTreeMap<&str, Vec<&str>>, (k,v)| {
       acc.entry(k).or_default().push(v);
       acc
    });
    (rules, s.next().unwrap())
}

fn part1(input: &str) -> usize {
    let (rules, input) = parse(input);
    let mut trie = Trie::new();
    
    for i in 0..input.len() {
        let left = &input[..i];
        let rest = &input[i..];
        //println!("{left} and {rest}");
        for (k,v) in &rules {
            if let Some(right) = rest.strip_prefix(k) {
                for replacement in v {
                    trie.insert(&format!("{left}{replacement}{right}"))
                }
                break
            }
        }
    }
    
    trie.count()
}

fn part2(input: &str) -> usize {
    let (rules, input) = parse(input);
    let mut input = String::from(input);
    let rrules = rules.into_iter().fold(BTreeMap::new(), |mut acc, (k,v)| {
        for value in v {
            acc.insert(value, k);
        }
        acc
    });
    //println!("{rrules:?}");
    
    let mut x = 0;
    while input != "e" {
        // Find the longest matching reverse rule.
        let (k,v) = rrules.iter().filter(|&(k,_v)| input.contains(k)).max_by_key(|(k,_v)| k.len()).unwrap();
        //println!("{:?}, {input}", (k,v));
        x += input.matches(k).count();
        input = input.replace(k, v);
    }
    
    x
}

const _SAMPLE1: &str = "H => HO
H => OH
O => HH

HOH";

const _SAMPLE2: &str = "e => H
e => O
H => HO
H => OH
O => HH

HOHOHO";

#[cfg(test)]
mod day19 {
    use super::*;

    const SAMPLE1: &str = "H => HO
H => OH
O => HH";

    const SAMPLE2: &str = "e => H
e => O
H => HO
H => OH
O => HH";

    #[test]
    fn hoh1() {
        assert_eq!(part1(format!("{SAMPLE1}\n\nHOH").as_str()), 4)
    }

    #[test]
    fn hohoho1() {
        assert_eq!(part1(format!("{SAMPLE1}\n\nHOHOHO").as_str()), 7)
    }

    #[test]
    fn hoh2() {
        assert_eq!(part2(format!("{SAMPLE2}\n\nHOH").as_str()), 3)
    }

    #[test]
    fn hohoho2() {
        assert_eq!(part2(format!("{SAMPLE2}\n\nHOHOHO").as_str()), 6)
    }   
}

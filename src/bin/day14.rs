fn main() {
    let puzzle = include_str!("../../puzzles/day14.txt").trim();
    println!("Part 1: {}", part1(&puzzle, 2503));
    println!("Part 2: {}", part2(&puzzle, 2503));
}

fn part1(input: &str, time: u64) -> u64 {
   parse(input).into_iter().map(|mut deer| deer.fly(time)).max().unwrap()
}

fn part2(input: &str, time: u64) -> u64 {
    let mut reindeer = parse(input);
    for _ in 1..=time {
        let mut leading_distance = 0;
        for deer in reindeer.iter_mut() {
            let d = deer.tick();
            leading_distance = leading_distance.max(d);
        }
        for deer in reindeer.iter_mut() {
            if deer.distance == leading_distance {
                deer.points += 1;
            }
        }
    }
    reindeer.into_iter().map(|deer| deer.points).max().unwrap()
}

#[derive(Debug)]
struct Reindeer {
    speed: u64,
    fly: u64,
    rest: u64,
    distance: u64,
    time: u64,
    points: u64,
}

impl Reindeer {
    fn tick(&mut self) -> u64 {
        let current = self.time % (self.fly + self.rest);
        if current < self.fly {
            self.distance += self.speed;
        }
        self.time += 1;
        self.distance
    }

    fn fly(&mut self, time: u64) -> u64 {
        for _ in 1..=time {
            self.tick();
        }
        self.distance
    }
}

fn parse(input: &str) -> Vec<Reindeer> {
    input.lines().map(|line| {
        let s: Vec<_> = line.split_ascii_whitespace().collect();
        let speed = s[3].parse().unwrap();
        let fly = s[6].parse().unwrap();
        let rest = s[13].parse().unwrap();
        Reindeer { speed, fly, rest, distance: 0, time: 0, points: 0 }
    }).collect()
}

#[cfg(test)]
mod day14 {
    use std::assert_eq;

    use super::*;

    const SAMPLE: &str = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    #[test]
    fn test1() {
        assert_eq!(part1(SAMPLE, 1000), 1120)
    }
 
    #[test]
    fn test2() {
        assert_eq!(part2(SAMPLE, 1000), 689)
    }   
}

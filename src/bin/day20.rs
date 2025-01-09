fn main() {
    let puzzle = include_str!("../../puzzles/day20.txt").trim().parse().unwrap();
    println!("Part 1: {}", solve(puzzle, 1000000, 10, f64::INFINITY).unwrap());
    println!("Part 2: {}", solve(puzzle, 1000000, 11, 50.0).unwrap());
}

fn solve(puzzle: usize, limit: usize, pph: usize, hpe: f64) -> Option<usize> {
    let mut a = vec![0; limit];
    for i in 1..limit {
        let mut j=i;
        let mut houses = 0.0;
        while j < limit && houses < hpe {
            a[j] += pph * i;
            j += i;
            houses += 1.0;
        }
    }
    a.iter().position(|x| *x >= puzzle)
}

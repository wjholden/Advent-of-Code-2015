use std::collections::HashMap;

fn main() {
    let puzzle = std::fs::read_to_string("puzzles/day07.txt").unwrap();
    let mut h = parse(&puzzle);
    let a = get("a", &h);
    println!("Part 1: {a}");
    let b = a.to_string();
    h.insert("b", &b);
    let a = get("a", &h);
    println!("Part 2: {a}");
}

fn parse(input: &str) -> HashMap<&str, &str> {
    // key = label of wire leaving gate
    // value = inputs and operations at that gate
    let mut circuit = HashMap::new();
    for line in input.lines() {
        let lr: Vec<&str> = line.split(" -> ").collect();
        let input = lr[0];
        let label = lr[1];
        circuit.insert(label, input);
    }
    circuit
}

pub fn get(wire: &str, circuit: &HashMap<&str, &str>) -> u16 {
    // So we're using a mutable HashMap here as a memo table.
    // The get() function is the one users should call.
    // I remember using a similar design in Java and it worked OK.
    // The lifetimes for the string key ("wire") are kinda wonky.
    // 
    // For this problem, the test cases can work fine without dynamic
    // programming but the real puzzle takes too long to finish.
    get_memoized(wire, circuit, &mut HashMap::new())
}

fn get_memoized<'a>(
    wire: &'a str,
    circuit: &'a HashMap<&'a str, &str>,
    cache: &mut HashMap<&'a str, u16>,
) -> u16 {
    if cache.contains_key(wire) {
        return *cache.get(wire).unwrap();
    }

    match wire.parse::<u16>() {
        Ok(n) => {
            cache.insert(wire, n);
            return n;
        } // "wire" is an integer literal
        _ => (),
    }

    let source: Vec<&str> = circuit.get(wire).unwrap().split(" ").collect();
    match source.len() {
        1 => get_memoized(source[0], circuit, cache), // unchanged copy of another wire
        2 => {
            // The only possible instruction here is "NOT".
            assert_eq!(source[0], "NOT");
            let result = !get_memoized(source[1], circuit, cache);
            cache.insert(wire, result);
            result
        }
        3 => {
            // 4 possible infix operators here.
            let operator = match source[1] {
                "AND" => |a, b| a & b,
                "OR" => |a, b| a | b,
                "LSHIFT" => |a, n| a << n,
                "RSHIFT" => |a, n| a >> n,
                _ => panic!("unexpected operator"),
            };
            let left = get_memoized(source[0], circuit, cache);
            let right = get_memoized(source[2], circuit, cache);
            let result = operator(left, right);
            cache.insert(wire, result);
            result
        }
        _ => panic!("unexpected pattern length"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CIRCUIT: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    #[test]
    fn d() {
        let h = parse(CIRCUIT);
        assert_eq!(get("d", &h), 72);
    }

    #[test]
    fn e() {
        let h = parse(CIRCUIT);
        assert_eq!(get("e", &h), 507);
    }

    #[test]
    fn f() {
        let h = parse(CIRCUIT);
        assert_eq!(get("f", &h), 492);
    }

    #[test]
    fn g() {
        let h = parse(CIRCUIT);
        assert_eq!(get("g", &h), 114);
    }

    #[test]
    fn h() {
        let h = parse(CIRCUIT);
        assert_eq!(get("h", &h), 65412);
    }

    #[test]
    fn i() {
        let h = parse(CIRCUIT);
        assert_eq!(get("i", &h), 65079);
    }

    #[test]
    fn x() {
        let h = parse(CIRCUIT);
        assert_eq!(get("x", &h), 123);
    }

    #[test]
    fn y() {
        let h = parse(CIRCUIT);
        assert_eq!(get("y", &h), 456);
    }
}

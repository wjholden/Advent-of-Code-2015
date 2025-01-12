use regex::Regex;
use lazy_static::lazy_static;

#[derive(Copy, Clone)]
enum State {
    On,
    Off,
}

enum Action {
    On,
    Off,
    Toggle,
}

#[derive(Copy, Clone)]
struct Light {
    state: State,
    brightness: u64,
}

impl Light {
    fn toggle(&mut self) {
        match self.state {
            State::On => self.state = State::Off,
            State::Off => self.state = State::On,
        }

        self.brightness += 2
    }

    fn on(&mut self) {
        self.state = State::On;

        self.brightness += 1
    }

    fn off(&mut self) {
        self.state = State::Off;

        if self.brightness > 0 {
            self.brightness -= 1
        }
    }
}

fn main() {
    let mut lights = [Light {
        state: State::Off,
        brightness: 0,
    }; 1_000_000]
        .to_vec();
    let puzzle = std::fs::read_to_string("puzzles/day06.txt").unwrap();
    for line in puzzle.lines() {
        let (action, x1, y1, x2, y2) = parse_line(line);
        set_range(&mut lights, action, x1, y1, x2, y2);
    }
    println!("Part 1: {}", count_on(&lights));
    println!("Part 2: {}", total_brightness(&lights));
}

fn count_on(lights: &[Light]) -> u64 {
    lights
        .iter()
        .map(|l| match l.state {
            State::On => 1,
            State::Off => 0,
        })
        .sum()
}

fn total_brightness(lights: &[Light]) -> u64 {
    lights.iter().map(|l| l.brightness).sum()
}

fn set_range(
    lights: &mut [Light],
    action: Action,
    xmin: usize,
    ymin: usize,
    xmax: usize,
    ymax: usize,
) {
    for x in xmin..=xmax {
        for y in ymin..=ymax {
            let light = lights.get_mut(x * 1000 + y).unwrap();
            match action {
                Action::On => light.on(),
                Action::Off => light.off(),
                Action::Toggle => light.toggle(),
            }
        }
    }
}

fn parse_line(s: &str) -> (Action, usize, usize, usize, usize) {
    let action = if s.contains("turn on") {
        Action::On
    } else if s.contains("turn off") {
        Action::Off
    } else if s.contains("toggle") {
        Action::Toggle
    } else {
        panic!("Invalid action in command")
    };

    lazy_static! {
        static ref re: Regex = Regex::new(r"(\d+),(\d+) through (\d+),(\d+)").unwrap();
    }
    let cap = re.captures(s).unwrap();
    let x1 = cap.get(1).unwrap().as_str();
    let y1 = cap.get(2).unwrap().as_str();
    let x2 = cap.get(3).unwrap().as_str();
    let y2 = cap.get(4).unwrap().as_str();
    let x1 = str::parse(x1).unwrap();
    let y1 = str::parse(y1).unwrap();
    let x2 = str::parse(x2).unwrap();
    let y2 = str::parse(y2).unwrap();
    (action, x1, y1, x2, y2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_on_manual() {
        let mut lights = [Light {
            state: State::Off,
            brightness: 0,
        }; 1_000_000]
            .to_vec();
        for light in lights.iter_mut() {
            light.toggle();
        }
        assert_eq!(count_on(&lights), 1_000_000)
    }

    #[test]
    fn toggle_all() {
        let mut lights = [Light {
            state: State::Off,
            brightness: 0,
        }; 1_000_000]
            .to_vec();
        set_range(&mut lights, Action::Toggle, 0, 0, 999, 999);
        assert_eq!(count_on(&lights), 1_000_000)
    }

    #[test]
    fn toggle_row() {
        let mut lights = [Light {
            state: State::Off,
            brightness: 0,
        }; 1_000_000]
            .to_vec();
        set_range(&mut lights, Action::Toggle, 0, 0, 999, 0);
        assert_eq!(count_on(&lights), 1000)
    }

    #[test]
    fn toggle_4() {
        let mut lights = [Light {
            state: State::Off,
            brightness: 0,
        }; 1_000_000]
            .to_vec();
        set_range(&mut lights, Action::Toggle, 499, 499, 500, 500);
        assert_eq!(count_on(&lights), 4)
    }

    #[test]
    fn brightness_1() {
        let mut lights = [Light {
            state: State::Off,
            brightness: 0,
        }; 1_000_000]
            .to_vec();
        set_range(&mut lights, Action::On, 0, 0, 0, 0);
        assert_eq!(total_brightness(&lights), 1)
    }

    #[test]
    fn brightness_million() {
        let mut lights = [Light {
            state: State::Off,
            brightness: 0,
        }; 1_000_000]
            .to_vec();
        set_range(&mut lights, Action::Toggle, 0, 0, 999, 999);
        assert_eq!(total_brightness(&lights), 2_000_000)
    }
}

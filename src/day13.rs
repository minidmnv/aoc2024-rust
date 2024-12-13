use measure_time_macro::measure_time;
use regex::Regex;

struct Machine {
    a_button: (f64, f64),
    b_button: (f64, f64),
    prize: (f64, f64),
}

impl Machine {

    pub fn solve(&self) -> Option<f64> {
        let a = ((self.prize.0 - self.prize.1 * self.b_button.0 / self.b_button.1) / (self.a_button.0 - self.a_button.1 * self.b_button.0 / self.b_button.1)).round();
        let b = ((self.prize.1 - a * self.a_button.1) / self.b_button.1).round();

        if a * self.a_button.0 + b * self.b_button.0 == self.prize.0 && a * self.a_button.1 + b * self.b_button.1 == self.prize.1 && a >= 0f64 && b >= 0f64 { Some( a * 3f64 + b) } else { None };
    }
}

pub fn run(input: &str) {
    part_one(input);
    part_two(input);
}


#[measure_time]
fn part_one(input: &str) -> f64 {
    let machines = parse(input, 0f64);

    let result = machines.iter().filter_map(|machine| machine.solve()).sum();

    println!("Day 13, part 1 result: {:?}", result);

    result
}

#[measure_time]
fn part_two(input: &str) -> f64 {
    let machines = parse(input, 10000000000000f64);

    let result = machines.iter().filter_map(|machine| machine.solve()).sum();

    println!("Day 13, part 2 result: {:?}", result);

    result
}

fn parse(input: &str, offset: f64) -> Vec<Machine> {
    let re = Regex::new(r"Button A: X\+(?P<ax>\d+), Y\+(?P<ay>\d+)
Button B: X\+(?P<bx>\d+), Y\+(?P<by>\d+)
Prize: X=(?P<px>\d+), Y=(?P<py>\d+)").unwrap();
    let mut machines = vec![];

    for captures in re.captures_iter(input) {
        let ax = captures.name("ax").unwrap().as_str().parse::<f64>().unwrap();
        let ay = captures.name("ay").unwrap().as_str().parse::<f64>().unwrap();
        let a = (ax, ay);

        let bx = captures.name("bx").unwrap().as_str().parse::<f64>().unwrap();
        let by = captures.name("by").unwrap().as_str().parse::<f64>().unwrap();
        let b = (bx, by);

        let px = captures.name("px").unwrap().as_str().parse::<f64>().unwrap();
        let py = captures.name("py").unwrap().as_str().parse::<f64>().unwrap();
        let prize = (px + offset, py + offset);

        machines.push(Machine { a_button: a, b_button: b, prize });
    }

    machines
}

#[cfg(test)]
mod day13 {
    use crate::day13::*;
    use std::assert_eq;

    const SAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    #[test]
    fn test1() {
        assert_eq!(part_one(SAMPLE), 480.00)
    }
}

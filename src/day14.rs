use std::collections::{HashMap, HashSet};
use measure_time_macro::measure_time;
use regex::Regex;

#[derive(Debug, Clone)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32)
}


impl Robot {

    pub fn predict_future(&self, time: i32, width: i32, height: i32) -> Robot {
        let mut result_robot = self.clone();
        for _ in 0..time {
            result_robot = result_robot.move_me(width, height);
        }

        return result_robot;
    }

    fn move_me(&self, width: i32, height: i32) -> Robot {
        let mut new_x = self.position.0 + self.velocity.0;
        let mut new_y = self.position.1 + self.velocity.1;

        new_x = if new_x < 0 { 1 + width + new_x } else if new_x > width { new_x - width - 1 } else { new_x };
        new_y = if new_y < 0 { 1 + height + new_y } else if new_y > height { new_y - height - 1 } else { new_y };

        Robot { position: (new_x, new_y), velocity: self.velocity }
    }

}

pub fn run(input: &str) {
    part_one(input, 100, 102);
    part_two(input, 101, 103);
}


#[measure_time]
fn part_one(input: &str, width: i32, height: i32) -> i32 {
    let quadrants: [(std::ops::Range<i32>, std::ops::Range<i32>); 4] = [
        (0..width / 2, 0..height / 2),       // Quadrant 1
        (width / 2 + 1..width + 1, 0..height / 2),     // Quadrant 2
        (0..width / 2, height / 2 + 1..height + 1),     // Quadrant 3
        (width / 2 + 1..width + 1, height / 2 + 1..height + 1),   // Quadrant 4
    ];

    let robots = parse(input);
    let mut quadrant_counts: HashMap<i32, i32> = HashMap::new();

    robots.iter().map(|robot| robot.predict_future(100, width, height)).for_each(|future_robot| {
        let position = future_robot.position;
        if let Some(quadrant) = quadrants.iter().position(|(x_range, y_range)| {
            x_range.contains(&position.0) && y_range.contains(&position.1)
        }) {
            *quadrant_counts.entry((quadrant + 1) as i32).or_insert(0) += 1;
        }
    });

    for i in 1..=4 {
        quadrant_counts.entry(i).or_insert(0);
    }

    // Calculate the product of counts
    let safety_factor: i32 = quadrant_counts
        .values()
        .copied()
        .product();

    println!("Day 14, part 1 result: {:?}", safety_factor);

    safety_factor
}

#[measure_time]
fn part_two(input: &str, width: i32, height: i32) -> i32 {
    let robots = parse(input);

    let boundary: (i32, i32) = (width, height);
    let mut result = 0;
    'outer: loop {
        let mut pos = HashSet::new();

        for robot in robots.clone() {
            let x = robot.position.0 + robot.velocity.0 * result;
            let y = robot.position.1 + robot.velocity.1 * result;
            let p = (
                ((x % boundary.0) + boundary.0) % boundary.0,
                ((y % boundary.1) + boundary.1) % boundary.1,
            );
            if !pos.contains(&p) {
                pos.insert(p);
            } else {
                break;
            }
        }

        println!("POS: {} in res: {}", pos.len(), result);
        // 4 hours to get to this point :...(
        // I tried all sorts of shit, including rendering every frame as an image to create an animation
        if pos.len() == robots.len() {
            for y in 0..boundary.1 {
                for x in 0..boundary.0 {
                    if pos.contains(&(x, y)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!("Iteration {}", result);
            break 'outer;
        }

        result += 1;
    }

    //
    //
    // let quadrants: [(std::ops::Range<i32>, std::ops::Range<i32>); 4] = [
    //     (0..width / 2, 0..height / 2),       // Quadrant 1
    //     (width / 2 + 1..width + 1, 0..height / 2),     // Quadrant 2
    //     (0..width / 2, height / 2 + 1..height + 1),     // Quadrant 3
    //     (width / 2 + 1..width + 1, height / 2 + 1..height + 1),   // Quadrant 4
    // ];
    //
    // let robots = parse(input);
    // let mut result = 0;
    // let mut current_factor = i32::MAX;
    //
    // for i in 0..i32::MAX {
    //
    //     let mut quadrant_counts: HashMap<i32, i32> = HashMap::new();
    //
    //     robots.iter().map(|robot| robot.predict_future(i, width, height)).for_each(|future_robot| {
    //         let position = future_robot.position;
    //         if let Some(quadrant) = quadrants.iter().position(|(x_range, y_range)| {
    //             x_range.contains(&position.0) && y_range.contains(&position.1)
    //         }) {
    //             *quadrant_counts.entry((quadrant + 1) as i32).or_insert(0) += 1;
    //         }
    //     });
    //
    //     for i in 1..=4 {
    //         quadrant_counts.entry(i).or_insert(0);
    //     }
    //
    //     let factor = quadrant_counts.values().sum();
    //     println!("Trying {} seconds with {}", i, factor);
    //
    //     let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
    //     for robot in &robots {
    //         let position = robot.predict_future(i, width, height).position;
    //         *grid.entry(position).or_insert(0) += 1;
    //     }
    //
    //     for y in 0..=height {
    //         for x in 0..=width {
    //             if let Some(count) = grid.get(&(x, y)) {
    //                 print!("{}", count);
    //             } else {
    //                 print!(".");
    //             }
    //         }
    //         println!();
    //     }
    //
    //     if factor < current_factor {
    //         result = i;
    //         current_factor = factor;
    //     }
    // }

    println!("Day 14, part 2 result: {:?}", result);

    result
}

fn parse(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(?P<x>-?\d+),(?P<y>-?\d+) v=(?P<dx>-?\d+),(?P<dy>-?\d+)").unwrap();
    let mut robots = vec![];

    for captures in re.captures_iter(input) {
        let x = captures.name("x").unwrap().as_str().parse::<i32>().unwrap();
        let y = captures.name("y").unwrap().as_str().parse::<i32>().unwrap();
        let position = (x, y);

        let dx = captures.name("dx").unwrap().as_str().parse::<i32>().unwrap();
        let dy = captures.name("dy").unwrap().as_str().parse::<i32>().unwrap();
        let velocity = (dx, dy);

        robots.push(Robot { position, velocity });
    }

    robots
}

#[cfg(test)]
mod day14 {
    use crate::day14::*;
    use std::assert_eq;

    const SAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    #[test]
    fn test1() {
        assert_eq!(part_one(SAMPLE, 10, 6), 12)
    }
}

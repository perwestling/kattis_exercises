use std::{io::{self, prelude::*}};

#[derive(Debug)]
struct Point {
    x : f32,
    y : f32,
}

pub fn main() {
    let mut stdin_iter = io::stdin().lock().lines();
    let lines = stdin_iter.next().expect("Missing line 1").unwrap().parse::<u32>().unwrap();
    let mut points : Vec<Point> = Vec::new();
    let mut used : Vec<bool> = Vec::new();
    const DEBUG : bool = false;
    for _ in 0..lines {
        let line = stdin_iter.next().expect("Missing float line").unwrap();
        let float_pair = get_f32_numbers(&line);
        let p = Point { x: float_pair[0], y: float_pair[1]};
        points.push(p);
        used.push(false);
    }
    if DEBUG {
        println!("{:#?}", points);
        for p in points.iter() {
            println!("Distance between {:#?} and {:#?} is {}", points[0], p, dist(&points[0], &p));
        }
    }

    let mut tour : Vec<usize> = Vec::new();
    _ = greedy_tour(lines as usize, &mut tour, &mut used, &points, DEBUG);
    if DEBUG {
        println!("TOUR:");
    }
    for t in tour.iter() {
        println!("{}", t);
    }
    ()
}

fn greedy_tour(max : usize, tour: &mut Vec<usize>, used: &mut Vec<bool>, points: &Vec<Point>, debug: bool) {
    if debug {
        println!("Start greedy tour in index 0");
    }
    tour.push(0);
    used[0] = true;

    for i in 1..max {
        let mut best : Option<usize> = None;
        let i_point : &Point = &points[tour[i-1]];
        if debug {
            println!("i: {}", &i);
        }
        for j in 0..max {
            if debug {
                println!("j: {}", &j);
            }
            if !used[j] && (best == None ||
                            dist(i_point, &points[j]) < dist(i_point, &points[best.unwrap()])) {
                if debug {
                    println!("Is best candidate");
                }
                best = Some(j);
            }
        }
        let best_index = best.unwrap();
        if debug {
            println!("Best was: {}", &best_index);
            println!("");
        }
        tour.push(best_index);
        used[best_index] = true;
    }
}

fn get_f32_numbers(s: &str) -> Vec<f32> {
    return s
     .to_string()
     .split_whitespace()
     .map(|item| item.to_string().parse::<f32>())
     .collect::<Result<Vec<_>, _>>()
     .unwrap_or_else(|err| panic!("Failure during parse: {}", err));
}

fn dist(p: &Point, q: &Point) -> i32 {
    return (sqr_diff(p.x, q.x) + (sqr_diff(p.y, q.y))).sqrt().floor() as i32;
}

fn sqr_diff(x : f32, y: f32) -> f32 {
    return (x - y) * (x - y);
}

#[cfg(test)]
mod tests;

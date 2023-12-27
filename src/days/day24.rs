use std::fs::read_to_string;
use std::ops::RangeInclusive;

use rayon::prelude::*;
use itertools::Itertools;

use crate::etc::Coords2D;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type Pos2D = Coords2D<f64>;
const COLLISION_RANGE: RangeInclusive<f64> = 200_000_000_000_000.0..=400_000_000_000_000.0;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Pos3D {
    x: f64, y: f64, z: f64
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct HailData {
    start: Pos3D,
    velocity: Pos3D,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct HailData2D {
    start: Pos2D,
    velocity: Pos2D,
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day24.txt").unwrap();
    let hails = input.lines().map(parse_hail).collect_vec();

    let sol1 = hails.iter()
        .map(|h| project_hail(h, 2))
        .tuple_combinations()
        .filter(|pair| find_intersection(pair, false).is_some())
        .count();

    let rock = find_rock_coords(&hails);
    let sol2 = rock.x as i64 + rock.y as i64 + rock.z as i64;

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn find_rock_coords(hails: &[HailData]) -> Pos3D {
    let proj0 = hails.iter().map(|h| project_hail(h, 0)).collect_vec();
    let proj1 = hails.iter().map(|h| project_hail(h, 1)).collect_vec();
    let res0 = find_common_intersection(&proj0);
    let res1 = find_common_intersection(&proj1);

    Pos3D { x: res1.x.round(), y: res0.x.round(), z: res0.y.round() }
}

// Finds the speed of the rock in a 2D projection by brute-forcing
// all possible values between -400 and 400. This works thanks to
// the fact that the rock moving at a speed of v is equivalent to
// it staying stationary and all the hails moving at their respective
// speed minus v. In that case, if the speed of the rock is correct,
// the paths of all pairs of hailstones will pass through the position
// of the stationary rock, and thus all pairs of paths must cross
// each other exactly at the starting position of the rock.
fn find_common_intersection(projected: &[HailData2D]) -> Pos2D {
    (-500..=500).into_par_iter().find_map_any(|speed_x| {
        'outer: for speed_y in -500..=500 {
            let stone_speed = Pos2D::new(speed_x as f64, speed_y as f64);
            let relatives = projected.iter()
                .map(|h| HailData2D { start: h.start, velocity: h.velocity - stone_speed } );

            // Remove parallel paths and vertical ones because
            // they don't work well with our continuous interception finder
            let mut pairs = relatives
                .filter(|h| h.velocity.x != 0.0)
                .tuple_combinations()
                .filter(|(h1, h2)| slope(h1) != slope(h2));

            // If this is the correct stone speed, all intersections
            // will be valid and at the same position
            let first_intersect = find_intersection(&pairs.next().unwrap(), true);
            if first_intersect.is_none() {
                continue
            }

            let mut matched = 0;
            for pair in pairs {
                let other_intersect = find_intersection(&pair, true);
                if !check_intersects(&other_intersect, &first_intersect) {
                    continue 'outer;
                } else {
                    matched += 1;
                    if matched >= 5 {
                        return Some(first_intersect.unwrap());
                    }
                }
            }
        }

        None
    }).unwrap()
}

fn check_intersects(e1: &Option<Pos2D>, e2: &Option<Pos2D>) -> bool {
    if e1.is_none() || e2.is_none() {
        return false;
    }

    let p1 = e1.unwrap();
    let p2 = e2.unwrap();
    (p1.x - p2.x).abs() < 1.0 && (p1.y - p2.y).abs() < 1.0
}

// Determines whether two hail trajectories in normal form will cross in the
// designated area AND in the future for both hailstones
fn find_intersection((h1, h2): &(HailData2D, HailData2D), part2: bool) -> Option<Pos2D> {
    // Calculate the intersection point using the normal forms
    let (a1, b1, c1) = normal_form(h1);
    let (a2, b2, c2) = normal_form(h2);

    if a1 == a2 {
        return None;  // Paths are parallel, will never cross
    }

    let x = (b1*c2 - b2*c1) / (a1*b2 - a2*b1);
    let y = (c1*a2 - c2*a1) / (a1*b2 - a2*b1);

    let in_bounds = part2 || COLLISION_RANGE.contains(&x) && COLLISION_RANGE.contains(&y);

    if time_at(h1, x) > 0.0 && time_at(h2, x) > 0.0 && in_bounds {
        Some(Pos2D::new(x, y))
    } else { None }
}

// Calculates the time at which a given hail will be at a given x coordinate
fn time_at(hail: &HailData2D, x: f64) -> f64 {
    (x - hail.start.x) / hail.velocity.x
}

// Computes the normal form of the line defined by the x and y
// coordinates of the hail's initial position of speed,
// in the form Ax + By + C = 0, returning (A, B, C)
fn normal_form(hail: &HailData2D) -> (f64, f64, f64) {
    let slope = slope(hail);
    let a = slope;
    let b = -1.0;
    let c = slope * (-hail.start.x) + hail.start.y;
    (a, b, c)
}

fn slope(hail: &HailData2D) -> f64 {
    hail.velocity.y / hail.velocity.x
}

fn project_hail(hail: &HailData, axis: u8) -> HailData2D {
    HailData2D {
        start: project(hail.start, axis),
        velocity: project(hail.velocity, axis),
    }
}

fn project(coords: Pos3D, axis: u8) -> Pos2D {
    match axis {
        0 => Pos2D { x: coords.y, y: coords.z },
        1 => Pos2D { x: coords.x, y: coords.z },
        _ => Pos2D { x: coords.x, y: coords.y }
    }
}

fn parse_hail(line: &str) -> HailData {
    let (start, velocity) = line.split('@')
        .map(parse_coords)
        .collect_tuple().unwrap();
    HailData { start, velocity }
}

fn parse_coords(s: &str) -> Pos3D {
    let (x, y, z) = s.split(',')
        .map(|i| i.trim().parse().unwrap())
        .collect_tuple().unwrap();
    Pos3D { x, y, z }
}

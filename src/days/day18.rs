use std::fs::read_to_string;
use itertools::Itertools;
use crate::etc::Coords2D;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type Pos = Coords2D<i64>;
type Instr<'a> = (char, i64, &'a str);

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day18.txt").unwrap();
    let instrs = input.lines().map(parse_line).collect_vec();
    let instrs2 = instrs.iter().map(true_instruction).collect_vec();

    let sol1 = calculate_points(&instrs);
    let sol2 = calculate_points(&instrs2);

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn calculate_points(instrs: &[Instr]) -> i64 {
    let vertices = find_vertices(instrs);
    let total_area = internal_area(&vertices);
    let outside_points: i64 = instrs.iter().map(|i| i.1).sum();
    // We use Pick's theorem to calculate the number of internal
    // points of the polygon based on the area and external points
    // https://en.m.wikipedia.org/wiki/Pick's_theorem
    let inside_points = total_area - (outside_points / 2) + 1;
    outside_points + inside_points
}

fn find_vertices(instrs: &[Instr]) -> Vec<Pos> {
    // We use a vec because preserving the order is important for area calculation
    let mut current = Pos::origin();
    let mut res = vec![current];

    for &(dir, amt, _) in instrs {
        let delta = match dir {
            'U' => Pos::up(),
            'D' => Pos::down(),
            'L' => Pos::left(),
             _  => Pos::right(),
        };

        current += delta * amt;
        res.push(current);
    }

    res
}
 
// https://en.wikipedia.org/wiki/Shoelace_formula
fn internal_area(vertices: &[Pos]) -> i64 {
    vertices.iter().tuple_windows()
        .map(|(a, b)| a.x * b.y - a.y * b.x)
        .sum::<i64>().abs() / 2
}

fn true_instruction<'a>(instr: &'a Instr<'a>) -> Instr<'a> {
    let number = i64::from_str_radix(&instr.2[..5], 16).unwrap();
    let dir = match &instr.2[5..] {
        "0" => 'R',
        "1" => 'D',
        "2" => 'L',
         _  => 'U'
    };
    (dir, number, "")
}

fn parse_line(line: &str) -> Instr {
    let mut spl = line.split_whitespace();
    let dir = spl.next().unwrap().chars().next().unwrap();
    let number = spl.next().unwrap().parse().unwrap();
    let hex = &spl.next().unwrap()[2..8];
    (dir, number, hex)
}

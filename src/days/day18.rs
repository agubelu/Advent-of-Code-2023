use std::fs::read_to_string;
use itertools::Itertools;
use sscanf::sscanf;
use crate::etc::Coords2D;
use crate::days::day10;
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
    let outside = day10::outside_points(&vertices);
    let inside = day10::inside_points(&vertices, outside);
    outside + inside
}

fn find_vertices(instrs: &[Instr]) -> Vec<Pos> {
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
    sscanf!(line, "{char} {i64} (#{str})").unwrap()
}

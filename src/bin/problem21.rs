extern crate aoc;

use aoc::int_code::IntProgram;

fn convert(code: &str) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    for c in code.chars() {
        result.push(c as u8 as i64);
    }
    result
}

fn main() {
    let code = aoc::parse_file::<i64>("input.txt", ",");
    let walk_code = "OR A J\nAND B J\nAND C J\nNOT J J\nAND D J\nWALK\n";
    println!("walk {}", IntProgram::new(code.clone()).execute(convert(walk_code)).last().unwrap());
    let run_code = "OR A T\nAND B T\nAND C T\nNOT T T\nAND D T\nOR F J\nOR I J\nAND E J\nOR H J\nAND T J\nRUN\n";
    println!("run {}", IntProgram::new(code.clone()).execute(convert(run_code)).last().unwrap());
}

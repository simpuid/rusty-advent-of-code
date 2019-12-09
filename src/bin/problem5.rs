extern crate aoc;
use aoc::int_code::IntProgram;

fn main() {
    let code: Vec<i64> = aoc::parse_file::<i64>("input.txt", ",");
    let mut program: IntProgram = IntProgram::new(code.clone());
    let result: i64 = *program.execute(vec![1]).last().expect("program failed");
    println!("result one {}", result);
    let mut program: IntProgram = IntProgram::new(code);
    let result: i64 = *program.execute(vec![5]).last().expect("program failed");
    println!("result two {}", result);
}

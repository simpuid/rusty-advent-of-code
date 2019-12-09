extern crate aoc;
use aoc::int_code::IntProgram;

fn main() {
    let code = aoc::parse_file::<i64>("input.txt", ",");
    let mut program = IntProgram::new(code.clone());
    println!("{:?}", program.execute(vec![1]));
    let mut program = IntProgram::new(code.clone());
    println!("{:?}", program.execute(vec![2]));
}

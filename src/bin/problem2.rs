extern crate aoc;
use aoc::int_code::IntProgram;

fn execute_program(code: &Vec<i64>, arg1: i64, arg2: i64) -> Option<i64> {
    let mut program: IntProgram = IntProgram::new(code.clone());
    program.set(1, arg1);
    program.set(2, arg2);
    program.execute(Vec::new());
    program.get(0)
}

fn main() {
    let code = aoc::parse_file::<i64>("input.txt", ",");
    if let Some(e) = execute_program(&code, 12, 2) {
        println!("first answer is {}", e);
    }
    let result: i64 = 19690720;
    for i in 0..99 {
        for j in 0..99 {
            if let Some(e) = execute_program(&code, i, j) {
                if e == result {
                    println!("second answer is {}", i * 100 + j);
                }
            }
        }
    }
}

extern crate aoc;
use aoc::int_code::IntProgram;

fn block_count(v: &[i64]) -> isize {
    let mut iter = v.iter();
    let mut result: isize = 0;
    while let (Some(_), Some(_), Some(id)) = (iter.next(), iter.next(), iter.next()) {
        if *id == 2 {
            result += 1;
        }
    }
    result
}

fn sign(a: i64) -> i64 {
    if a > 0 {
        1
    } else if a < 0 {
        -1
    } else {
        0
    }
}

fn main() {
    let code = aoc::parse_file::<i64>("input.txt", ",");
    let mut comp = IntProgram::new(code.clone());
    let output = comp.execute(Vec::new());
    println!("block count: {}", block_count(&output));
    let mut program = IntProgram::new(code);
    program.set(0, 2);
    let mut ball_pos: i64 = 0;
    let mut paddle_pos: i64 = 0;
    let mut score: i64 = -1;
    print!("playing");
    let mut dot_counter = 0;
    while program.can_run() {
        let output = program.execute(vec![sign(ball_pos - paddle_pos)]);
        let mut iter = output.into_iter();
        while let (Some(x), Some(y), Some(id)) = (iter.next(), iter.next(), iter.next()) {
            match (x, y, id) {
                (-1, 0, new_score) => score = new_score,
                (x, _, 3) => paddle_pos = x,
                (x, _, 4) => ball_pos = x,
                _ => (),
            }
        }
        dot_counter += 1;
        while dot_counter > 100 {
            dot_counter -= 100;
            print!(".");
        }
    }
    println!("finished");
    println!("final score: {}", score);
}

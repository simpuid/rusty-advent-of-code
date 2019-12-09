extern crate aoc;
use aoc::int_code::IntProgram;

fn main() {
    let code: Vec<i64> = aoc::parse_file::<i64>("input.txt", ",");
    find_max([0, 1, 2, 3, 4], &code);
    find_max([5, 6, 7, 8, 9], &code);
}

fn call_feedback(phases: &[i64; 5], code: &[i64], thrust: &mut Vec<i64>) {
    let mut input: i64 = 0;
    let mut amp: [IntProgram; 5] = [
        IntProgram::new(code.to_vec()),
        IntProgram::new(code.to_vec()),
        IntProgram::new(code.to_vec()),
        IntProgram::new(code.to_vec()),
        IntProgram::new(code.to_vec()),
    ];
    for (i, prog) in amp.iter_mut().enumerate() {
        prog.execute(vec![*phases.get(i).expect("phase error")]);
    }
    let mut index: usize = 0;
    while amp[index % 5].can_run() {
        if let Some(prog) = amp.get_mut(index % 5) {
            if let Some(e) = prog.execute(vec![input]).last() {
                input = *e;
            }
        }
        index += 1;
    }
    thrust.push(input);
}

fn permute(phases: &mut [i64; 5], size: usize, code: &[i64], thrust: &mut Vec<i64>) {
    if size == 1 {
        call_feedback(&phases, code, thrust);
    } else {
        for i in 0..size {
            permute(phases, size - 1, code, thrust);
            if (size % 2) != 0 {
                phases.swap(0, size - 1);
            } else {
                phases.swap(i, size - 1);
            }
        }
    }
}

fn find_max(mut phases: [i64; 5], code: &[i64]) {
    let mut thrust: Vec<i64> = Vec::new();
    permute(&mut phases, 5, &code, &mut thrust);
    let mut max: i64 = *thrust.first().expect("empty permutation");
    for i in thrust {
        if max < i {
            max = i;
        }
    }
    println!("max {}", max);
}

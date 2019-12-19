extern crate aoc;

use aoc::int_code::IntProgram;

fn count50x50(code: &[i64]) {
    let mut count = 0;
    for y in 0..50 {
        for x in 0..50 {
            if get(code, x, y) {
                count += 1;
            }
        }
    }
    println!("total {}", count);
}

fn fit100x100(code: &[i64], size: usize) {
    let mut y = size - 1;
    let mut x = 0;
    loop {
        while !get(code, x, y) {
            x += 1;
        }
        if y >= size - 1 && get(code, x, y + 1 - size) && get(code, x + size - 1, y + 1 - size) {
            println!("{}*10000+{} = {}", x, y + 1 - size, x * 10000 + y + 1 - size);
            break;
        }
        y += 1;
    }
}

fn get(code: &[i64], x: usize, y: usize) -> bool {
    *IntProgram::new(Vec::from(code)).execute(vec![x as i64, y as i64]).first().unwrap() == 1
}

fn main() {
    let code = aoc::parse_file::<i64>("input.txt", ",");
    count50x50(&code);
    fit100x100(&code, 100);
}

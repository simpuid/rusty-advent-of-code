extern crate aoc;

use aoc::int_code::IntProgram;

fn i64_to_string(data: &[i64]) -> String {
    let mut res = String::new();
    for i in data.iter() {
        res.push(*i as u8 as char)
    }
    res
}

fn string_to_i64(data: String) -> Vec<i64> {
    let mut res: Vec<i64> = Vec::new();
    for c in data.chars() {
        res.push(c as u8 as i64);
    }
    res
}

fn io_routine(program: &mut IntProgram) {
    let output = i64_to_string(program.execute(Vec::new()).as_slice());
    print!("{}", output);
    while program.can_run() {
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_ok() {
            let output = i64_to_string(program.execute(string_to_i64(input)).as_slice());
            print!("{}", output);
        } else {
            break;
        }
    }
}

fn collect_items_to_checkpoint(program: &mut IntProgram) {
    program.execute(string_to_i64(
        "west
take hologram
north
take space heater
east
take space law space brochure
east
take tambourine
west
west
south
east
east
take festive hat
east
take food ration
east
take spool of cat6
west
west
south
east
east
take fuel cell
east
"
        .to_string(),
    ));
}

fn permute_sensor(program: &mut IntProgram) {
    let items = vec![
        "fuel cell",
        "space heater",
        "hologram",
        "space law space brochure",
        "food ration",
        "tambourine",
        "spool of cat6",
        "festive hat",
    ];
    'mask_loop: for mask in 0..=0b1111_1111 {
        let mut clone = program.clone();
        for (i, string) in items.iter().enumerate() {
            if (mask & (1 << i)) != 0 {
                clone.execute(string_to_i64(format!("drop {}\n", *string)));
            }
        }
        let out = i64_to_string(clone.execute(string_to_i64("south\n".to_string())).as_slice());
        for s in aoc::parse_string::<String>(out.as_str(), "\n") {
            if s.as_str() == "== Security Checkpoint ==" {
                continue 'mask_loop;
            }
        }
        println!("out mask {} \n{}", mask, &out);
    }
}

fn main() {
    let mut program = IntProgram::new(aoc::parse_file::<i64>("input.txt", ","));
    collect_items_to_checkpoint(&mut program);
    permute_sensor(&mut program);
}

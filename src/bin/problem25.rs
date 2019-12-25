extern crate aoc;

use aoc::int_code::IntProgram;
use std::collections::{HashMap, HashSet};

fn execute(program: &mut IntProgram, string: &str) -> String {
    let mut input: Vec<i64> = Vec::new();
    for c in string.chars() {
        input.push(c as i64);
    }
    let mut output = String::new();
    for i in program.execute(input) {
        output.push(i as u8 as char);
    }
    output
}

fn io_routine(mut program: &mut IntProgram) {
    let mut input = String::new();
    while program.can_run() {
        let output = execute(&mut program, input.as_str());
        println!("{}", &output);
        parse_output(output);

        if std::io::stdin().read_line(&mut input).is_err() {
            println!("error reading input");
            break;
        }
    }
}

fn opposite(string: &str) -> &str {
    match string {
        "north" => "south",
        "south" => "north",
        "east" => "west",
        _ => "east",
    }
}

fn collect_items(mut program: &mut IntProgram, direction: &str, mut rooms: &mut HashSet<String>) {
    let (name, directions, items) = parse_output(execute(&mut program, format!("{}\n", direction).as_str()));
    let blacklist = ["giant electromagnet", "infinite loop", "escape pod", "photons", "molten lava"];
    for i in items {
        if !blacklist.contains(&i.as_str()) {
            execute(&mut program, format!("take {}\n", i).as_str());
        }
    }
    if !rooms.contains(&name) {
        rooms.insert(name.clone());
        if name != "== Security Checkpoint ==" {
            for d in directions {
                collect_items(&mut program, d.as_str(), &mut rooms);
            }
        }
    }
    execute(&mut program, format!("{}\n", opposite(direction)).as_str());
}

fn find_sensor(mut program: &mut IntProgram, direction: &str, mut rooms: &mut HashSet<String>) -> Option<String> {
    let (name, directions, _) = parse_output(execute(&mut program, format!("{}\n", direction).as_str()));
    if !rooms.contains(&name) {
        rooms.insert(name.clone());
        for d in directions {
            if d != opposite(direction) {
                if let Some(e) = find_sensor(&mut program, d.as_str(), &mut rooms) {
                    return Some(e);
                }
            }
        }
    } else if name == "== Security Checkpoint ==" {
        return Some(direction.to_string());
    }
    execute(&mut program, format!("{}\n", opposite(direction)).as_str());
    None
}

fn main() {
    let mut program = IntProgram::new(aoc::parse_file::<i64>("input.txt", ","));
    let mut rooms_found: HashSet<String> = HashSet::new();
    let (name, directions, _) = parse_output(execute(&mut program, ""));
    rooms_found.insert(name);
    for d in &directions {
        collect_items(&mut program, d.as_str(), &mut rooms_found);
    }
    println!("collected");
    rooms_found.clear();
    let mut permute_direction: Option<String> = None;
    for d in &directions {
        if let Some(e) = find_sensor(&mut program, d.as_str(), &mut rooms_found) {
            permute_direction = Some(e);
            continue;
        }
    }
    if let Some(e) = permute_direction {
        permute(&mut program, e);
    }
    io_routine(&mut program);
}

fn permute(mut program: &mut IntProgram, direction: String) {
    println!("permuting");
    let mut map: HashMap<u64, String> = HashMap::new();
    let items = parse_inventory(execute(&mut program, "inv\n"));
    let item_count = items.len() as u64;
    let mut bits: HashSet<u64> = HashSet::new();
    for (i, name) in items.iter().enumerate() {
        map.insert(1u64 << (i as u64), name.clone());
        bits.insert(1 << i as u64);
    }
    for mask in 0..(1u64 << item_count) {
        let mut i: u64 = 1;
        for _ in 0..64 {
            if mask & i != 0 {
                if !bits.contains(&i) {
                    if let Some(e) = map.get(&i) {
                        execute(&mut program, format!("take {}\n", e).as_str());
                        bits.insert(i);
                    }
                }
            } else if bits.contains(&i) {
                if let Some(e) = map.get(&i) {
                    execute(&mut program, format!("drop {}\n", e).as_str());
                    bits.remove(&i);
                }
            }
            i <<= 1;
        }
        let output = execute(&mut program, format!("{}\n", direction).as_str());
        let (name, _, _) = parse_output(output.clone());
        if name != "== Security Checkpoint ==" {
            println!("{}", output);
            break;
        }
    }
}

fn parse_output(string: String) -> (String, Vec<String>, Vec<String>) {
    let lines = aoc::parse_string::<String>(string.as_str(), "\n");
    let mut name = String::new();
    let mut direction: Vec<String> = Vec::new();
    let mut item: Vec<String> = Vec::new();
    let mut target = 0;
    for l in lines {
        if l.is_empty() {
            target = 0;
        } else if l.starts_with("==") {
            name = l;
            direction.clear();
            item.clear();
        } else if l == "Doors here lead:" {
            target = 1;
        } else if l == "Items here:" {
            target = 2;
        } else {
            match target {
                1 => direction.push(String::from(l.trim_start_matches("- "))),
                2 => item.push(String::from(l.trim_start_matches("- "))),
                _ => (),
            }
        }
    }
    (name, direction, item)
}

fn parse_inventory(string: String) -> Vec<String> {
    let lines = aoc::parse_string::<String>(string.as_str(), "\n");
    let mut result: Vec<String> = Vec::new();
    for l in lines {
        if l.starts_with("- ") {
            result.push(String::from(l.trim_start_matches("- ")));
        }
    }
    result
}

extern crate aoc;

use aoc::int_code::IntProgram;
use aoc::vector::Vector;
use std::collections::HashSet;

fn read_grid(data: &[i64]) -> (HashSet<Vector>, Vector, Vector) {
    let mut set: HashSet<Vector> = HashSet::new();
    let mut position = Vector::new(0, 0);
    let mut direction = Vector::new(0, 0);
    let mut index = Vector::new(0, 0);
    for val in data {
        match *val {
            10 => {
                index.y += 1;
                index.x = -1;
            }
            35 => {
                set.insert(index);
            }
            118 | 94 | 60 | 62 => {
                set.insert(index);
                position = index;
                direction = match *val {
                    118 => Vector::new(0, 1),
                    94 => Vector::new(0, -1),
                    60 => Vector::new(-1, 0),
                    62 => Vector::new(1, 0),
                    _ => Vector::new(0, 0),
                }
            }
            _ => (),
        }
        index.x += 1;
    }
    (set, position, direction)
}

fn rotate_right(v: Vector) -> Vector {
    Vector::new(-v.y, v.x)
}

fn rotate_left(v: Vector) -> Vector {
    Vector::new(v.y, -v.x)
}

fn calculate_intersection(grid: &HashSet<Vector>) -> i32 {
    let offsets = vec![Vector::new(0, 0), Vector::new(1, 0), Vector::new(-1, 0), Vector::new(0, 1), Vector::new(0, -1)];
    let mut sum = 0;
    for cell in grid.iter() {
        let mut result = true;
        for off in &offsets {
            result = result && grid.contains(&(*cell + *off));
        }
        if result {
            sum += cell.x * cell.y;
        }
    }
    sum
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Instruction {
    Left,
    Right,
    Forward(usize),
}

fn generate_instructions(grid: &HashSet<Vector>, mut position: Vector, mut direction: Vector) -> Vec<Instruction> {
    let mut inst: Vec<Instruction> = Vec::new();
    loop {
        if grid.contains(&(position + direction)) {
            match inst.pop() {
                Some(Instruction::Forward(step)) => inst.push(Instruction::Forward(step + 1)),
                Some(i) => {
                    inst.push(i);
                    inst.push(Instruction::Forward(1));
                }
                None => inst.push(Instruction::Forward(1)),
            }
        } else if grid.contains(&(position + rotate_left(direction))) {
            inst.push(Instruction::Left);
            inst.push(Instruction::Forward(1));
            direction = rotate_left(direction);
        } else if grid.contains(&(position + rotate_right(direction))) {
            inst.push(Instruction::Right);
            inst.push(Instruction::Forward(1));
            direction = rotate_right(direction);
        } else {
            break;
        }
        position = position + direction;
    }
    inst
}

fn generate_sub_instructions(instruction: &[Instruction]) -> Vec<&[Instruction]> {
    let mut sub_instructions: HashSet<&[Instruction]> = HashSet::new();
    for start in 0..instruction.len() {
        for end in (start + 1)..instruction.len() {
            let slice = &instruction[start..end];
            if !slice.is_empty() && count(&slice) <= 20 {
                sub_instructions.insert(slice);
            }
        }
    }
    sub_instructions.into_iter().collect()
}

fn digits(mut x: usize) -> usize {
    let mut sum = 0;
    while x > 0 {
        sum += 1;
        x /= 10;
    }
    sum
}

fn count(instructions: &[Instruction]) -> usize {
    if instructions.is_empty() {
        return 0;
    }
    let mut sum = instructions.len() - 1;
    for inst in instructions {
        match *inst {
            Instruction::Forward(step) => sum += digits(step),
            _ => sum += 1,
        }
    }
    sum
}

fn match_instruction(main: &[Instruction], pattern: &[Instruction]) -> bool {
    if main.len() >= pattern.len() {
        let (mut m_iter, mut p_iter) = (main.iter(), pattern.iter());
        while let (Some(m), Some(p)) = (m_iter.next(), p_iter.next()) {
            if *m != *p {
                return false;
            }
        }
        return true;
    }
    false
}

fn find_combination<'a>(main: &[Instruction], subs: &[&'a [Instruction]]) -> Option<(Vec<usize>, [&'a [Instruction]; 3])> {
    for i in 0..subs.len() {
        for j in (i + 1)..subs.len() {
            for k in (j + 1)..subs.len() {
                let selected_subs = [subs[i], subs[j], subs[k]];
                let mut pattern: Vec<usize> = Vec::new();
                let mut slice = main;
                'slicing_loop: while !slice.is_empty() {
                    for (index, sub) in selected_subs.iter().enumerate() {
                        if match_instruction(slice, sub) {
                            slice = &slice[sub.len()..];
                            pattern.push(index);
                            continue 'slicing_loop;
                        }
                    }
                    break 'slicing_loop;
                }
                if slice.is_empty() {
                    return Some((pattern, selected_subs));
                }
            }
        }
    }
    None
}

fn generate_input(pattern: &[usize], subs: [&[Instruction]; 3]) -> String {
    let mut p_str = String::from("");
    for (i, p) in pattern.iter().enumerate() {
        p_str.push(match p {
            0 => 'A',
            1 => 'B',
            _ => 'C',
        });
        if i != pattern.len() - 1 {
            p_str.push(',');
        }
    }
    let mut sub_str = [String::new(), String::new(), String::new()];
    for index in 0..3 {
        for (i, p) in subs[index].iter().enumerate() {
            match p {
                Instruction::Forward(step) => sub_str[index].push_str(&format!("{}", step)),
                Instruction::Left => sub_str[index].push('L'),
                Instruction::Right => sub_str[index].push('R'),
            }
            if i != subs[index].len() - 1 {
                sub_str[index].push(',');
            }
        }
    }
    format!("{}\n{}\n{}\n{}\nn\n", p_str, sub_str[0], sub_str[1], sub_str[2])
}

fn string_to_i64(s: String) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    for c in s.chars() {
        result.push(c as i64)
    }
    result
}

fn main() {
    let code = aoc::parse_file::<i64>("input.txt", ",");
    let grid_data = IntProgram::new(code.clone()).execute(Vec::new());
    let (grid, position, direction) = read_grid(grid_data.as_slice());
    println!("intersection sum {}", calculate_intersection(&grid));
    let instruction = generate_instructions(&grid, position, direction);
    let sub_instruction = generate_sub_instructions(&instruction);
    if let Some((pattern, sub_task)) = find_combination(instruction.as_slice(), sub_instruction.as_slice()) {
        let input = string_to_i64(generate_input(pattern.as_slice(), sub_task));
        let mut program = IntProgram::new(code);
        program.set(0, 2);
        if let Some(e) = program.execute(input).last() {
            println!("dust {}", e);
        }
    }
}

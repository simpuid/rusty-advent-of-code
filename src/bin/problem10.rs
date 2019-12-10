extern crate aoc;
use aoc::vector::Vector;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Copy, Clone)]
struct Asteroid {
    pos: Vector,
    los: i32,
}

fn check_los(a: &Asteroid, b: &Asteroid, ass: &[Asteroid]) -> bool {
    for c in ass.iter() {
        if c.pos != a.pos && c.pos != b.pos {
            let ca = c.pos - a.pos;
            let ba = b.pos - a.pos;
            if Vector::collinear(a.pos, b.pos, c.pos) && ca * ba >= 0 && ca.sqr_mag() < ba.sqr_mag() {
                return false;
            }
        }
    }
    true
}

fn get_angle(v: Vector) -> f32 {
    let angle = (v.y as f32).atan2(v.x as f32);
    (std::f32::consts::FRAC_PI_2 - angle + std::f32::consts::PI * 2f32) % (std::f32::consts::PI * 2f32)
}

fn main() {
    let lines = aoc::parse_file::<String>("input.txt", "\n");
    let mut ass: Vec<Asteroid> = Vec::new();
    for (y, string) in lines.into_iter().enumerate() {
        for (x, c) in string.chars().enumerate() {
            if c == '#' {
                ass.push(Asteroid {
                    pos: Vector::new(x as i32, -(y as i32)),
                    los: 0,
                });
            }
        }
    }
    for i in 0..ass.len() {
        for j in (i + 1)..ass.len() {
            if check_los(&ass[i], &ass[j], &ass) {
                ass.get_mut(i).expect("index error").los += 1;
                ass.get_mut(j).expect("index error").los += 1;
            }
        }
    }

    let mut target = ass.first().expect("empty").pos;
    let mut max = ass.first().expect("empty").los;
    for a in ass.iter() {
        if max < a.los {
            max = a.los;
            target = a.pos
        }
    }
    println!("max los {} at ({}, {})", max, target.x, target.y);

    let mut map: HashMap<Vector, Vec<Vector>> = HashMap::new();
    for a in ass.into_iter() {
        if (a.pos - target).sqr_mag() != 0 {
            map.entry((a.pos - target).direction()).or_insert_with(|| vec![a.pos - target]);
        }
    }

    let mut last: Vec<(Vector, Vec<Vector>)> = Vec::new();
    for (key, value) in map.into_iter() {
        last.push((key, value));
    }

    let mut sorted = false;
    while !sorted {
        sorted = true;
        for i in 0..(last.len() - 1) {
            let (v1, _) = last[i];
            let (v2, _) = last[i + 1];
            if get_angle(v2) < get_angle(v1) {
                last.swap(i, i + 1);
                sorted = false;
            }
        }
    }

    for (_, vec) in last.iter_mut() {
        let mut sorted = false;
        while !sorted {
            sorted = true;
            for i in 0..(vec.len() - 1) {
                if vec[i].sqr_mag() < vec[i + 1].sqr_mag() {
                    vec.swap(i, i + 1);
                    sorted = false;
                }
            }
        }
    }

    let mut count = 0;
    let mut empty = false;
    while !empty {
        empty = true;
        for (_, vec) in last.iter_mut() {
            if let Some(e) = vec.pop() {
                let tmp = e + target;
                println!("count {}: ({}, {})", count, tmp.x, -tmp.y);
                count += 1;
                empty = false;
            }
        }
    }
}

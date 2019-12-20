extern crate aoc;
use std::fs;

use aoc::vector::Vector;
use std::collections::{HashMap, HashSet, VecDeque};

fn read_file() -> (HashSet<Vector>, HashMap<Vector, Vector>, Vector, Vector) {
    let input = fs::read_to_string("input.txt").unwrap();
    let strings: Vec<&str> = input.split('\n').collect();
    let mut chars: Vec<char> = Vec::new();
    let width = strings.first().unwrap().len();
    let height = strings.len();
    for s in strings {
        for c in s.chars() {
            chars.push(c);
        }
    }
    let mut open: HashSet<Vector> = HashSet::new();
    let mut tmp_portals: HashMap<[char; 2], Vec<Vector>> = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            let head = Vector::new(x as i32, y as i32);
            match chars.get(x + y * width) {
                Some('.') => {
                    open.insert(head);
                }
                Some(c) => {
                    if c.is_alphabetic() {
                        let checks = [
                            (Vector::new(1, 0), Vector::new(2, 0)),
                            (Vector::new(1, 0), Vector::new(-1, 0)),
                            (Vector::new(0, 1), Vector::new(0, 2)),
                            (Vector::new(0, 1), Vector::new(0, -1)),
                        ];
                        for (mut other_off, mut tile_off) in checks.iter() {
                            other_off = other_off + head;
                            tile_off = tile_off + head;
                            if tile_off.x >= 0 && tile_off.y >= 0 && other_off.x >= 0 && other_off.y >= 0 {
                                if let Some(c2) = chars.get(other_off.x as usize + other_off.y as usize * width) {
                                    if c2.is_alphabetic() {
                                        if let Some('.') = chars.get(tile_off.x as usize + tile_off.y as usize * width) {
                                            tmp_portals.entry([*c, *c2]).or_insert_with(Vec::new).push(tile_off);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }

    let mut portals: HashMap<Vector, Vector> = HashMap::new();
    for (_, j) in tmp_portals.iter() {
        if let (Some(a), Some(b)) = (j.get(0), j.get(1)) {
            portals.insert(*a, *b);
            portals.insert(*b, *a);
        }
    }
    (
        open,
        portals,
        *tmp_portals.get(&['A', 'A']).unwrap().first().unwrap(),
        *tmp_portals.get(&['Z', 'Z']).unwrap().first().unwrap(),
    )
}

fn calculate_outer_bound(portals: &HashMap<Vector, Vector>, start: Vector, end: Vector) -> (Vector, Vector) {
    let mut min = Vector::new(start.x.min(end.x), start.y.min(end.y));
    let mut max = Vector::new(start.x.max(end.x), start.y.max(end.y));
    for portal in portals.keys() {
        min.x = min.x.min(portal.x);
        min.y = min.y.min(portal.y);
        max.x = max.x.max(portal.x);
        max.y = max.y.max(portal.y);
    }
    (min, max)
}

fn is_outer(pos: Vector, min: Vector, max: Vector) -> bool {
    (pos.x == min.x) || (pos.x == max.x) || (pos.y == min.y) || (pos.y == max.y)
}

fn main() {
    let (open, portals, start, end) = read_file();
    let (min, max) = calculate_outer_bound(&portals, start, end);
    let mut visited: HashSet<(Vector, i32)> = HashSet::new();
    let mut queue: VecDeque<((Vector, i32), usize)> = VecDeque::new();
    visited.insert((start, 0));
    queue.push_back(((start, 0), 0));
    let mut wrap_found = false;
    while let Some(((pos, level), step)) = queue.pop_front() {
        if pos == end {
            if !wrap_found {
                println!("wrapped steps {}", step);
                wrap_found = true;
            }
            if level == 0 {
                println!("physical steps {}", step);
                break;
            }
        }
        let offsets = vec![Vector::new(1, 0), Vector::new(-1, 0), Vector::new(0, 1), Vector::new(0, -1)];
        for off in offsets.into_iter() {
            if !visited.contains(&(pos + off, level)) {
                visited.insert((pos + off, level));
                if open.contains(&(pos + off)) {
                    queue.push_back(((pos + off, level), step + 1));
                }
            }
        }
        if let Some(warp_pos) = portals.get(&(pos)) {
            let new_level = level - if is_outer(pos, min, max) { 1 } else { -1 };
            let warp = (*warp_pos, new_level);
            if new_level >= 0 && !visited.contains(&warp) {
                visited.insert(warp);
                if open.contains(&warp_pos) {
                    queue.push_back((warp, step + 1));
                }
            }
        }
    }
}

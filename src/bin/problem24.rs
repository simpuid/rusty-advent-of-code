extern crate aoc;

use std::collections::{HashMap, HashSet};

type Grid = [bool; 25];
type List = [Vec<(isize, usize)>; 25];
type GridMap = HashMap<isize, Grid>;

fn create_grid() -> Grid {
    let mut result = [false; 25];
    let lines = aoc::parse_file::<String>("input.txt", "\n");
    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                if let Some(e) = result.get_mut(x + y * 5) {
                    *e = true;
                }
            }
        }
    }
    result
}

fn calculate_bio(g: &[bool; 25]) -> u32 {
    let mut res = 0;
    let mut mask = 1;
    for v in g.iter() {
        if *v {
            res |= mask;
        }
        mask <<= 1;
    }
    res
}

fn count(g: &[bool; 25], v: bool) -> usize {
    let mut res = 0;
    for val in g.iter() {
        if *val == v {
            res += 1;
        }
    }
    res
}

fn simulate(mut grids: GridMap, list: &List) -> GridMap {
    let mut levels: Vec<isize> = Vec::new();
    for k in grids.keys() {
        levels.push(*k);
    }
    for l in levels {
        grids.entry(l + 1).or_insert([false; 25]);
        grids.entry(l - 1).or_insert([false; 25]);
    }
    let mut new_grids: GridMap = HashMap::new();

    for lvl in grids.keys() {
        for y in 0..5 {
            for x in 0..5 {
                if let Some(list) = list.get(x + y * 5) {
                    let mut count = 0;
                    for (lvl_off, index) in list {
                        if get(&grids, *lvl + *lvl_off, *index) {
                            count += 1;
                        }
                    }
                    if get(&grids, *lvl, x + y * 5) {
                        if count == 1 {
                            set(&mut new_grids, *lvl, x + y * 5, true);
                        }
                    } else if count == 1 || count == 2 {
                        set(&mut new_grids, *lvl, x + y * 5, true);
                    }
                }
            }
        }
    }
    new_grids
}

fn get(grids: &GridMap, level: isize, index: usize) -> bool {
    if let Some(g) = grids.get(&level) {
        if let Some(val) = g.get(index) {
            return *val;
        }
    }
    false
}

fn set(grids: &mut GridMap, lvl: isize, index: usize, val: bool) {
    let g = grids.entry(lvl).or_insert([false; 25]);
    if let Some(v) = g.get_mut(index) {
        *v = val;
    }
}

fn main() {
    let grid = create_grid();
    let list1 = adj_list();
    println!("{:?}", list1);
    let mut grids: GridMap = HashMap::new();
    grids.insert(0, grid);
    let mut set: HashSet<u32> = HashSet::new();
    loop {
        grids = simulate(grids, &list1);
        if let Some(e) = grids.get(&0) {
            let bio = calculate_bio(e);
            if set.contains(&bio) {
                println!("repeat bio {}", bio);
                break;
            }
            set.insert(bio);
        }
    }

    let mut grids: GridMap = HashMap::new();
    grids.insert(0, grid);
    let rec_list = adj_list_recursive();

    for _ in 0..200 {
        grids = simulate(grids, &rec_list);
    }

    let mut total = 0;
    for (_, g) in grids {
        total += count(&g, true);
    }
    println!("total bugs {}", total);
}

fn adj_list() -> List {
    let mut list: List = Default::default();
    for xx in 0..5 {
        for yy in 0..5 {
            if let Some(v) = list.get_mut(xx + yy * 5) {
                let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                for (x_off, y_off) in offsets.iter() {
                    let (x, y) = (xx as i32 + *x_off, yy as i32 + *y_off);
                    if x >= 0 && x < 5 && y >= 0 && y < 5 {
                        v.push((0, (x + y * 5) as usize));
                    }
                }
            }
        }
    }
    list
}

fn adj_list_recursive() -> List {
    let mut list: List = Default::default();
    for xx in 0..5 {
        for yy in 0..5 {
            if let Some(v) = list.get_mut(xx + yy * 5) {
                let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                for (x_off, y_off) in offsets.iter() {
                    let (x, y) = (xx as i32 + *x_off, yy as i32 + *y_off);
                    if x >= 0 && x < 5 && y >= 0 && y < 5 && x + y * 5 != 12 {
                        v.push((0, (x + y * 5) as usize));
                    }
                }
            }
        }
    }
    list[12].clear();
    for i in 0..5 {
        list[11].push((1, i * 5));
        list[13].push((1, 4 + i * 5));
        list[7].push((1, i));
        list[17].push((1, 20 + i));

        list[i].push((-1, 7));
        list[20 + i].push((-1, 17));
        list[i * 5].push((-1, 11));
        list[4 + 5 * i].push((-1, 13));
    }

    list
}

extern crate aoc;

use aoc::vector::Vector;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum Cell {
    Empty,
    Block,
    Key(u32),
    Door(u32),
}

#[derive(Debug, Clone)]
struct Maze {
    width: usize,
    height: usize,
    data: Vec<Cell>,
    visited: Vec<bool>,
}

impl Maze {
    fn get(&self, vec: Vector) -> Cell {
        if vec.x < 0 || vec.y < 0 {
            return Cell::Block;
        }
        if let Some(e) = self.data.get(vec.x as usize + vec.y as usize * self.width) {
            return *e;
        }
        Cell::Block
    }

    fn get_path(&self, start: Vector, end: Vector) -> Option<(u32, usize)> {
        let mut visited: HashSet<Vector> = HashSet::new();
        let mut queue: VecDeque<(Vector, u32, usize)> = VecDeque::new();
        visited.insert(start);
        queue.push_back((start, 0, 0));
        while let Some((position, bunch, step)) = queue.pop_front() {
            if position == end {
                return Some((bunch, step));
            }
            let offsets = [Vector::new(1, 0), Vector::new(-1, 0), Vector::new(0, 1), Vector::new(0, -1)];
            for off in offsets.iter() {
                let pos = position + *off;
                if !visited.contains(&pos) {
                    visited.insert(pos);
                    match self.get(pos) {
                        Cell::Empty | Cell::Key(_) => {
                            queue.push_back((pos, bunch, step + 1));
                        }
                        Cell::Door(k) => {
                            queue.push_back((pos, add(bunch, k), step + 1));
                        }
                        Cell::Block => (),
                    }
                }
            }
        }
        None
    }
}

fn add(bunch: u32, key: u32) -> u32 {
    key | bunch
}

fn count_keys(bunch: u32) -> usize {
    let mut sum = 0;
    let mut index: u32 = 1;
    while index != 0 {
        if bunch & index != 0 {
            sum += 1;
        }
        index <<= 1
    }
    sum
}

fn generate_maze(data: &[String]) -> (Maze, HashMap<u32, Vector>, Vec<Vector>) {
    let mut keys: HashMap<u32, Vector> = HashMap::new();
    let mut cells: Vec<Cell> = Vec::new();
    let mut visited: Vec<bool> = Vec::new();
    let mut header = Vector::new(0, 0);
    let mut start: Vec<Vector> = Vec::new();
    for string in data {
        for c in string.chars() {
            match c {
                '#' => {
                    cells.push(Cell::Block);
                    visited.push(true);
                }
                '.' => {
                    cells.push(Cell::Empty);
                    visited.push(false);
                }
                '@' => {
                    start.push(header);
                    cells.push(Cell::Empty);
                    visited.push(false);
                }
                e => {
                    let code: u32 = 1u32 << (e.to_ascii_lowercase() as u8 - b'a') as u32;
                    if e.is_uppercase() {
                        cells.push(Cell::Door(code));
                    } else {
                        cells.push(Cell::Key(code));
                        keys.insert(code, header);
                    }
                    visited.push(false);
                }
            }
            header.x += 1;
        }
        header.y += 1;
        header.x = 0;
    }
    (
        Maze {
            width: data.first().unwrap().len(),
            height: header.y as usize,
            data: cells,
            visited,
        },
        keys,
        start,
    )
}

fn get_min(set: &mut BTreeSet<State>) -> Option<State> {
    let result = match set.iter().next() {
        Some((bunch, pos)) => Some((*bunch, pos.clone())),
        None => None,
    };
    if let Some(e) = &result {
        set.remove(e);
    }
    result
}

fn compute_paths(maze: &Maze, keys: &HashMap<u32, Vector>, starts: &[Vector]) -> HashMap<(Vector, Vector), (u32, usize)> {
    let mut result: HashMap<(Vector, Vector), (u32, usize)> = HashMap::new();
    for (i, i_pos) in keys.iter() {
        for (j, j_pos) in keys.iter() {
            if *i > *j {
                if let Some(e) = maze.get_path(*i_pos, *j_pos) {
                    result.insert((*i_pos, *j_pos), e);
                }
            }
        }
    }

    for start in starts {
        for (_, k_pos) in keys.iter() {
            if let Some(e) = maze.get_path(*start, *k_pos) {
                result.insert((*start, *k_pos), e);
            }
        }
    }

    result
}

fn get_path(path: &HashMap<(Vector, Vector), (u32, usize)>, a: Vector, b: Vector) -> Option<(u32, usize)> {
    if let Some(e) = path.get(&(a, b)) {
        Some(*e)
    } else if let Some(e) = path.get(&(b, a)) {
        Some(*e)
    } else {
        None
    }
}

type State = (u32, Vec<Vector>);

fn main() {
    let maze_data: Vec<String> = aoc::parse_file("input.txt", "\n");
    let (maze, keys, starts) = generate_maze(maze_data.as_slice());
    let paths = compute_paths(&maze, &keys, &starts);
    println!("precomputed {} paths", paths.len());

    let mut map: HashMap<State, usize> = HashMap::new();
    let mut set: BTreeSet<State> = BTreeSet::new();

    map.insert((0, starts.clone()), 0);
    set.insert((0, starts));

    let mut min = !0usize;
    let max_keys = keys.len();
    while let Some(state) = get_min(&mut set) {
        let step = *map.get(&state).unwrap();
        let (bunch, pos) = state;
        let bunch_count = count_keys(bunch);
        if bunch_count == max_keys && min > step {
            min = step;
        }
        for i in 0..pos.len() {
            if let Some(poss) = pos.get(i) {
                for (k, k_pos) in keys.iter() {
                    if *k_pos != *poss && add(*k, bunch) != bunch {
                        if let Some((req_bunch, step_inc)) = get_path(&paths, *poss, *k_pos) {
                            if add(req_bunch, bunch) == bunch {
                                let mut new_pos = pos.clone();
                                if let Some(e) = new_pos.get_mut(i) {
                                    *e = *k_pos;
                                }
                                let new_state = (add(bunch, *k), new_pos);

                                if let Some(mut_step) = map.get_mut(&new_state) {
                                    if *mut_step > step + step_inc {
                                        *mut_step = step + step_inc;
                                        set.insert(new_state);
                                    }
                                } else {
                                    map.insert(new_state.clone(), step + step_inc);
                                    set.insert(new_state);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("min step {}", min);
}

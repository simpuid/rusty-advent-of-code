extern crate aoc;
use aoc::int_code::IntProgram;
use aoc::vector::Vector;
use std::collections::{HashSet, VecDeque};

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}

impl Direction {
    fn to_vector(self) -> Vector {
        match self {
            Direction::Up => Vector::new(0, 1),
            Direction::Down => Vector::new(0, -1),
            Direction::Left => Vector::new(-1, 0),
            Direction::Right => Vector::new(1, 0),
        }
    }

    fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone)]
struct Droid {
    program: IntProgram,
    position: Vector,
    steps: u32,
}

impl Droid {
    fn new(code: Vec<i64>) -> Droid {
        Droid {
            program: IntProgram::new(code),
            position: Vector::new(0, 0),
            steps: 0,
        }
    }

    fn move_droid(&mut self, dir: Direction) -> Option<bool> {
        if let Some(code) = self.program.execute(vec![dir as i64]).first() {
            if *code != 0 {
                self.position = self.position + dir.to_vector();
                self.steps += 1;
                return Some(*code == 2);
            }
        }
        None
    }

    fn check(&mut self, dir: Direction) -> Option<bool> {
        if let Some(code) = self.program.execute(vec![dir as i64]).first() {
            if *code != 0 {
                self.program.execute(vec![dir.opposite() as i64]);
                return Some(*code == 2);
            }
        }
        None
    }
}

fn main() {
    let droid = Droid::new(aoc::parse_file::<i64>("input.txt", ","));
    let mut visited: HashSet<Vector> = HashSet::new();
    visited.insert(Vector::new(0, 0));
    let mut stack: VecDeque<Droid> = VecDeque::new();
    stack.push_back(droid.clone());
    let mut droid_at_oxygen: Droid = droid.clone();
    'bfs: while let Some(mut droid) = stack.pop_front() {
        let dir = [Direction::Right, Direction::Left, Direction::Up, Direction::Down];
        for d in &dir {
            let new_pos = droid.position + d.to_vector();
            if !visited.contains(&new_pos) {
                visited.insert(new_pos);
                if let Some(e) = droid.check(*d) {
                    let mut new_droid = droid.clone();
                    new_droid.move_droid(*d);
                    if !e {
                        stack.push_back(new_droid);
                    } else {
                        droid_at_oxygen = new_droid;
                        break 'bfs;
                    }
                }
            }
        }
    }
    println!("found oxygen @ steps:{}", droid_at_oxygen.steps);

    droid_at_oxygen.steps = 0;
    visited.clear();
    stack.clear();
    stack.push_back(droid_at_oxygen);
    let mut time: u32 = 0;
    while let Some(mut droid) = stack.pop_front() {
        time = time.max(droid.steps);
        let dir = [Direction::Right, Direction::Left, Direction::Up, Direction::Down];
        for d in &dir {
            let new_pos = droid.position + d.to_vector();
            if !visited.contains(&new_pos) {
                visited.insert(new_pos);
                if let Some(_) = droid.check(*d) {
                    let mut new_droid = droid.clone();
                    new_droid.move_droid(*d);
                    stack.push_back(new_droid);
                }
            }
        }
    }
    println!("max time {}", time);
}

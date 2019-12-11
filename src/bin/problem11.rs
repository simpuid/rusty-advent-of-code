extern crate aoc;
use aoc::int_code::IntProgram;
use aoc::vector::Vector;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

#[derive(PartialEq)]
enum Color {
    Black,
    White,
}

struct Robot {
    position: Vector,
    direction: Vector,
}

impl Robot {
    fn rotate_left(&mut self) {
        self.direction = Vector::new(-self.direction.y, self.direction.x)
    }
    fn rotate_right(&mut self) {
        self.direction = Vector::new(self.direction.y, -self.direction.x)
    }
    fn move_forward(&mut self) {
        self.position = self.position + self.direction;
    }
}

fn paint_to_console(map: &HashMap<Vector, Color>) {
    if let Some((first, _)) = map.iter().next() {
        let (mut min_pos, mut max_pos) = (*first, *first);
        for (pos, _) in map.iter() {
            min_pos = Vector::new(min(min_pos.x, pos.x), min(min_pos.y, pos.y));
            max_pos = Vector::new(max(max_pos.x, pos.x), max(max_pos.y, pos.y));
        }
        for y in (min_pos.y..=max_pos.y).rev() {
            for x in min_pos.x..=max_pos.x {
                if let Some(e) = map.get(&Vector::new(x, y)) {
                    if *e == Color::White {
                        print!("##");
                        continue;
                    }
                }
                print!("  ");
            }
            println!()
        }
    }
}

fn main() {
    let mut code = IntProgram::new(aoc::parse_file::<i64>("input.txt", ","));
    let mut map: HashMap<Vector, Color> = HashMap::new();
    map.insert(Vector::new(0, 0), Color::White);
    let mut robot = Robot {
        position: Vector::new(0, 0),
        direction: Vector::new(0, 1),
    };
    while code.can_run() {
        let input = map.get(&robot.position).unwrap_or(&Color::Black);
        let result = code.execute(vec![match input {
            Color::Black => 0,
            Color::White => 1,
        }]);
        if let (Some(pt), Some(dir)) = (result.get(0), result.get(1)) {
            let color: Color = match pt {
                0 => Color::Black,
                _ => Color::White,
            };
            *map.entry(robot.position).or_insert(Color::Black) = color;
            if *dir == 0 {
                robot.rotate_left();
            } else {
                robot.rotate_right();
            }

            robot.move_forward();
        }
    }
    println!("terminate paint_count:{}", map.iter().count());
    paint_to_console(&map);
}

extern crate aoc;

enum Direction {
    Horizontal,
    Vertical,
}

struct Line {
    x: i32,
    y: i32,
    len: i32,
    direction: Direction,
}

impl Line {
    fn new(x: i32, y: i32, len: i32, direction: Direction) -> Line {
        Line { x, y, len, direction }
    }
}

fn abs(val: i32) -> i32 {
    if val > 0 {
        val
    } else {
        -val
    }
}

fn range_check(val: i32, bound1: i32, bound2: i32) -> bool {
    if bound1 < bound2 {
        (val >= bound1) && (val <= bound2)
    } else {
        (val >= bound2) && (val <= bound1)
    }
}

fn intersect(l1: &Line, l2: &Line) -> Option<(i32, i32)> {
    let (l1, l2): (&Line, &Line) = match (&l1.direction, &l2.direction) {
        (Direction::Vertical, Direction::Horizontal) => (l1, l2),
        (Direction::Horizontal, Direction::Vertical) => (l2, l1),
        _ => return None,
    };
    if range_check(l1.x, l2.x, l2.x + l2.len) && range_check(l2.y, l1.y, l1.y + l1.len) {
        Some((l1.x, l2.y))
    } else {
        None
    }
}

fn convert_wire(code: &String) -> Vec<Line> {
    let mut wire: Vec<Line> = Vec::new();
    let iter = code.split(",");
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for s in iter {
        let mut char_iter = s.chars();
        let prefix: char = char_iter.next().expect("incorrect input");
        let len: i32 = char_iter.collect::<String>().trim().parse::<i32>().expect("parsing error");
        wire.push(match prefix {
            'L' => Line::new(x, y, -len, Direction::Horizontal),
            'R' => Line::new(x, y, len, Direction::Horizontal),
            'U' => Line::new(x, y, -len, Direction::Vertical),
            'D' => Line::new(x, y, len, Direction::Vertical),
            _ => panic!("unknown prefix"),
        });
        x += match prefix {
            'L' => -len,
            'R' => len,
            _ => 0,
        };
        y += match prefix {
            'U' => -len,
            'D' => len,
            _ => 0,
        };
    }
    wire
}

fn main() {
    let wire_code = aoc::parse_file::<String>("input.txt", "\n");
    if let (Some(c0), Some(c1)) = (wire_code.get(0), wire_code.get(1)) {
        let (wire0, wire1) = (convert_wire(c0), convert_wire(c1));
        let mut min: i32 = i32::max_value();
        for w1 in &wire0 {
            for w2 in &wire1 {
                if let Some((x, y)) = intersect(&w1, &w2) {
                    let dis = abs(x) + abs(y);
                    if dis != 0 && dis < min {
                        min = dis;
                    }
                }
            }
        }
        println!("manhattan minimum {}", min);

        min = i32::max_value();
        let mut l1: i32 = 0;
        for w1 in &wire0 {
            let mut l2: i32 = 0;
            for w2 in &wire1 {
                if let Some((x, y)) = intersect(&w1, &w2) {
                    let dis = l1 + l2 + abs(x - w1.x) + abs(y - w1.y) + abs(x - w2.x) + abs(y - w2.y);
                    if dis != 0 && dis < min {
                        min = dis;
                    }
                };
                l2 += abs(w2.len);
            }
            l1 += abs(w1.len);
        }
        println!("steps minimum {}", min);
    }
}

extern crate aoc;

use crate::Action::{Cut, Deal, Reverse};
use std::i128;

#[derive(Debug)]
enum Action {
    Reverse,
    Cut(i128),
    Deal(i128),
}

fn read_input() -> Vec<Action> {
    let mut actions: Vec<Action> = Vec::new();
    let lines = aoc::parse_file::<String>("input.txt", "\n");
    for line in lines {
        let seg = aoc::parse_string::<String>(line.as_str(), " ");
        match seg.first().unwrap().as_str() {
            "cut" => {
                if let Ok(n) = seg.last().unwrap().parse::<i128>() {
                    actions.push(Cut(n));
                }
            }
            "deal" => {
                if "stack" == seg.last().unwrap().as_str() {
                    actions.push(Reverse);
                } else if let Ok(n) = seg.last().unwrap().parse::<i128>() {
                    actions.push(Deal(n));
                }
            }
            _ => (),
        }
    }
    actions
}

fn mod_inv(a: i128, module: i128) -> i128 {
    let mut mn = (module, a);
    let mut xy = (0, 1);

    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }

    while xy.0 < 0 {
        xy.0 += module;
    }
    xy.0
}

fn simplify_action(actions: &Vec<Action>, size: i128) -> (i128, i128) {
    let mut a = 1;
    let mut b = 0;

    for action in actions {
        match action {
            Reverse => {
                a = modulo(-a, size);
                b = modulo(-b - 1, size);
            }
            Cut(c) => {
                b = modulo(b - *c, size);
            }
            Deal(c) => {
                a = modulo(a * *c, size);
                b = modulo(b * *c, size);
            }
        }
    }

    (a, b)
}

fn modulo(a: i128, n: i128) -> i128 {
    (a % n + n) % n
}

fn invert(a: i128, b: i128, size: i128) -> (i128, i128) {
    let x = modulo(mod_inv(a, size), size);
    (x, modulo(-b * x, size))
}

fn apply(x: i128, a: i128, b: i128, m: i128, n: i128) -> i128 {
    let a_m = pow(a, m, n);
    let mut ans = modulo(b * (a_m - 1), n);
    ans = modulo(ans * mod_inv(a - 1, n), n);
    ans = modulo(ans + modulo(a_m * x, n), n);
    ans
}

fn pow(a: i128, m: i128, n: i128) -> i128 {
    if m == 0 {
        1
    } else if m % 2 == 0 {
        pow((a * a) % n, m / 2, n)
    } else {
        (pow((a * a) % n, (m - 1) / 2, n) * a) % n
    }
}

fn main() {
    let actions = read_input();
    let n = 10007;
    let target = 2019;
    let (a, b) = simplify_action(&actions, n);
    println!("{}", apply(target, a, b, 1, n));

    let n = 119_315_717_514_047;
    let m = 101_741_582_076_661;
    let target = 2020;
    let (a, b) = simplify_action(&actions, n);
    let (a_inv, b_inv) = invert(a, b, n);
    println!("{}", apply(target, a_inv, b_inv, m, n))
}

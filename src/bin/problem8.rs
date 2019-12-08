extern crate aoc;

use std::collections::VecDeque;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const SIZE: usize = WIDTH * HEIGHT;

fn get_count(layer: &[u8; SIZE], val: u8) -> usize {
    let mut result: usize = 0;
    for i in layer.iter() {
        if *i == val {
            result += 1;
        }
    }
    result
}

fn compute_pixel(layers: &Vec<[u8; SIZE]>, index: usize) -> u8 {
    let mut value: u8 = 2;
    for l in layers {
        if let Some(e) = l.get(index) {
            value = *e;
            if value != 2 {
                return value;
            }
        }
    }
    value
}

fn main() {
    let mut seq = VecDeque::from(aoc::parse_file::<u8>("input.txt", ""));
    let mut layers: Vec<[u8; SIZE]> = Vec::new();
    let layer_count = seq.len() / SIZE;
    for _ in 0..layer_count {
        let mut layer: [u8; SIZE] = [0; SIZE];
        for j in layer.iter_mut() {
            *j = seq.pop_front().expect("parse error");
        }
        layers.push(layer);
    }
    let mut layer_ref = layers.first().expect("empty layers");
    let mut min = SIZE;
    for l in &layers {
        let zero = get_count(l, 0);
        if zero < min {
            min = zero;
            layer_ref = l;
        }
    }
    println!("{}", get_count(layer_ref, 1) * get_count(layer_ref, 2));

    let mut fin: [u8; SIZE] = [0; SIZE];
    for i in 0..SIZE {
        if let Some(e) = fin.get_mut(i) {
            *e = compute_pixel(&layers, i);
        }
    }
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if let Some(e) = fin.get(y * WIDTH + x) {
                print!("{}", if *e != 0 { "##" } else { "  " });
            }
        }
        println!();
    }
}

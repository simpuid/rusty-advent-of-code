pub mod int_code;
pub mod vector;
use std::fs;

pub fn parse_string<T: std::str::FromStr>(string: &str, split_pattern: &str) -> Vec<T> {
    let mut vector: Vec<T> = Vec::new();
    for s in string.split(split_pattern) {
        if let Ok(o) = s.trim().parse::<T>() {
            vector.push(o);
        }
    }
    vector
}

pub fn parse_file<T: std::str::FromStr>(name: &str, split_pattern: &str) -> Vec<T> {
    let string: String = fs::read_to_string(name).expect("file reading error");
    parse_string::<T>(&string, split_pattern)
}

pub fn gcd(a: i32, b: i32) -> i32 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

pub fn sign(a: i32) -> i32 {
    if a < 0 {
        -1
    } else if a > 0 {
        1
    } else {
        0
    }
}

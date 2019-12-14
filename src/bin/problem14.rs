extern crate aoc;

use std::collections::{HashMap, HashSet, VecDeque};

fn parse(code: &str) -> Option<(String, i64)> {
    let mut iter = code.trim().split(' ');
    if let (Some(code), Some(name)) = (iter.next(), iter.next()) {
        if let Ok(quantity) = code.trim().parse::<i64>() {
            return Some((String::from(name.trim()), quantity));
        }
    }
    None
}

fn calculate_dependency(name: &String, set: &mut HashSet<String>, hash: &HashMap<String, (i64, Vec<(String, i64)>)>) {
    if let Some((_, vec)) = hash.get(name) {
        for (name, _) in vec {
            calculate_dependency(name, set, hash);
            set.insert(name.clone());
        }
    }
}

fn check_dependency(name: &str, dependency: &HashMap<String, HashSet<String>>, queue: &VecDeque<String>) -> bool {
    for mat in queue {
        if let Some(e) = dependency.get(mat) {
            if e.contains(name) {
                return true;
            }
        }
    }
    false
}

fn queue_merge(name: &str, quantity: i64, queue: &mut VecDeque<String>, map: &mut HashMap<String, i64>, set: &mut HashSet<String>) {
    if set.contains(name) {
        *map.entry(String::from(name)).or_insert(0) += quantity;
        return;
    }
    *map.entry(String::from(name)).or_insert(quantity) = quantity;
    queue.push_back(String::from(name));
    set.insert(String::from(name));
}

fn calculate_ore(count: i64, hash: &HashMap<String, (i64, Vec<(String, i64)>)>, dependency: &HashMap<String, HashSet<String>>) -> i64 {
    let mut queue: VecDeque<String> = VecDeque::new();
    let mut map: HashMap<String, i64> = HashMap::new();
    let mut set: HashSet<String> = HashSet::new();
    map.insert("FUEL".to_string(), count);
    queue.push_back("FUEL".to_string());
    set.insert("FUEL".to_string());
    let mut ore_count: i64 = 0;
    while let Some(name) = queue.pop_front() {
        set.remove(&name);
        if name.as_str() == "ORE" {
            if let Some(e) = map.get(&name) {
                ore_count += *e;
            }
        } else if check_dependency(&name, &dependency, &queue) {
            set.insert(name.clone());
            queue.push_back(name);
        } else if let Some(e) = hash.get(&name) {
            let (q, vec) = e;
            if let Some(quantity) = map.get(&name) {
                let factor = if quantity % *q == 0 { quantity / (*q) } else { (quantity / (*q)) + 1 };
                for (input, input_count) in vec {
                    queue_merge(input, input_count * factor, &mut queue, &mut map, &mut set);
                }
            }
        }
    }
    ore_count
}

fn main() {
    let mut hash: HashMap<String, (i64, Vec<(String, i64)>)> = HashMap::new();
    let mut dependency: HashMap<String, HashSet<String>> = HashMap::new();
    let reaction_codes = aoc::parse_file::<String>("input.txt", "\n");
    for reaction in reaction_codes {
        let mut split = reaction.split("=>");
        if let (Some(input_code), Some(output_code)) = (split.next(), split.next()) {
            if let Some(output) = parse(output_code) {
                let mut input: Vec<(String, i64)> = Vec::new();
                for s in input_code.split(',') {
                    if let Some(m) = parse(s) {
                        input.push(m);
                    }
                }
                let (name, quantity) = output;
                hash.insert(name, (quantity, input));
            }
        }
    }
    for a in hash.keys() {
        let mut set: HashSet<String> = HashSet::new();
        calculate_dependency(a, &mut set, &hash);
        dependency.insert(a.clone(), set);
    }
    println!("min ore {}", calculate_ore(1, &hash, &dependency));
    let trillion: i64 = 1_000_000_000_000;
    let mut low: i64 = 1;
    let mut high: i64 = trillion;
    while high - low != 1 {
        let mid = (low + high) / 2;
        let mid_value = calculate_ore(mid, &hash, &dependency);
        if mid_value > trillion {
            high = mid;
        } else {
            low = mid;
        }
    }
    println!("max fuel {}", low);
}

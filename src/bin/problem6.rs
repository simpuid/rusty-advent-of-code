extern crate aoc;
use std::collections::{HashMap, HashSet, VecDeque};

type Hash = HashMap<String, Vec<String>>;

fn calculate_sum(hash: &Hash, root: &str, initial_sum: usize) -> usize {
    let mut sum = initial_sum;
    if let Some(sub_planets) = hash.get(root) {
        for sub_planet in sub_planets {
            sum += calculate_sum(hash, sub_planet, initial_sum + 1);
        }
    }
    sum
}

trait Graph {
    fn add_directed(&mut self, child_name: &str, parent_name: &str);
    fn add_undirected(&mut self, one: &str, two: &str);
}

impl Graph for Hash {
    fn add_directed(&mut self, child_name: &str, parent_name: &str) {
        if self.get(child_name).is_none() {
            self.insert(String::from(child_name), Vec::new());
        }
        let parent = self.entry(String::from(parent_name)).or_insert(Vec::new());
        parent.push(String::from(child_name));
    }

    fn add_undirected(&mut self, one_name: &str, two_name: &str) {
        let one = self.entry(String::from(one_name)).or_insert(Vec::new());
        one.push(String::from(two_name));
        let two = self.entry(String::from(two_name)).or_insert(Vec::new());
        two.push(String::from(one_name));
    }
}

fn calculate_bfs(hash: &Hash, src: &str, des: &str) -> Option<usize> {
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<(String, usize)> = VecDeque::new();
    queue.push_back((String::from(src), 0));
    visited.insert(String::from(src));
    while !queue.is_empty() {
        if let Some((planet, distance)) = queue.pop_front() {
            if planet == String::from(des) {
                return Some(distance);
            }
            if let Some(adj_list) = hash.get(planet.as_str()) {
                for adj_planet in adj_list {
                    if !visited.contains(adj_planet) {
                        queue.push_back((adj_planet.clone(), distance + 1));
                        visited.insert(adj_planet.clone());
                    }
                }
            }
        }
    }
    None
}

fn main() {
    let mut hash_directed: Hash = HashMap::new();
    let mut hash_undirected: Hash = HashMap::new();
    for s in aoc::parse_file::<String>("input.txt", "\n") {
        let planets = aoc::parse_string::<String>(&s, ")");
        if let (Some(parent_name), Some(child_name)) = (planets.get(0), planets.get(1)) {
            hash_directed.add_directed(child_name, parent_name);
            hash_undirected.add_undirected(child_name, parent_name);
        }
    }
    let sum = calculate_sum(&hash_directed, "COM", 0);
    println!("total orbits:{}", sum);
    if let Some(i) = calculate_bfs(&hash_undirected, "YOU", "SAN") {
        println!("orbit jumps:{}", i - 2)
    }
}

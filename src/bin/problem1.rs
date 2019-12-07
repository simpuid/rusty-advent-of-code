extern crate aoc;

fn calculate_fuel(input: i64) -> i64 {
    (input / 3) - 2
}

fn calculate_fuel_recursive(input: i64) -> i64 {
    let mut fuel: i64 = calculate_fuel(input);
    let mut result: i64 = 0;
    while fuel > 0 {
        result += fuel;
        fuel = calculate_fuel(fuel);
    }
    result
}

fn main() {
    let mut module_fuel: i64 = 0;
    let mut total_fuel: i64 = 0;
    for mass in aoc::parse_file::<i64>("input.txt", "\n").into_iter() {
        module_fuel += calculate_fuel(mass);
        total_fuel += calculate_fuel_recursive(mass);
    }
    println!("module fuel {}", module_fuel);
    println!("total fuel {}", total_fuel);
}

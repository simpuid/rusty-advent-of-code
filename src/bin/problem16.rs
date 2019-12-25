extern crate aoc;

fn get_offset(slice: &[i32]) -> usize {
    let mut result = 0;
    let mut multiplier = 1;
    for i in slice.iter().rev() {
        result += multiplier * *i;
        multiplier *= 10;
    }
    result as usize
}

fn main() {
    let input = aoc::parse_file::<i32>("input.txt", "");
    let mut prev_signal = input.clone();
    let mut next_signal = input.clone();

    for _ in 0..100 {
        for (i, val) in next_signal.iter_mut().enumerate() {
            let interval = i + 1;
            let mut index = interval - 1;
            let mut sign = 1;
            *val = 0;
            while index < prev_signal.len() {
                for _ in 0..interval {
                    if index < prev_signal.len() {
                        *val += prev_signal[index] * sign;
                        index += 1;
                    }
                }
                index += interval;
                sign *= -1;
            }
        }
        for (i, val) in prev_signal.iter_mut().enumerate() {
            *val = next_signal[i] % 10;
            if *val < 0 {
                *val *= -1;
            }
        }
    }
    println!("{:?}", &prev_signal[0..8]);

    let offset = get_offset(&input[0..7]);
    let mut values: Vec<i32> = Vec::new();
    for i in offset..(input.len() * 10_000) {
        values.push(input[i % input.len()]);
    }
    for _ in 0..100 {
        let mut sum = 0;
        for i in values.iter_mut().rev() {
            *i = (*i + sum) % 10;
            sum = *i;
        }
    }
    println!("{:?}", &values[0..8]);
}

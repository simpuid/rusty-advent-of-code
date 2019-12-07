fn check_old_pattern(mut val: i32) -> bool {
    let mut safe: bool = false;
    let mut last_digit: i32 = 10;
    while val != 0 {
        if val % 10 > last_digit {
            return false;
        }
        if val % 10 == last_digit {
            safe = true;
        }
        last_digit = val % 10;
        val /= 10;
    }
    safe
}

fn check_new_pattern(mut val: i32) -> bool {
    let mut safe: bool = false;
    let mut last_digit: i32 = 10;
    let mut streak: i32 = 1;
    while val != 0 {
        if val % 10 > last_digit {
            return false;
        }
        if val % 10 == last_digit {
            streak += 1;
        } else {
            if streak == 2 {
                safe = true;
            }
            streak = 1;
        }
        last_digit = val % 10;
        val /= 10;
    }
    if streak == 2 {
        safe = true;
    }
    safe
}

fn main() {
    let mut count_one: i32 = 0;
    let mut count_two: i32 = 0;
    for x in 123_257..647_015 {
        if check_old_pattern(x) {
            count_one += 1;
        }
        if check_new_pattern(x) {
            count_two += 1
        }
    }
    println!("old pattern {}", count_one);
    println!("new pattern {}", count_two);
}

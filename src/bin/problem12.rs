extern crate aoc;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    r: i32,
    v: i32,
}

fn collection(v: [i32; 4]) -> [Point; 4] {
    let mut result = [Point { r: 0, v: 0 }; 4];
    for (i, p) in result.iter_mut().enumerate() {
        *p = Point { r: v[i], v: 0 };
    }
    result
}

fn simulate_axis(array: &mut [Point; 4]) {
    for i in 0..4usize {
        for j in (i + 1)..4usize {
            let delta = aoc::sign(array.get(i).unwrap().r - array.get(j).unwrap().r);
            array[i].v -= delta;
            array[j].v += delta;
        }
    }
    for p in array.iter_mut() {
        p.r += p.v;
    }
}

fn abs(a: i32) -> i32 {
    if a > 0 {
        a
    } else {
        -a
    }
}

fn equal(a: &[Point; 4], b: &[Point; 4]) -> bool {
    for i in 0..4 {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn main() {
    let mut x = collection([-9, 2, 10, -6]);
    let mut y = collection([-1, 9, 18, 15]);
    let mut z = collection([-1, 5, -12, -7]);
    let (init_x, init_y, init_z) = (x, y, z);

    for _ in 0..10 {
        simulate_axis(&mut x);
        simulate_axis(&mut y);
        simulate_axis(&mut z);
    }

    let mut energy: i32 = 0;
    for i in 0..4usize {
        energy += (abs(x[i].r) + abs(y[i].r) + abs(z[i].r)) * (abs(x[i].v) + abs(y[i].v) + abs(z[i].v));
    }
    println!("energy {}", energy);
    x = init_x;
    y = init_y;
    z = init_z;
    let mut step_x: i64 = 0;
    loop {
        simulate_axis(&mut x);
        step_x += 1;
        if equal(&x, &init_x) {
            break;
        }
    }

    let mut step_y: i64 = 0;
    loop {
        simulate_axis(&mut y);
        step_y += 1;
        if equal(&y, &init_y) {
            break;
        }
    }

    let mut step_z: i64 = 0;
    loop {
        simulate_axis(&mut z);
        step_z += 1;
        if equal(&z, &init_z) {
            break;
        }
    }
    println!("total {}", lcm(step_x, lcm(step_y, step_z)));
}

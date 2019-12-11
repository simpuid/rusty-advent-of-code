use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Vector {
        Vector { x, y }
    }

    pub fn direction(self) -> Vector {
        match (self.x, self.y) {
            (0, 0) => Vector::new(0, 0),
            (0, x) => Vector::new(0, crate::sign(x)),
            (x, 0) => Vector::new(crate::sign(x), 0),
            (_, _) => {
                let gcd = crate::gcd(self.x.abs(), self.y.abs());
                Vector::new(self.x / gcd, self.y / gcd)
            }
        }
    }

    pub fn sqr_mag(self) -> i32 {
        self.x * self.x + self.y * self.y
    }

    pub fn collinear(a: Vector, b: Vector, c: Vector) -> bool {
        a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y) == 0
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<Vector> for Vector {
    type Output = i32;

    fn mul(self, rhs: Vector) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector2Int {
    pub x: i32,
    pub y: i32,
}

impl Vector2Int {
    pub fn new(x: i32, y: i32) -> Self {
        Vector2Int {x, y}
    }
    /// Returns coordinates 0,0
    pub fn zero() -> Self {
        Self {x: 0i32, y: 0i32}
    }

    /// Calculate Length between X and Y
    pub fn length(&self) -> i32 {
        (self.x * self.x + self.y * self.y).isqrt()
    }

    /// Scaling vector so its length will be 1
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len != 0 {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            Self::zero()
        }
    }

    /// Calculates the scalar product of two vectors
    pub fn dot(&self, other: Self) -> i32 {
        self.x * other.x + self.y * other.y
    }

    /// Calculates the square of the distance between two vector points
    pub fn distance(&self, other: Self) -> i32 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2)
    }
}

impl From<(i32, i32)> for Vector2Int {
    fn from(v: (i32, i32)) -> Self {
        Self {x: v.0, y: v.1}
    }
}

impl Add for Vector2Int {
    type Output = Self;
    /// adding two vectors
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vector2Int {
    type Output = Self;
    /// subtraction of two vectors
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<i32> for Vector2Int {
    type Output = Self;
    /// multiplying a vector by a scalar
    fn mul(self, rhs: i32) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<i32> for Vector2Int {
    type Output = Self;
    /// dividing a vector by a scalar
    fn div(self, rhs: i32) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

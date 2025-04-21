use std::ops::{Add, Sub, Mul, Div};
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Pod, Zeroable)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 {x, y}
    }
    /// Returns coordinates 0,0
    pub fn zero() -> Self {
        Self {x: 0f32, y: 0f32}
    }

    /// Calculate Length between X and Y
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Scaling vector so its length will be 1
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len != 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            Self::zero()
        }
    }

    /// Calculates the scalar product of two vectors
    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Calculates the square of the distance between two vector points
    pub fn distance(&self, other: Self) -> f32 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
    
    /// Transforms Vector to NDC (Normalized Device Coordinates)
    pub fn to_ndc(&self, window: Vector2) -> [f32; 2] {
        [
            (self.x / window.x) * 2f32 - 1f32,
            1f32 - (self.y / window.y) * 2f32,
        ]
    }


    /// Calculates the angle (in radians) between two vectors
    pub fn angle_between(&self, other: &Self) -> f32 {
        let dot_product = self.dot(*other);
        let length_product = self.length() * other.length();
        (dot_product / length_product).acos() // zwróci kąt w radianach
    }

    /// Transforms Vector to Array of X and Y
    pub fn to_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from(v: (f32, f32)) -> Self {
        Self {x: v.0, y: v.1}
    }
}

impl Add for Vector2 {
    type Output = Self;
    /// adding two vectors
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vector2 {
    type Output = Self;
    /// subtraction of two vectors
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;
    /// multiplying a vector by a scalar
    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;
    /// dividing a vector by a scalar
    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

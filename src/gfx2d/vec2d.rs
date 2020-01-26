use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

// Very basic 2D vector type.
// f32his is a bit over complicated because I feature matched the C++ version javidx9 wrote.
//   Missing stuff - 
//      index operator
//      

#[derive(Debug, Clone)]
pub struct Vec2d
{
    pub x: f32,
    pub y: f32,
}

impl Vec2d {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2d { x, y }
    }

    pub fn mag(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn norm(&self) -> Self {
        let r = 1.0 / self.mag();
        Vec2d { x: self.x * r, y: self.y * r }
    }

    pub fn perp(&self) -> Self {
        Vec2d { x: -self.y, y: self.x }
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn cross(&self, rhs: &Self) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }
}

impl Add for Vec2d {
    type Output = Vec2d;

    fn add(self, rhs: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2d {
    type Output = Vec2d;

    fn sub(self, rhs: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Vec2d {
    type Output = Vec2d;

    fn mul(self, rhs: f32) -> Vec2d {
        Vec2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Vec2d {
    type Output = Vec2d;

    fn div(self, rhs: f32) -> Vec2d {
        Vec2d {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl AddAssign for Vec2d {
    fn add_assign(&mut self, rhs: Vec2d) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl SubAssign for Vec2d {
    fn sub_assign(&mut self, rhs: Vec2d) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl MulAssign<f32> for Vec2d {
    fn mul_assign(&mut self, rhs: f32) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl DivAssign<f32> for Vec2d {
    fn div_assign(&mut self, rhs: f32) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

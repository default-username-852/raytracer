use std::fmt::{Error, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3{ x, y, z }
    }

    pub fn mag(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt() //TODO: implement the square root in a more efficient way
    }

    pub fn norm(&self) -> Vector3 {
        self.clone() / self.mag()
    }

    pub fn dot(&self, vec: Vector3) -> f64 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }

    pub fn cross(&self, vec: Vector3) -> Vector3 {
        (self.y * vec.z - self.z * vec.y, self.z * vec.x - self.x * vec.z, self.x * vec.y - self.y * vec.x).into()
    }

    pub fn dist_between(vec1: &Vector3, vec2: &Vector3) -> f64 {
        (*vec1 - *vec2).mag()
    }
}

impl std::ops::Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl std::ops::Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        };
    }
}

impl std::ops::Div<f32> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs as f64,
            y: self.y / rhs as f64,
            z: self.z / rhs as f64,
        }
    }
}

impl std::ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x / rhs as f64,
            y: self.y / rhs as f64,
            z: self.z / rhs as f64,
        };
    }
}

//TODO: implement {float} * Vector3
impl std::ops::Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs as f64,
            y: self.y * rhs as f64,
            z: self.z * rhs as f64,
        }
    }
}

impl std::ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs as f64,
            y: self.y * rhs as f64,
            z: self.z * rhs as f64,
        };
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl From<(f64, f64, f64)> for Vector3 {
    fn from(arg: (f64, f64, f64)) -> Self {
        Self { x: arg.0, y: arg.1, z: arg.2 }
    }
}

impl From<[f64; 3]> for Vector3 {
    fn from(arg: [f64; 3]) -> Self {
        Self { x: arg[0], y: arg[1], z: arg[2] }
    }
}

impl From<Color> for Vector3 {
    fn from(col: Color) -> Self {
        Self {
            x: col.r,
            y: col.g,
            z: col.b,
        }
    }
}

impl From<&Color> for Vector3 {
    fn from(col: &Color) -> Self {
        Self {
            x: col.r,
            y: col.g,
            z: col.b,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            r: r.min(1.0).max(0.0),
            g: g.min(1.0).max(0.0),
            b: b.min(1.0).max(0.0),
        }
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: rhs.min(1.0).max(0.0) * self.r,
            g: rhs.min(1.0).max(0.0) * self.g,
            b: rhs.min(1.0).max(0.0) * self.b,
        }
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: (self.r + rhs.r).min(1.0).max(0.0),
            g: (self.g + rhs.g).min(1.0).max(0.0),
            b: (self.b + rhs.b).min(1.0).max(0.0),
        }
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r = (self.r + rhs.r).min(1.0).max(0.0);
        self.g = (self.g + rhs.g).min(1.0).max(0.0);
        self.b = (self.b + rhs.b).min(1.0).max(0.0);
    }
}

impl std::ops::Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r.min(1.0).max(0.0),
            g: self.g * rhs.g.min(1.0).max(0.0),
            b: self.b * rhs.b.min(1.0).max(0.0),
        }
    }
}

impl std::ops::MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r.min(1.0).max(0.0);
        self.g *= rhs.g.min(1.0).max(0.0);
        self.b *= rhs.b.min(1.0).max(0.0);
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{} {} {} ", (self.r * 255.0).floor() as isize , (self.g * 255.0).floor() as isize, (self.b * 255.0).floor() as isize)
    }
}

impl From<Vector3> for Color {
    fn from(arg: Vector3) -> Self {
        Self::new(arg.x, arg.y, arg.z)
    }
}

impl From<&Vector3> for Color {
    fn from(arg: &Vector3) -> Self {
        Self::new(arg.x, arg.y, arg.z)
    }
}

impl From<(f64, f64, f64)> for Color {
    fn from(arg: (f64, f64, f64)) -> Self {
        Self { r: arg.0, g: arg.1, b: arg.2 }
    }
}

impl From<[f64; 3]> for Color {
    fn from(arg: [f64; 3]) -> Self {
        Self { r: arg[0], g: arg[1], b: arg[2] }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Material {
    color: Color,
    reflectivity: f64,
}

impl Material {
    pub fn new<T: Into<Color>>(color: T, reflectivity: f64) -> Self {
        Self { color: color.into(), reflectivity }
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn get_reflectivity(&self) -> f64 {
        self.reflectivity
    }
}
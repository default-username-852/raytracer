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

    pub fn normalize(&self) -> Vector3 {
        self.clone() / self.mag()
    }

    pub fn dot(&self, vec: Vector3) -> f64 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum QuadraticRoots {
    None,
    One(f64),
    Two(f64, f64)
}

impl QuadraticRoots {
    pub fn has_solution(&self) -> bool {
        match self {
            QuadraticRoots::None => return false,
            QuadraticRoots::One(_) => return true,
            QuadraticRoots::Two(_, _) => return true,
        }
    }
}

struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

impl From<Vector3> for Color {
    fn from(arg: Vector3) -> Self {
        Self { r: arg.x, g: arg.y, b: arg.z }
    }
}

impl From<(f64, f64, f64)> for Color {
    fn from(_: (f64, f64, f64)) -> Self {
        Self { r: arg.0, g: arg.1, b: arg.2 }
    }
}

impl From<[f64; 3]> for Color {
    fn from(_: [f64; 3]) -> Self {
        Self { r: arg[0], g: arg[1], b: arg[2] }
    }
}

struct Material {
    color: Color,
    reflectivity: f64,
}

impl Material {
    fn new<T: Into<Color>>(color: T, reflectivity: f64) -> Self {
        Self { color: color.into(), reflectivity }
    }
}
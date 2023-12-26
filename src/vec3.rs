use std::{
    cmp::PartialOrd,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }
}

impl<T> From<(T, T, T)> for Vec3<T> {
    fn from(x: (T, T, T)) -> Vec3<T> {
        Vec3 {
            x: x.0,
            y: x.1,
            z: x.2,
        }
    }
}

impl<T> Into<[T; 3]> for Vec3<T> {
    fn into(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}

impl<T: Copy> From<[T; 3]> for Vec3<T> {
    fn from(x: [T; 3]) -> Vec3<T> {
        Vec3 {
            x: x[0],
            y: x[1],
            z: x[2],
        }
    }
}

impl<T> Into<(T, T, T)> for Vec3<T> {
    fn into(self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }
}

impl<T: Copy> TryFrom<Vec<T>> for Vec3<T> {
    type Error = &'static str;

    fn try_from(v: Vec<T>) -> Result<Self, Self::Error> {
        if v.len() == 3 {
            Ok(Vec3 {
                x: v[0],
                y: v[1],
                z: v[2],
            })
        } else {
            Err("Vec3 only accepts 3 values")
        }
    }
}

impl<T> Into<Vec<T>> for Vec3<T> {
    fn into(self) -> Vec<T> {
        vec![self.x, self.y, self.z]
    }
}

impl<T: Add<Output = T> + Mul<Output = T>> Vec3<T> {
    pub fn dot(self, b: Vec3<T>) -> T {
        dot(self, b)
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> Vec3<T> {
    pub fn dot2(self) -> T {
        dot(self, self)
    }
}

impl Vec3f {
    pub fn norm(self) -> f32 {
        norm(self)
    }

    pub fn normalize(self) -> Vec3f {
        normalize(self)
    }

    pub fn interpolate(self, v: Vec3f, k: f32) -> Vec3f {
        interpolate(self, v, k)
    }
}

impl<T: Add<Output = T> + Copy> Add<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: T) -> Vec3<T> {
        Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl<T: Add<Output = T>> Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Sub<Output = T> + Copy> Sub<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: T) -> Vec3<T> {
        Vec3::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

impl<T: Sub<Output = T>> Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Vec3<T> {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T: Mul<Output = T>> Mul<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<&Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Vec3<T> {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T: Div<Output = T>> Div<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

pub fn dot<T: Add<Output = T> + Mul<Output = T>>(a: Vec3<T>, b: Vec3<T>) -> T {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn maximum<T: PartialOrd>(a: Vec3<T>, b: Vec3<T>) -> Vec3<T> {
    let (xa, ya, za) = a.into();
    let (xb, yb, zb) = b.into();
    let x = if xa > xb { xa } else { xb };
    let y = if ya > yb { ya } else { yb };
    let z = if za > zb { za } else { zb };
    Vec3::new(x, y, z)
}

pub fn minimum<T: PartialOrd>(a: Vec3<T>, b: Vec3<T>) -> Vec3<T> {
    let (xa, ya, za) = a.into();
    let (xb, yb, zb) = b.into();
    let x = if xa < xb { xa } else { xb };
    let y = if ya < yb { ya } else { yb };
    let z = if za < zb { za } else { zb };
    Vec3::new(x, y, z)
}

pub fn norm(v: Vec3f) -> f32 {
    f32::sqrt(v.x * v.x + v.y * v.y + v.z * v.z)
}

pub fn normalize(v: Vec3f) -> Vec3f {
    let norm = v.norm();
    if norm != 0.0 {
        v / norm
    } else {
        v
    }
}

pub fn interpolate(v1: Vec3f, v2: Vec3f, k: f32) -> Vec3f {
    let a = 1.0 - k;
    Vec3::new(
        v1.x * k + v2.x * a,
        v1.y * k + v2.y * a,
        v1.z * k + v2.z * a,
    )
}

pub type Vec3f = Vec3<f32>;

#[cfg(test)]
mod tests {
    use super::{dot, Vec3f};

    #[test]
    fn dot_two() {
        let v1 = Vec3f::new(1., 2., 3.);
        let v2 = Vec3f::new(4., 5., 6.);
        assert_eq!(dot(v1, v2), 32.);
    }

    #[test]
    fn minimum_two() {
        let v1 = Vec3f::new(1., 2., 3.);
        let v2 = Vec3f::new(4., 5., 6.);
        assert_eq!(dot(v1, v2), 32.);
    }
}

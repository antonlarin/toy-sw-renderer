use std::ops::{Add, Mul, Sub};

const EPSILON: f32 = 1e-7;

#[derive(Clone, Copy)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}

impl Point2 {
    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Sub for Point2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
}

#[derive(Clone, Copy)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub fn origin() -> Self {
        Point3 { x: 0f32, y: 0f32, z: 0f32 }
    }

    pub fn from_slice(coord: &[f32]) -> Self {
        assert!(coord.len() == 3);
        Point3 { x: coord[0], y: coord[1], z: coord[2] }
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3 { x: self.x, y: self.y, z: self.z }
    }
}

impl From<Vec3> for Point3 {
    fn from(v: Vec3) -> Self {
        Point3 { x: v.x, y: v.y, z: v.z }
    }
}

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let norm = self.norm();
        assert!(norm > EPSILON);
        *self * norm.recip()
    }

    pub fn orthogonalize(&self, ref_vec: Self) -> Option<Self> {
        assert!(ref_vec.norm() > EPSILON);
        let collinear_portion = self.dot(ref_vec) * ref_vec;
        let res = *self - collinear_portion;
        if res.norm() < EPSILON {
            None
        } else {
            Some(res)
        }
    }
}

// TODO: add impl Add<Point3> for Vec3 and Add<Vec3> for Point3

impl From<Point3> for Vec3 {
    fn from(p: Point3) -> Self {
        Self { x: p.x, y: p.y, z: p.z }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}

pub struct BndBox2 {
    pub min: Point2,
    pub max: Point2,
}

impl BndBox2 {
    pub fn new_empty() -> Self {
        Self { min: Point2::origin(), max: Point2::origin() }
    }

    pub fn add_point(&mut self, pnt: Point2) {
        if pnt.x < self.min.x { self.min.x = pnt.x; }
        else if pnt.x > self.max.x { self.max.x = pnt.x; }

        if pnt.y < self.min.y { self.min.y = pnt.y; }
        else if pnt.y > self.max.y { self.max.y = pnt.y; }
    }

    pub fn center(&self) -> Point2 {
        Point2 { x: 0.5 * (self.min.x + self.max.x), y: 0.5 * (self.min.y + self.max.y) }
    }
}


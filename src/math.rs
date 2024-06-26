use std::ops::{Add, Div, Mul, Sub};

const EPSILON: f32 = 1e-7;

#[derive(Clone, Copy, Debug)]
pub struct Point2<S> {
    pub x: S,
    pub y: S,
}

impl<S> Point2<S> where S: Default {
    pub fn origin() -> Self {
        Self { x: S::default(), y: S::default() }
    }

    pub fn from_slice(coord: &[S]) -> Self {
        assert!(coord.len() == 2);
        Self { x: coord[0], y: coord[1] }
    }
}

impl<> From<Point2<f32>> for Point2<i32> {
    fn from(value: Point2<f32>) -> Self {
        Self { x: value.x as i32, y: value.y as i32 }
    }
}

impl<S> Sub for Point2<S> where S: Sub<Output = S> {
    type Output = Vec2<S>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<S> Add<Vec2<S>> for Point2<S> where S: Add<Output = S> {
    type Output = Point2<S>;

    fn add(self, rhs: Vec2<S>) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<S> Sub<Vec2<S>> for Point2<S> where S: Sub<Output = S> {
    type Output = Point2<S>;

    fn sub(self, rhs: Vec2<S>) -> Self::Output {
        Self::Output { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

pub type Point2f = Point2<f32>;
pub type Point2i = Point2<i32>;

pub struct Vec2<S> {
    pub x: S,
    pub y: S,
}

pub type Vec2f = Vec2<f32>;
pub type Vec2i = Vec2<i32>;

#[derive(Clone, Copy, Debug)]
pub struct Point3<S> {
    pub x: S,
    pub y: S,
    pub z: S,
}

impl<S> Point3<S> where
    S: Default + Copy {
    pub fn origin() -> Self {
        Self { x: S::default(), y: S::default(), z: S::default() }
    }

    pub fn drop_z(&self) -> Point2<S> {
        Point2 { x: self.x, y: self.y }
    }
}

impl<S> Sub<Point3<S>> for Point3<S> where
    S: Sub<Output = S> {
    type Output = Vec3<S>;

    fn sub(self, rhs: Point3<S>) -> Self::Output {
        Self::Output {
            x: rhs.x - self.x,
            y: rhs.y - self.y,
            z: rhs.z - self.z
        }
    }
}

impl<S> From<Vec3<S>> for Point3<S> {
    fn from(v: Vec3<S>) -> Self {
        Point3 { x: v.x, y: v.y, z: v.z }
    }
}

impl<S> From<&[S]> for Point3<S> where S: Copy {
    fn from(coord: &[S]) -> Self {
        assert!(coord.len() == 3);
        Self { x: coord[0], y: coord[1], z: coord[2] }
    }
}

pub type Point3f = Point3<f32>;
pub type Point3i = Point3<i32>;

#[derive(Clone, Copy, Debug)]
pub struct Vec3<S> {
    pub x: S,
    pub y: S,
    pub z: S,
}

pub type Vec3f = Vec3<f32>;
pub type Vec3i = Vec3<i32>;

impl<S> Vec3<S> where
    S: Mul<Output = S> +
       Add<Output = S> +
       Sub<Output = S> +
       Copy {

    pub fn dot(&self, other: Self) -> S {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Vec3<f32> {
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

impl<S> From<Point3<S>> for Vec3<S> {
    fn from(p: Point3<S>) -> Self {
        Self { x: p.x, y: p.y, z: p.z }
    }
}

impl<S> From<&[S]> for Vec3<S> where S: Copy {
    fn from(value: &[S]) -> Self {
        assert!(value.len() == 3);
        Self { x: value[0], y: value[1], z: value[2] }
    }
}

impl<S> Add for Vec3<S> where S: Add<Output = S> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl<S> Sub for Vec3<S> where S: Sub<Output = S> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl<S> Mul<S> for Vec3<S> where
    S: Mul<Output = S> +
            Copy {
    type Output = Self;

    fn mul(self, rhs: S) -> Self::Output {
        Self::Output { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Mul<Vec3<f32>> for f32 {
    type Output = Vec3<f32>;

    fn mul(self, rhs: Vec3<f32>) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vec3<i32>> for i32 {
    type Output = Vec3<i32>;

    fn mul(self, rhs: Vec3<i32>) -> Self::Output {
        rhs * self
    }
}

#[derive(Debug)]
pub struct BndBox2<S> {
    pub min: Point2<S>,
    pub max: Point2<S>,
    empty: bool,
}

pub type BndBox2f = BndBox2<f32>;
pub type BndBox2i = BndBox2<i32>;

impl<S> BndBox2<S> where
    S: Copy +
       Default +
       PartialOrd +
       Add<Output = S> +
       Mul<Output = S> +
       Div<Output = S> +
       From<u8> {
    pub fn new_empty() -> Self {
        Self { min: Point2::origin(), max: Point2::origin(), empty: true }
    }

    pub fn add_point(&mut self, pnt: Point2<S>) {
        if self.empty {
            self.empty = false;
            self.min = pnt;
            self.max = pnt;
        }

        if pnt.x < self.min.x { self.min.x = pnt.x; }
        else if pnt.x > self.max.x { self.max.x = pnt.x; }

        if pnt.y < self.min.y { self.min.y = pnt.y; }
        else if pnt.y > self.max.y { self.max.y = pnt.y; }
    }

    pub fn clamp_by(&mut self, clamp: &Self) {
        if self.min.x < clamp.min.x {
            self.min.x = clamp.min.x;
        }
        if self.max.x > clamp.max.x {
            self.max.x = clamp.max.x;
        }
        if self.min.y < clamp.min.y {
            self.min.y = clamp.min.y;
        }
        if self.max.y > clamp.max.y {
            self.max.y = clamp.max.y;
        }
    }

    pub fn clamp(&self, pnt: Point2<S>) -> Point2<S> where S: PartialOrd {
        let mut x = if pnt.x > self.min.x { pnt.x } else { self.min.x };
        if pnt.x > self.max.x { x = self.max.x };
        let mut y = if pnt.y > self.min.y { pnt.y } else { self.min.y };
        if pnt.y > self.max.y { y = self.max.y };
        Point2 { x, y }
    }

    pub fn center(&self) -> Point2<S> {
        let two = <S as From<_>>::from(2);
        Point2 { x: (self.min.x + self.max.x) / two, y: (self.min.y + self.max.y) / two }
    }
}

pub struct BndBox3<S> {
    pub min: Point3<S>,
    pub max: Point3<S>,
    empty: bool,
}

pub type BndBox3f = BndBox3<f32>;
pub type BndBox3i = BndBox3<i32>;

impl<S> BndBox3<S> where
    S: Copy +
       Default +
       PartialOrd +
       Add<Output = S> +
       Mul<Output = S> +
       Div<Output = S> +
       From<u8> {
    pub fn new_empty() -> Self {
        Self { min: Point3::origin(), max: Point3::origin(), empty: true }
    }

    pub fn add_point(&mut self, pnt: Point3<S>) {
        if self.empty {
            self.empty = false;
            self.min = pnt;
            self.max = pnt;
        }

        if pnt.x < self.min.x { self.min.x = pnt.x; }
        else if pnt.x > self.max.x { self.max.x = pnt.x; }

        if pnt.y < self.min.y { self.min.y = pnt.y; }
        else if pnt.y > self.max.y { self.max.y = pnt.y; }

        if pnt.z < self.min.z { self.min.z = pnt.z; }
        else if pnt.z > self.max.z { self.max.z = pnt.z; }
    }

    pub fn center(&self) -> Point3<S> {
        let three = <S as From<_>>::from(3);
        Point3 { x: (self.min.x + self.max.x) / three,
                 y: (self.min.y + self.max.y) / three,
                 z: (self.min.z + self.max.z) / three }
    }
}


use std::ops::{Add, Div, Mul, Sub};

const EPSILON: f32 = 1e-7;

// TODO: consider carefully which operations make sense for integer-component
// points and vecs

#[derive(Clone, Copy, Debug)]
pub struct Point2<Scalar> {
    pub x: Scalar,
    pub y: Scalar,
}

impl<Scalar> Point2<Scalar> where Scalar: Default {
    pub fn origin() -> Self {
        Self { x: Scalar::default(), y: Scalar::default() }
    }
}

impl<> From<Point2<f32>> for Point2<i32> {
    fn from(value: Point2<f32>) -> Self {
        Self { x: value.x as i32, y: value.y as i32 }
    }
}

impl<Scalar> Sub for Point2<Scalar> where Scalar: Sub<Output = Scalar> {
    type Output = Vec2<Scalar>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

pub type Point2f = Point2<f32>;
pub type Point2i = Point2<i32>;

pub struct Vec2<Scalar> {
    pub x: Scalar,
    pub y: Scalar,
}

impl<Scalar> Vec2<Scalar> {
}

#[derive(Clone, Copy, Debug)]
pub struct Point3<Scalar> {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

impl<S> Point3<S> where
    S: Default + Copy {
    pub fn origin() -> Self {
        Point3 { x: S::default(), y: S::default(), z: S::default() }
    }

    pub fn from_slice(coord: &[S]) -> Self {
        assert!(coord.len() == 3);
        Point3 { x: coord[0], y: coord[1], z: coord[2] }
    }

    pub fn to_vec3(&self) -> Vec3<S> {
        Vec3 { x: self.x as S, y: self.y as S, z: self.z as S }
    }
}

impl<Scalar> From<Vec3<Scalar>> for Point3<Scalar> {
    fn from(v: Vec3<Scalar>) -> Self {
        Point3 { x: v.x, y: v.y, z: v.z }
    }
}

pub type Point3f = Point3<f32>;
pub type Point3i = Point3<i32>;

#[derive(Clone, Copy, Debug)]
pub struct Vec3<Scalar> {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

pub type Vec3f = Vec3<f32>;
pub type Vec3i = Vec3<i32>;

impl<Scalar> Vec3<Scalar> where
    Scalar: Mul<Output = Scalar> +
            Add<Output = Scalar> +
            Sub<Output = Scalar> +
            Copy {
    pub fn dot(&self, other: Self) -> Scalar {
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

impl From<Point3<f32>> for Vec3<f32> {
    fn from(p: Point3<f32>) -> Self {
        Self { x: p.x, y: p.y, z: p.z }
    }
}

impl<Scalar> Add for Vec3<Scalar> where Scalar: Add<Output = Scalar> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl<Scalar> Sub for Vec3<Scalar> where Scalar: Sub<Output = Scalar> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl<Scalar> Mul<Scalar> for Vec3<Scalar> where
    Scalar: Mul<Output = Scalar> +
            Copy {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
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

pub struct BndBox2<Scalar> {
    pub min: Point2<Scalar>,
    pub max: Point2<Scalar>,
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

    pub fn center(&self) -> Point2<S> {
        let two = <S as From<_>>::from(2);
        Point2 { x: (self.min.x + self.max.x) / two, y: (self.min.y + self.max.y) / two }
    }
}


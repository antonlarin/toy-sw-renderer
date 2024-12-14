use super::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct HVec4<S> {
    pub x: S,
    pub y: S,
    pub z: S,
    pub w: S,
}

impl<> HVec4<f32> {
    fn is_vector(self: &Self) -> bool { self.w.abs() < f32::EPSILON }
    fn is_point(self: &Self) -> bool { self.w.abs() > f32::EPSILON }
}

impl<> HVec4<f64> {
    fn is_vector(self: &Self) -> bool { self.w.abs() < f64::EPSILON }
    fn is_point(self: &Self) -> bool { self.w.abs() > f64::EPSILON }
}

impl<S> From<Vec3<S>> for HVec4<S> where S: From<i16> {
    fn from(value: Vec3<S>) -> Self {
        Self { x: value.x, y: value.y, z: value.z, w: 0.into() }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HMat4<S> {
    arr: [S; 16],
}

impl<S> HMat4<S> where S: From<i16> {
    pub fn eye() -> Self{
        Self {
            arr: [ 1.into(), 0.into(), 0.into(), 0.into(),
                   0.into(), 1.into(), 0.into(), 0.into(),
                   0.into(), 0.into(), 1.into(), 0.into(),
                   0.into(), 0.into(), 0.into(), 1.into() ]
        }
    }
}


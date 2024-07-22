pub struct HVec4<S> {
    pub x: S,
    pub y: S,
    pub z: S,
    pub w: S,
}

impl<S> HVec4<S> {
}

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


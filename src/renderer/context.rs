use crate::renderer::Camera;
use crate::math::Vec3f;

pub struct Context {
    pub camera: Camera,
    pub light: Vec3f,
}

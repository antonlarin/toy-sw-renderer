use crate::math::{Point2, Point3, Vec3};
#[derive(Debug)]
pub struct Camera {
    dir: Vec3,
    up: Vec3,
    ref_dir: Vec3,
}

impl Camera {
    pub fn new(dir: Vec3, up: Vec3) -> Camera {
        let dir = dir.normalize();
        if let Some(up) = up.orthogonalize(dir) {
            let up = up.normalize();
            let ref_dir = dir.cross(up);
            Camera { dir, up, ref_dir }
        } else {
            panic!("Wrong params for camera given");
        }
    }

    /// Project a point in 3D space onto the camera plane
    pub fn project_point(&self, pnt: &Point3) -> Point2 {
        let rad_vec: Vec3 = (*pnt).into();
        let dp = rad_vec.dot(self.dir);
        let vec_in_plane = rad_vec - dp * self.dir;
        let y = vec_in_plane.dot(self.up);
        let x = vec_in_plane.dot(self.ref_dir);
        Point2 { x, y }
    }
}

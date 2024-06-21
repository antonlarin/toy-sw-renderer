use crate::math::{Point2f, Point3f, Vec3f};
#[derive(Debug)]
pub struct Camera {
    loc: Point3f,
    dir: Vec3f,
    up: Vec3f,
    ref_dir: Vec3f,
    zoom: f32,
}

impl Camera {
    pub fn new(loc: Point3f, dir: Vec3f, up: Vec3f, zoom: f32) -> Camera {
        let dir = dir.normalize();
        if let Some(up) = up.orthogonalize(dir) {
            let up = up.normalize();
            let ref_dir = dir.cross(up);
            Camera { loc, dir, up, ref_dir, zoom }
        } else {
            panic!("Wrong params for camera given");
        }
    }

    /// Transform a point in 3D space into the camera coordinate system.
    pub fn transform(&self, pnt: &Point3f) -> Point3f {
        let rad_vec: Vec3f = Vec3f::from(*pnt) - self.loc.into();
        let z = rad_vec.dot(self.dir);
        let vec_in_plane = rad_vec - z * self.dir;
        let y = vec_in_plane.dot(self.up) * self.zoom;
        let x = vec_in_plane.dot(self.ref_dir) * self.zoom;
        Point3f { x, y, z }
    }

    /// Project a point in 3D space onto the camera plane.
    pub fn project(&self, pnt: &Point3f) -> Point2f {
        let trf_pnt = self.transform(pnt);
        Point2f { x: trf_pnt.x, y: trf_pnt.y }
    }
}

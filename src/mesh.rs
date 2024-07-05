use crate::math::{Point2f, Point3f, Vec3f};

#[derive(Debug)]
pub struct Triangle {
    pub vertices: [u32; 3],
    pub texcoords: Option<[u32; 3]>,
    pub normals: Option<[u32; 3]>,
}

#[derive(Debug)]
pub struct IndexedTriangleMesh {
    pub vertices: Vec<Point3f>,
    pub triangles: Vec<Triangle>,
    pub texcoords: Option<Vec<Point2f>>,
    pub normals: Option<Vec<Vec3f>>,
}


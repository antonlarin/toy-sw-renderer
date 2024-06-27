use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::math::{Point2f, Point3f, Vec3f};

#[derive(Debug)]
pub struct Triangle {
    pub vertices: [u32; 3],
    pub texcoords: Option<[u32; 3]>,
    pub normals: Option<[u32; 3]>,
}

#[derive(Debug)]
pub struct ObjModel {
    pub vertices: Vec<Point3f>,
    pub triangles: Vec<Triangle>,
    pub texcoords: Option<Vec<Point2f>>,
    pub normals: Option<Vec<Vec3f>>,
}

impl ObjModel {
    fn read_coords<const DIM: usize>(split: std::str::Split<char>) -> Option<[f32; DIM]> {
        let mut i = 0;
        let mut coord = [0.0f32; DIM];
        for line_elem in split {
            match line_elem.parse::<f32>() {
                Ok(value) if i < DIM => { coord[i] = value; },
                _ => { break; }
            }
            i += 1;
        }

        if i == DIM {
            Some(coord)
        } else {
            None
        }
    }

    pub fn from_file(filename: &str) -> Option<Self> {
        let file = File::open(filename);
        if let Err(_) = file {
            return None
        }

        let mut res = ObjModel {
            vertices: vec![],
            triangles: vec![],
            texcoords: None,
            normals: None,
        };
        let buffered_file = BufReader::new(file.unwrap());
        for l in buffered_file.lines() {
            if let Err(_) = l {
                return Some(res)
            }

            let line = l.unwrap();
            let mut split = line.split(' ');
            match split.next() {
                Some("v") => {
                    if let Some(coords) = Self::read_coords::<3>(split) {
                        let pnt = Point3f::from(&coords[..]);
                        res.vertices.push(pnt);
                    }
                },
                Some("f") => {
                    // parse the rest as face
                    let mut i = 0;
                    let mut indices = [0u32; 9];
                    for line_elem in split {
                        for (j, s) in line_elem.split('/').enumerate() {
                            match s.parse::<u32>() {
                                Ok(idx) if i < 3 && j < 3 => {
                                    indices[i * 3 + j] = idx;
                                },
                                _ => {
                                    continue
                                }
                            }
                        }
                        i += 1;
                    }

                    if i < 3 {
                        // not enough vertex indexes for triangle
                        continue;
                    } else {
                        let vert_indices: &[u32; 3] = indices[..3].try_into().unwrap();
                        let texcoord_indices: &[u32; 3] = indices[3..6].try_into().unwrap();
                        let vnormal_indices: &[u32; 3] = indices[6..].try_into().unwrap();
                        res.triangles.push(Triangle {
                            vertices: *vert_indices,
                            texcoords: if texcoord_indices[0] == 0 {
                                None
                            } else {
                                Some(*texcoord_indices)
                            },
                            normals: if vnormal_indices[0] == 0 {
                                None
                            } else {
                                Some(*vnormal_indices)
                            },
                        });
                    }
                },
                Some("vt") => {
                    if let Some(coords) = Self::read_coords::<2>(split) {
                        let pnt = Point2f::from_slice(&coords);
                        if let Some(texcoords) = res.texcoords.as_mut() {
                            texcoords.push(pnt);
                        } else {
                            res.texcoords = Some(vec![pnt]);
                        }
                    }
                },
                Some("vn") => {
                    if let Some(coords) = Self::read_coords::<3>(split) {
                        let normal = Vec3f::from(&coords[..]);
                        if let Some(normals) = res.normals.as_mut() {
                            normals.push(normal);
                        } else {
                            res.normals = Some(vec![normal]);
                        }
                    }
                },
                Some(_) => { continue },
                None => { continue }
            }
        }

        Some(res)
    }

}

#[cfg(test)]
mod tests {
    use super::ObjModel;

    #[test]
    fn import_cube() {
        let model = ObjModel::from_file("assets/cube.obj");
        assert!(model.is_some());

        let model = model.unwrap();
        assert_eq!(model.vertices.len(), 8);
        assert_eq!(model.triangles.len(), 12);
    }

    #[test]
    fn import_cube_with_texcoords() {
        let model = ObjModel::from_file("assets/cube_1.obj");
        assert!(model.is_some());

        let model = model.unwrap();
        assert!(model.texcoords.is_some());
        assert_eq!(model.texcoords.unwrap().len(), 4);
        assert!(model.triangles[0].texcoords.is_some());

        assert!(model.normals.is_none());
        assert!(model.triangles[0].normals.is_none());
    }

    #[test]
    fn import_cube_with_normals() {
        let model = ObjModel::from_file("assets/cube_2.obj");
        assert!(model.is_some());

        let model = model.unwrap();
        assert!(model.normals.is_some());
        assert_eq!(model.normals.unwrap().len(), 8);
        assert!(model.triangles[0].normals.is_some());

        assert!(model.texcoords.is_none());
        assert!(model.triangles[0].texcoords.is_none());
    }

    #[test]
    fn import_african_head() {
        let model = ObjModel::from_file("assets/african_head.obj");
        assert!(model.is_some());

        let model = model.unwrap();
        assert_eq!(model.vertices.len(), 1258);
        assert_eq!(model.triangles.len(), 2492);
    }
}

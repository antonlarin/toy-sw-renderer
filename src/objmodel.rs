use std::fs::File;
use std::io::{BufReader, BufRead};

struct Point3 {
    coord: [f32; 3],
}

struct Triangle {
    vertices: [u32; 3]
}

pub struct ObjModel {
    vertices: Vec<Point3>,
    triangles: Vec<Triangle>,
}

impl ObjModel {
    pub fn from_file(filename: &str) -> Option<Self> {
        let file = File::open(filename);
        if let Err(_) = file {
            return None
        }

        let mut res = ObjModel {
            vertices: vec![],
            triangles: vec![],
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
                    // parse the rest as vertex
                    let mut i = 0;
                    let mut coord = [0.0f32; 3];
                    for line_elem in split {
                        match line_elem.parse::<f32>() {
                            Ok(value) if i < 3 => {
                                coord[i] = value;
                                i += 1;
                            },
                            _ => {
                                // parsing error or too many coords for vertex
                                continue;
                            }
                        }

                        if i < 3 {
                            // not enough coords for vertex
                            continue;
                        } else {
                            res.vertices.push(Point3 { coord });
                        }
                    }
                },
                Some("f") => {
                    // parse the rest as face
                },
                Some("vt") => {
                    // TODO
                    // parse the rest as texture coord
                },
                Some("vn") => {
                    // TODO
                    // parse the rest as vertex normal
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
        assert_eq!(model.triangles.len(), 0);
    }
}

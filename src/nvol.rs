use crate::threed::*;
use std::fmt;
type Handle = usize;
pub struct Geometry {
    vertices: Vec<Vertex>,
    pub tris_cache: Vec<Triangle3D>,
}
#[derive(Clone)]
pub struct Vertex {
    coordinate: Point3D,
    links: Vec<Handle>,
}
impl Geometry {
    fn add_point(&mut self, x: f32, y: f32, z: f32) {
        self.vertices.push(Vertex {
            coordinate: Point3D { x, y, z },
            links: Vec::new(),
        })
    }
    fn link(&mut self, vertex: Handle, to: Handle) {
        self.vertices
            .get_mut(vertex)
            .expect("failed to find vertex at given index: fn link")
            .links
            .push(to);
    }
    fn get_links(&self, at: Handle) -> Vec<Handle> {
        self.vertices
            .get(at)
            .expect("failed to find vertex at given index: fn get_links")
            .clone()
            .links
    }
    fn get_pt_at(&self, at: Handle) -> Point3D {
        self.vertices
            .get(at)
            .expect("failed to find vertex at given index: fn get_pt_at")
            .coordinate
    }
    fn add_if_no_match(&mut self, tri: Triangle3D) -> bool {
        for t in &self.tris_cache {
            if t.matches(&tri) {
                return true;
            }
        }
        self.tris_cache.push(tri);
        false
    }
    pub fn cube(sidelen: f32, c: Point3D) -> Geometry {
        let mut vol = Geometry {
            vertices: Vec::new(),
            tris_cache: Vec::new(),
        };
        let r = sidelen / 2.0;
        vol.add_point(c.x + r, c.y + r, c.z + r); // 0
        vol.add_point(c.x + r, c.y + r, c.z - r); // 1
        vol.add_point(c.x + r, c.y - r, c.z + r); // 2
        vol.add_point(c.x + r, c.y - r, c.z - r); // 3
        vol.add_point(c.x - r, c.y + r, c.z + r); // 4
        vol.add_point(c.x - r, c.y + r, c.z - r); // 5
        vol.add_point(c.x - r, c.y - r, c.z + r); // 6
        vol.add_point(c.x - r, c.y - r, c.z - r); // 7
        for index in 0..8 {
            for link_index in 0..8 {
                if link_index != index && link_index != 7 - index {
                    vol.link(index, link_index);
                }
            }
        }
        // for v in &vol.vertices {
        //     println!("{}", v);
        // }
        vol
    }
    pub fn make_tris(&mut self) {
        self.tris_cache = Vec::new();
        for v_handle in 0..self.vertices.len() {
            for link in self.get_links(v_handle) {
                for link_link in self.get_links(link) {
                    let tri = Triangle3D {
                        a: self.get_pt_at(v_handle),
                        b: self.get_pt_at(link),
                        c: self.get_pt_at(link_link),
                    };
                    if link_link != v_handle {
                        if self.add_if_no_match(tri) {}
                    }
                }
            }
        }
    }
}

impl Vertex {
    pub fn new() -> Vertex {
        Vertex {
            coordinate: Point3D::new(),
            links: Vec::new(),
        }
    }
}
impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Vertex {}, links: {}",
            self.coordinate,
            format!("{:?}", self.links)
        )
    }
}

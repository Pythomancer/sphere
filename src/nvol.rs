use crate::{matrix::*, threed::*};
pub struct Geometry {
    vertices: Vec<Vector4>,
    pub tris: Vec<Triangle>,
    center: Point3D,
}
#[derive(Clone)]
pub struct Triangle {
    links: [usize; 3],
}
impl Triangle {
    pub fn flip(&self) -> Triangle {
        Triangle {
            links: [self.links[0], self.links[2], self.links[1]],
        }
    }
}
impl Geometry {
    pub fn new(center: Point3D) -> Geometry {
        Geometry {
            vertices: Vec::new(),
            tris: Vec::new(),
            center,
        }
    }
    pub fn add_point(&mut self, pt: Vector4) {
        self.vertices.push(pt);
    }
    pub fn add_tri(&mut self, a: usize, b: usize, c: usize) {
        self.tris.push(Triangle { links: [a, b, c] });
    }
    pub fn cube(center: Point3D) -> Geometry {
        let mut vol = Geometry::new(center);
        vol.add_point(Vector4::new(1.0, 1.0, 1.0, 1.0));
        vol.add_point(Vector4::new(1.0, 1.0, -1.0, 1.0));
        vol.add_point(Vector4::new(1.0, -1.0, 1.0, 1.0));
        vol.add_point(Vector4::new(1.0, -1.0, -1.0, 1.0));
        vol.add_point(Vector4::new(1.0, 1.0, 1.0, 1.0));
        vol.add_point(Vector4::new(1.0, 1.0, -1.0, 1.0));
        vol.add_point(Vector4::new(1.0, -1.0, 1.0, 1.0));
        vol.add_point(Vector4::new(1.0, -1.0, -1.0, 1.0));
        vol.add_tri(0, 2, 1); // 0
        vol.add_tri(3, 1, 2);
        vol.add_tri(0, 4, 2); // 1
        vol.add_tri(6, 2, 4);
        vol.add_tri(0, 1, 4); // 2
        vol.add_tri(5, 4, 1);
        vol.add_tri(4, 5, 6); // 3
        vol.add_tri(7, 6, 5);
        vol.add_tri(1, 3, 5); // 4
        vol.add_tri(7, 5, 3);
        vol.add_tri(2, 6, 4); // 5
        vol.add_tri(7, 4, 6);
        vol
    }
    pub fn get_link_pt(&self, idx: usize) -> Point3D {
        self.center + self.vertices[idx]
    }
    pub fn tris_to_global(&self) -> Vec<Triangle3D> {
        let mut ts: Vec<Triangle3D> = Vec::new();
        for tri in &self.tris {
            ts.push(Triangle3D {
                a: self.get_link_pt(tri.links[0]),
                b: self.get_link_pt(tri.links[1]),
                c: self.get_link_pt(tri.links[2]),
            })
        }
        ts
    }
    pub fn apply_transformation(&mut self, transformation: &Matrix4) {
        for i in &mut self.vertices {
            i.transform(&transformation);
        }
    }
}

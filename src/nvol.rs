use crate::geometry::*;

pub struct Nexus<'a> {
    pub point: Point,
    links: Vec<&Nexus>,
}
// pub struct Geometry {
//     points: Vec<Nexus>,
//     tris_cache: Option<Vec<Triangle>>,
// }
impl Nexus {
    pub fn new() -> Nexus {
        Nexus {
            point: Point::new(),
            links: Vec::<&Nexus>::new(),
        }
    }
    // pub fn is(&self, pt: &Point) -> bool {
    //     self.point == pt
    // }

    // pub fn translate (&mut self, offset: &Point) {
    //     point.translate(offset);
    // }

    // pub fn link (&mut self, &other: Nexus){
    //     self.links.push(other);
    // }

    // pub fn is_in (&self, list: Vec<Nexus>) -> bool {
    //     for nexus in list {
    //         if self.point == nexus.point {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
}
// impl Geometry {
//     pub fn new () -> Geometry {
//         Geometry {
//             points: Vec<Nexus>::new(),
//             tris_cache: None
//         }
//     }
//     pub fn translate (&mut self, offset: &Point) {
//         for nexus in self.points {
//             nexus.translate(offset);
//         }
//     }
//     pub fn cube (center: &Point, rad: f32) -> Geometry {
//         let new_vol = Geometry::new();
//         let pts = Vec<Point>::new();
//         pts.push(Point{x: rad, y: rad, z: rad});                     // 0
//         pts.push(Point{x: rad, y: rad, z: -1.0 * rad});                // 1
//         pts.push(Point{x: rad, y: -1.0 * rad, z: rad});                // 2
//         pts.push(Point{x: rad, y: -1.0 * rad, z: -1.0 * rad});           // 3
//         pts.push(Point{x: -1.0 * rad, y: rad, z: rad});                // 4
//         pts.push(Point{x: -1.0 * rad, y: rad, z: -1.0 * rad});           // 5
//         pts.push(Point{x: -1.0 * rad, y: -1.0 * rad, z: rad});           // 6
//         pts.push(Point{x: -1.0 * rad, y: -1.0 * rad, z: -1.0 * rad});      // 7

//         new_vol.points.push(Nexus{points: pts.0, links: Vec<&Nexus>::new()});
//         new_vol.points.push(Nexus{points: pts.1, links: Vec<&Nexus>::new()});
//         new_vol.points.push(Nexus{points: pts.2, links: Vec<&Nexus>::new()});
//         new_vol.points.push(Nexus{points: pts.3, links: Vec<&Nexus>::new()});
//         new_vol.points.push(Nexus{points: pts.4, links: Vec<&Nexus>::new()});
//         new_vol.points.push(Nexus{points: pts.5, links: Vec<&Nexus>::new()});
//         new_vol.points.push(Nexus{points: pts.6, links: Vec<&Nexus>::new()});
//         new_vol.points.push(Nexus{points: pts.7, links: Vec<&Nexus>::new()});

//         for index in 0..8 {
//             for link_index in 0..8 {
//                 if link_index != index && link_index != 7-index {
//                     new_vol.points.get(index).link(&new_vol.points.get(link_index));
//                 }
//             }
//         }
//     }

//     fn make_tris (&mut self) {
//         self.tris_cache = Some(Vec<&Nexus>::new());
//         for point in self.points {
//             for link in point.links {
//                 for link_link in link.links {
//                     if link_link.is_in(point.links) {
//                         let t = Triangle{a: point.point, b: link.point, c: link.link};
//                         if !t.is_in(tris_cache) {
//                             tris_cache.push();
//                         }
//                     }
//                 }
//             }
//         }
//     }

// }

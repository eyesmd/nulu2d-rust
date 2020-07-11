
use crate::Point;
use crate::Segment;

pub struct Polygon {
	pub vertices : Vec<Point>
}


impl Polygon {

	// Constructors
	pub fn new(v: &Vec<Point>) -> Polygon {
        assert!(v.len() >= 3,
            "A polygon must be initialized with 3 or more vertices ({} used)", v.len());
        Polygon{ vertices: v.clone() }
    }

    // Accessors
    // TODO: consider caching some of these
    pub fn segments(&self) -> Vec<Segment> {
        let len = self.vertices.len();
        let mut res = Vec::with_capacity(len);
        for i in 0..len-1 {
            res.push(Segment::from_points(self.vertices[i], self.vertices[i+1]));
        }
        res.push(Segment::from_points(self.vertices[len-1], self.vertices[0]));
        return res;
    }
    pub fn width(&self) -> f64 {
        let max_x = self.vertices.iter().fold(std::f64::NEG_INFINITY, |m, p| f64::max(m, p.x));
        let min_x = self.vertices.iter().fold(std::f64::INFINITY, |m, p| f64::min(m, p.x));
        return max_x - min_x;
    }
    pub fn height(&self) -> f64 {
        let max_y = self.vertices.iter().fold(std::f64::NEG_INFINITY, |m, p| f64::max(m, p.y));
        let min_y = self.vertices.iter().fold(std::f64::INFINITY, |m, p| f64::min(m, p.y));
        return max_y - min_y;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let vertices = vec![
            Point::new(-1.0, 0.0),
            Point::new(1.0, 2.0),
            Point::new(1.0, 0.0)];
        let p = Polygon::new(&vertices);
        assert_eq!(p.vertices, vertices);
    }

    // una poronga esto
    #[test]
    #[should_panic(expected = "A polygon must be initialized with 3 or more vertices (2 used)")]
    fn new_too_few_vertices() {
        let vertices = vec![
            Point::new(-1.0, 0.0),
            Point::new(1.0, 2.0)];
        Polygon::new(&vertices);
    }

    #[test]
    fn segments() {
        let vertices = vec![
            Point::new(-1.0, 0.0),
            Point::new(1.0, 2.0),
            Point::new(1.0, 0.0)];
        let p = Polygon::new(&vertices);
        let expected = vec![
            Segment::from_points(vertices[0], vertices[1]),
            Segment::from_points(vertices[1], vertices[2]),
            Segment::from_points(vertices[2], vertices[0]),
        ];
        assert_eq!(p.segments(), expected);
    }

    #[test]
    fn size() {
        let p = Polygon::new(&vec![
            Point::new(-1.0, -1.0),
            Point::new(1.0, 1.3),
            Point::new(1.0, -1.0),
        ]);
        assert_eq!(p.width(), 2.0);
        assert_eq!(p.height(), 2.3);
    }
}

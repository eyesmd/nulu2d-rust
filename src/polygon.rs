
use crate::Point;
use crate::Segment;
use similar::Similar;

#[derive(Debug, PartialEq)]
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
        let max_x = self.vertices.iter().fold(f64::NEG_INFINITY, |m, p| f64::max(m, p.x));
        let min_x = self.vertices.iter().fold(f64::INFINITY, |m, p| f64::min(m, p.x));
        return max_x - min_x;
    }

    pub fn height(&self) -> f64 {
        let max_y = self.vertices.iter().fold(f64::NEG_INFINITY, |m, p| f64::max(m, p.y));
        let min_y = self.vertices.iter().fold(f64::INFINITY, |m, p| f64::min(m, p.y));
        return max_y - min_y;
    }

    pub fn center(&self) -> Point {
        return self.vertices.iter().fold(Point::zero(), |a, b| a + *b ) / self.vertices.len() as f64;
    }

    pub fn top(&self) -> f64 {
        return self.vertices.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
    }

    pub fn bottom(&self) -> f64 {
        return self.vertices.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
    }

    pub fn left(&self) -> f64 {
        return self.vertices.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
    }

    pub fn right(&self) -> f64 {
        return self.vertices.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
    }

    pub fn centroid(&self) -> Point {
        let mut signed_area = 0.0;
        let mut centroid = Point::zero();

        for i in 0..self.vertices.len()-1 {
            centroid += (self.vertices[i] + self.vertices[i+1]) * (self.vertices[i] ^ self.vertices[i+1]);
            signed_area += self.vertices[i] ^ self.vertices[i+1];
        }
        signed_area += *self.vertices.last().unwrap() ^ *self.vertices.first().unwrap();
        centroid += (*self.vertices.last().unwrap() + *self.vertices.first().unwrap()) * (*self.vertices.last().unwrap() ^ *self.vertices.first().unwrap());
        signed_area /= 2.0;
        centroid /= 6.0 * signed_area;
        return centroid;
    }

}

impl Similar for &Polygon {
    /* NOTE: the vertices order matter for this:
        A-B-C == A-B-C
        A-B-C == B-C-A
        A-C-B != A-B-C
    */
    /* TODO: We might want to have a more restricted invariant. Some ideas:
        - represent convex polygons only (I think we are already assuming this somewhere)
        - always have self.vertices sorted clockwise (take center/centroid as reference)
        - have one constructor that sanitizes the input (i.e. computes the convex hull
          and sorts the vertices) and another one with the adequate precondition for performance.
    */
    fn is_similar(self, other : &Polygon, eps : f64) -> bool {
        if self.vertices.len() != other.vertices.len() {
            return false;
        }

        let offset = other.vertices.iter().position(|v| v.is_similar(self.vertices[0], eps));
        match offset {
            None => return false,
            Some(offset) => {
                return
                    other.vertices[offset..].is_similar(&self.vertices[..self.vertices.len()-offset], eps)
                    && other.vertices[..offset].is_similar(&self.vertices[self.vertices.len()-offset..], eps);
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use similar::assert_similar;

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

    #[test]
    fn positions() {
        let p = Polygon::new(&vec![
            Point::new(0.0, -2.0),
            Point::new(2.0, -2.0),
            Point::new(2.0, 1.0),
            Point::new(0.0, 1.0),
        ]);
        assert_eq!(p.center(), Point::new(1.0, -0.5));
        assert_eq!(p.left(), 0.0);
        assert_eq!(p.right(), 2.0);
        assert_eq!(p.bottom(), -2.0);
        assert_eq!(p.top(), 1.0);
    }

    #[test]
    fn centroid_simple() {
        let p = Polygon::new(&vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(0.0, 1.0),
        ]);
        assert_eq!(p.centroid(), Point::new(1.0/3.0, 1.0/3.0));
    }

    #[test]
    fn centroid_complex() {
        let p = Polygon::new(&vec![
            Point::new(45.3142533036254, -93.47527313511819),
            Point::new(45.31232182518015, -93.34893036168069),
            Point::new(45.23694281999268, -93.35167694371194),
            Point::new(45.23500870841669, -93.47801971714944),
            Point::new(45.3142533036254, -93.47527313511819),
        ]);
        assert_eq!(p.centroid(), Point::new(45.27463866133501, -93.41400121829719));
    }

    #[test]
    fn is_similar() {
        let p1 = Polygon::new(&vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(1.0, 1.0),
            Point::new(1.0, 0.0),
        ]);
        let p2 = Polygon::new(&vec![
            Point::new(1.0, 0.99999999968),
            Point::new(1.0000002, 0.0),
            Point::new(0.0, 0.000000023),
            Point::new(-0.0000002, 1.0),
        ]);
        assert_similar!(p1, &p2, 1e-5);
    }
}

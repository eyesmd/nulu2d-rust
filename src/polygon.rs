
use crate::Point;

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

}

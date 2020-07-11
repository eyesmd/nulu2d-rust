
use crate::Point;
use similar::Similar;
use similar_derive::Similar;

#[derive(Copy, Clone, Debug, PartialEq, Similar)]
pub struct Segment {
	pub a : Point,
    pub b : Point
}


impl Segment {

	// Constructors
	pub fn from_points(a: Point, b: Point) -> Segment {
		Segment{ a, b }
	}

	pub fn from_arrow(center: Point, direction: Point) -> Segment {
		Segment{ a: center, b: center + direction }
	}

}

#[cfg(test)]
mod tests {
    use super::*;
    use similar::assert_similar;

    #[test]
    fn from_points() {
        let s = Segment::from_points(Point::new(0.0, 1.0), Point::new(1.0, 1.0));
        assert_similar!(s.a, Point::new(0.0, 1.0));
        assert_similar!(s.b, Point::new(1.0, 1.0));
    }

    #[test]
    fn from_arrow() {
        let s = Segment::from_arrow(Point::new(0.0, 1.0), Point::new(1.0, 1.0));
        assert_similar!(s.a, Point::new(0.0, 1.0));
        assert_similar!(s.b, Point::new(1.0, 2.0));
    }

}
